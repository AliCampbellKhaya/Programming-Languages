  use peg::*;
/* An implementation of a PEG parser for the below context-free grammar.
 * 
 * Terminal Symbols:
 * An id starts with any letter and can be optionally followed by any valid combination 
 * of letters, numbers and underscores.
 * 
 * A numeral is an optional minus sign, then a required integer part, 
 * then an optional period and fractional part.
 * An integer part is either a single 0 or a nonzero digit.
 * A fractional part is a nonempty sequence of digits.
 * 
 * Variable Symbols:
 * Atom <- numeral |  id "(" ArgList ")" | id | "(" Expr ")"
 * Op2 <- Atom * Op2 | Atom
 * Op1 <- Op2 + Op1 | Op2 - Op1 | Op2
 * Expr <- "let" Decl "in" Expr | Op1
 * Decl <- "var" id "=" Expr | "function" id "(" ArgList ")" "{" Expr "}"
 *
 * NonEmptyArgList <- id, NonEmptyArgList | id
 * ArgList <-  NonEmptyArgList | <empty string>
 */

 /* Define an Expression and a Declaration */
#[derive(Clone)]
pub enum Expr {
    Id(String),
    Numeral(f64),
    Times(Box<Expr>,Box<Expr>),
    Plus(Box<Expr>,Box<Expr>),
    Minus(Box<Expr>,Box<Expr>),
    Let(Box<Decl>,Box<Expr>),
    FunCall(String, Vec<Expr>),
} 
#[derive(Clone)]
pub enum Decl {
    VarDecl(String, Box<Expr>),
    FunDecl(String, Vec<String>, Box<Expr>),
}

/* The following five functions are for debugging and testing code. */
pub fn expr_eq(e1: Expr, e2: Expr) -> bool {
  match (e1,e2) {
    (Expr::Id(s1),Expr::Id(s2)) => s1 == s2,
    (Expr::Numeral(n1), Expr::Numeral(n2)) => n1 == n2,
    (Expr::Times(l1,r1),Expr::Times(l2,r2)) => expr_eq(*l1,*l2) && expr_eq(*r1,*r2),
    (Expr::Plus(l1,r1),Expr::Plus(l2,r2)) => expr_eq(*l1,*l2) && expr_eq(*r1,*r2),
    (Expr::Minus(l1,r1),Expr::Minus(l2,r2)) => expr_eq(*l1,*l2) && expr_eq(*r1,*r2),
    (Expr::Let(d1,e1),Expr::Let(d2,e2)) => decl_eq(*d1,*d2) && expr_eq(*e1,*e2),
    (Expr::FunCall(f1, args1),Expr::FunCall(f2,args2)) =>  {
    for (x,y) in args1.iter().zip(args2.iter()) {
      if !expr_eq(x.clone(),y.clone()) {
        return false;
      }
    }

    return f1 == f2},
    _ =>false,
  }
}

pub fn decl_eq(d1: Decl, d2: Decl) -> bool {
  match (d1,d2) {
    (Decl::FunDecl(f1, args1, body1), Decl::FunDecl(f2,args2,body2)) => 
    f1 == f2 && args1 == args2 && expr_eq(*body1,*body2),
    (Decl::VarDecl(x1, body1), Decl::VarDecl(x2, body2)) => x1 == x2 && expr_eq(*body1,*body2),
    _ => false,
  }
}

pub fn expr_to_string(e: Expr) -> String {
 match e {
    Expr::Id(s) => s,
    Expr::Numeral(f) => f.to_string(),
    Expr::Times(l,r) =>format!("{}*{}", expr_to_string(*l), expr_to_string(*r)),
    Expr::Plus(l,r) =>format!("{}+{}", expr_to_string(*l), expr_to_string(*r)),
    Expr::Minus(l,r) =>format!("{}-{}", expr_to_string(*l), expr_to_string(*r)),
    Expr::Let(d,e)=>format!("let {} in {}", decl_to_string(*d), expr_to_string(*e)),
    Expr::FunCall(f,args)=>{
      let mut arg_str = "".to_string();
      for s in args {
        arg_str = format!("{},{}",arg_str, expr_to_string(s))
      }
    format!("{}({})", f,arg_str)
    }  }
}

