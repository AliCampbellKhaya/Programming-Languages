/* An implementation of a type-system that takes an abstract syntax tree (AST) as input
 * and determines the type of the program or if it does not type-check (i.e. is ill-typed).
 * 
 * The following two functions are implemented for this task:
 * type_check_expr (for expressions)
 * type_check_defn (for definitions)
 * */

use std::{hash::Hash, env, any::TypeId};
use rpds::HashTrieMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
/* A value of t : Type represents a given type */
pub enum Type {
    Number,  /* represents "num" type */
    String,  /* represents "string" type */
    Boolean, /* represents "boolean" type */
    Function(Vec<Type>, Box<Type>), /* represents type of function t1 -> t2 */
}

/* This enumeration type lists out the different comparison operators */
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Comparison {
    LessEqual,
    Less,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
}

/* A value e : Expr is an AST for a Toi expression  */
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Expr {
    Id(String),  /* Identifier, i.e., variable name */
    Numeral(i64), /* Number literal, e.g., 5 */
    StringLiteral(String), /* String literal, e.g., "hi" */
    True, /* Boolean literal true */
    False, /* Boolean literal true */
    /* To reduce the number of cases, we combine all the comparison operators.
     * The arguments indicate the first operand, what kind of comparison, 
     * then the second operand.
     * For example, Compare(e1,Greater,e2) is (e1 > e2) */
    Compare(Box<Expr>, Comparison, Box<Expr>),
    Times(Box<Expr>,Box<Expr>), /* Multiplication */
    Plus(Box<Expr>,Box<Expr>),  /* Addition */
    Minus(Box<Expr>,Box<Expr>), /* Subtraction */
    Let(Box<Defn>,Box<Expr>),   /* Let-definitions */
    Call(String, Vec<Expr>),    /* Function calls */
} 
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Defn {
    /* Variable definitions */
    VarDefn(String, Box<Expr>), 
    /* Function definitions. Functions can be called recursively
     * For example in pseudo-code:
     * FunDecl("f", [("x",number),("y",number)], bool, Compare(x,Equal,y))
     * is the AST for the Toi function definition:
     *   function f(x:number,y:number):bool = 
     *     (x = y) */
    FunDefn(String, Vec<(String,Type)>, Type, Box<Expr>),
}

/* Type-checking for definitions.
 * Arguments: "con" is the typing context Γ (Gamma)
 *            "d" is the AST for a definition
 *   If the judgement Γ ⊢ d : Γ' holds, then 
 *   type_check_defn(Γ, d) = Some(Γ'). If not, type_check_defn(Γ,d) = None */
pub fn type_check_defn(con: &HashTrieMap<String, Type>, d: &Defn) -> Option<(String,Type)> {
    // Match definition type to variable or function
    match d {
        Defn::VarDefn(var, val) => {
            match type_check_expr(con, val) {
                Some(n) => Some((var.to_string(), n)),
                None => None
            }
        }, 
        Defn::FunDefn(func, params, t, expr) => {
            let mut param_type = vec![];
            let mut updated_con = con.clone();
            for p in params.iter() {
                param_type.push(p.clone().1);
                updated_con = HashTrieMap::insert(&updated_con, p.clone().0, p.clone().1);
            }
            updated_con = HashTrieMap::insert(&updated_con, func.to_string(), Type::Function(param_type.clone(), Box::new(t.clone())).clone());
            match type_check_expr(&updated_con, expr) {
                Some(p) => Some((func.to_string(), Type::Function(param_type.clone(), Box::new(t.clone())))),
                None => None
            }
        },
    }
}

/* Type-checking for expressions.
 * Arguments: "con" is the typing context Γ (Gamma)
 *            "e" is the AST for an expression
 *   If the judgement Γ ⊢ e : t holds, then 
 *   type_check_expr(Γ, e) = Some(t). If not, type_check_expr(Γ,e) = None */
pub fn type_check_expr(con: &HashTrieMap<String, Type>, e: &Expr) -> Option<Type> {
    match e {
        Expr::Id(s) => {
            match HashTrieMap::get(con, s) {
                Some(val) => Some(val.clone()),
                None => None
            }
        },
        Expr::Numeral(n) => {
            Some(Type::Number)
        },
        Expr::StringLiteral(s) => {
            Some(Type::String)
        },
        Expr::True => {
            Some(Type::Boolean)
        },
        Expr::False => {
            Some(Type::Boolean)
        },
        Expr::Compare(l, comp, r) => {
            let tcl = type_check_expr(con, l);
            let tcr = type_check_expr(con, r);

            if tcl == Some(Type::Number) && tcr == Some(Type::Number) {
                Some(Type::Boolean)
            }
            else {
                None
            }
        },
        Expr::Times(l, r) => {
            let tcl = type_check_expr(con, l);
            let tcr = type_check_expr(con, r);

            if tcl == Some(Type::Number) && tcr == Some(Type::Number) {
                Some(Type::Number)
            }
            else {
                None
            }
        },
        Expr::Plus(l,r ) => {
            let tcl = type_check_expr(con, l);
            let tcr = type_check_expr(con, r);

            if tcl == Some(Type::Number) && tcr == Some(Type::Number) {
                Some(Type::Number)
            }
            else {
                None
            }
        },
        Expr::Minus(l, r) => {
            let tcl = type_check_expr(con, l);
            let tcr = type_check_expr(con, r);

            if tcl == Some(Type::Number) && tcr == Some(Type::Number) {
                Some(Type::Number)
            }
            else {
                None
            }
        },
        Expr::Let(d, v) => {
            let Some(tcd) = type_check_defn(con, d) else { return None };
            let updated_con = HashTrieMap::insert(con, tcd.0, tcd.1);
            type_check_expr(&updated_con, v)
        },
        Expr::Call(f, params) => {
            match HashTrieMap::get(con, f) {
                Some(t) => {
                    match t {
                        Type::Function(pt, t) => {
                            if pt.len() != params.len() {
                                return None
                            }
                            for (pti, paramsi) in pt.iter().zip(params.iter()) {
                                let paramsi_type = type_check_expr(con, paramsi);
                                    if Some(pti.clone()) != paramsi_type {
                                        return None
                                    }
                                }
                            Some(*t.clone())
                        },
                        _ => Some(t.clone())
                    }
                },
                None => None 
            }
        }
    }
}

/* Materials Copyright Rose Bohrer 2023, Completed and Edited by Alasdair Campbell 2023 */