pub fn decl_to_string(d: Decl) -> String {
 match d {
  Decl::FunDecl(f, al, b) =>{
    let mut arg_str = "".to_string();
    for s in al {
      arg_str = format!("{}{}",arg_str, s)
    }
    format!("function {}({}){{{}}}", f,arg_str,expr_to_string(*b))
  }
  ,
  Decl::VarDecl(x,e)=> format!("var {} = {}",x,expr_to_string(*e)),
 }
}

pub fn e_res_to_str(r: Result<Expr,peg::error::ParseError<peg::str::LineCol>>) -> String {
  match r {
    Ok(s) => expr_to_string(s),
    Err(_s) => "err".to_string(),
  }
}


peg::parser!{
  pub grammar parser() for str {  
  /* Parse a single identifier (id) (i.e., variable name) */ 
  pub rule id() -> String 
  = n:$(['a'..='z' | 'A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*)
    {? n.parse().or(Err("String"))}

 /* Parse a single variable. var() behaves just like id(), except with a different return type.*/ 
  pub rule var() -> Expr 
  = n:(id()) { Expr::Id(n) }
    
  /* Parse a single literal number or numeral.*/
  rule numeral_f64() -> f64
  = (n:$("-"? ((['1'..='9'] ['0'..='9']*) / "0") ("." ['0'..='9']+)?)
    {? n.parse::<f64>().or(Err("f64")) })

  pub rule numeral() -> Expr 
  = n:(numeral_f64()) { Expr::Numeral((n)) }
  
  /* Parser implementation for all expressions and declarations. Uses a precedence-climbing approach.
     Both expr() and decl() call eachother.*/

  // An atom in the precedence hierachy
  rule atom() -> Expr
  = numeral() / (i:id() "(" a:arg_list_expr() ")" {Expr::FunCall(i, a)})
  / var() / ("(" e:expr() ")" {e}) 

  // Higher level operations in precedence hierachy
  rule op2() -> Expr
  = (l:atom() "*" r:op2() {Expr::Times(Box::new(l), Box::new(r))}) / atom()

  // Lower level operations in precedence hierachy
  rule op1() -> Expr 
  = (l:op2() "+" r:op1() {Expr::Plus(Box::new(l), Box::new(r))}) 
  / (l:op2() "-" r:op1() {Expr::Minus(Box::new(l), Box::new(r))}) / op2()

  // Argument lists split into empty and non empty to allow for correct types to be prsed
  rule non_empty_arg_list() -> Vec<String>
  = (i:(id() ** ("," non_empty_arg_list())) {i})   / i:id() {vec![i]}

  rule arg_list() -> Vec<String>
  = (n:non_empty_arg_list() {n}) / ({vec![]})

  rule non_empty_arg_list_expr() -> Vec<Expr>
  = (e:(expr() ** ("," non_empty_arg_list_expr())) {e}) / e:expr() {vec![e]}

  rule arg_list_expr() -> Vec<Expr>
  = (n:non_empty_arg_list_expr() {n}) / ({vec![]})

  // Expressions
  pub rule expr() -> Expr 
  = ("let " d:decl() " in " e:expr() {Expr::Let(Box::new(d), Box::new(e))}) / op1()
  
  // Declarations
  pub rule decl() -> Decl 
  = ("var " i:id() " = " e:expr() {Decl::VarDecl(i, Box::new(e))}) 
  / ("function " i:id() "(" a:arg_list() ")" "{" e:expr() "}" {Decl::FunDecl(i, a, Box::new(e))})
  }
}

/* Materials Copyright Rose Bohrer 2023, Completed and Edited by Alasdair Campbell 2023 */