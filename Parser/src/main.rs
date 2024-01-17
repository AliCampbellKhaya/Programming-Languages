pub mod parser;
use parser::*;
use parser::Expr as ex;
use parser::Decl as dc;

// Tests the parsing of numerals
fn test_numeral(score: f64, input: String, out: Option<f64>) -> f64 {
  match (parser::parser::numeral(&input), out) {
    (Ok(f1), Some(f2)) =>
      if parser::expr_eq(f1, parser::Expr::Numeral(f2))  {
        return score
      } else {
        println!("bad");
        return 0.0
      },
    (Err(_), None) => return score,
    (Ok(f), None) => {println!("Expression test case expected not to parse but got: {}", parser::expr_to_string(f)); return 0.0},
    (Err(_), Some(f)) => {println!("Expression test case did not parse but expected {}", f); return 0.0}
  }
}

// Tests the parsing of ids
fn test_id(score: f64, input: String, out: Option<String>) -> f64 {
  match (parser::parser::id(&input), out) {
    (Ok(s1), Some(s2)) =>
      if s1 == s2  {
        return score
      } else {
        println!("Expression test case {} expected id {} but got {}", input,s2,s1);
        return 0.0
      },
    (Err(_), None) => return score,
    (Ok(f), None) => {println!("Expression test case {} expected not to parse but got {}", input, f); return 0.0},
    (Err(_), Some(f)) => {println!("Expression test case {} expected {} but did not parse", input, f); return 0.0}
  }
}

// Tests the parsing of expressions
fn test_expr(score: f64, input: String, out: Option<parser::Expr>) -> f64 {
  match (parser::parser::expr(&input), out) {
    (Ok(e1), Some(e2)) =>
      if parser::expr_eq(e1.clone(),e2.clone()) {
        return score
      } else {
        println!("Expression test case {} expected expr {} but got {}", input,expr_to_string(e2),expr_to_string(e1));
        return 0.0
      },
    (Err(_), None) => return score,
    (Ok(f), None) => {println!("Expression test case {} expected not to parse but got {}", input, expr_to_string(f)); return 0.0},
    (Err(_), Some(f)) => {println!("Expression test case {} expected {} but did not parse", input, expr_to_string(f)); return 0.0}
  }
}

// Tests the parsing of declarations
fn test_decl(score: f64, input: String, out: Option<parser::Decl>) -> f64 {
  match (parser::parser::decl(&input), out) {
    (Ok(d1), Some(d2)) =>
      if parser::decl_eq(d1.clone(),d2.clone()) {
        return score
      } else {
        println!("Declaration test case {} expected decl {} but got {}", input,decl_to_string(d2),decl_to_string(d1));
        return 0.0
      },
    (Err(_), None) => return score,
    (Ok(d), None) => {println!("Declaration test case {} expected not to parse but got {}", input, decl_to_string(d)); return 0.0},
    (Err(_), Some(d)) => {println!("Declaration test case {} expected {} but did not parse", input, decl_to_string(d)); 
    return 0.0}
  }
}

pub fn main() {
  // 10 tests of ids
  let r1 = test_id(1.0, "a".to_string(), Some("a".to_string()));
  let r2 = test_id(1.0, "A".to_string(), Some("A".to_string()));
  let r3 = test_id(1.0, "aB".to_string(), Some("aB".to_string()));
  let r4 = test_id(1.0, "Ab".to_string(), Some("Ab".to_string()));
  let r5 = test_id(1.0, "a_2b".to_string(), Some("a_2b".to_string()));
  let r6 = test_id(1.0, "c3".to_string(), Some("c3".to_string()));
  let r7 = test_id(1.0, "d7_".to_string(), Some("d7_".to_string()));
  let r8 = test_id(1.0, "0ab".to_string(), None);
  let r9 = test_id(1.0, "_8e".to_string(), None);
  let r10 = test_id(1.0, "-29".to_string(), None);
  let r_id = r1+r2+r3+r4+r5+r6+r7+r8+r9+r10;
  // 10 tests of numerals
  let r11 = test_numeral(1.0, "3".to_string(), Some(3.));
  let r12 = test_numeral(1.0, "-3".to_string(), Some(-3.));
  let r13 = test_numeral(1.0, "7.0".to_string(), Some(7.));
  let r14 = test_numeral(1.0, "-1.000".to_string(), Some(-1.));
  let r15 = test_numeral(1.0, "0.738".to_string(), Some(0.738));
  let r16 = test_numeral(1.0, "-0".to_string(), Some(0.));
  let r17 = test_numeral(1.0, "00.7".to_string(), None);
  let r18 = test_numeral(1.0, "00".to_string(), None);
  let r19 = test_numeral(1.0, "-9.".to_string(), None);
  let r20 = test_numeral(1.0, "9.".to_string(), None);
  let r_num = r11+r12+r13+r14+r15+r16+r17+r18+r19+r20;
  // 12 tests of expressions
  let r21 = test_expr(1.0, "xyzzy".to_string(), Some(ex::Id("xyzzy".to_string())));
  let r22 = test_expr(1.0, "234".to_string(), Some(ex::Numeral(234.0)));
  let r23 = test_expr(1.0, "x*y".to_string(), Some(ex::Times(Box::new(ex::Id("x".to_string())), Box::new(ex::Id("y".to_string())))));
  let r24 = test_expr(1.0, "1.2+z_3".to_string(), Some(ex::Plus(Box::new(ex::Numeral(1.2)),Box::new(ex::Id("z_3".to_string())))));
  let r25 = test_expr(1.0, "3-1".to_string(), Some(ex::Minus(Box::new(ex::Numeral(3.0)),Box::new(ex::Numeral(1.0)))));
  let r26 = test_expr(1.0, "(2-(y))".to_string(), Some(ex::Minus(Box::new(ex::Numeral(2.0)),Box::new(ex::Id("y".to_string())))));
  let r27 = test_expr(1.0, "((2-(y))".to_string(), None);
  let r28 = test_expr(1.0, "+32".to_string(), None);
  let r29 = test_expr(1.0, "1*2+3*4".to_string(), 
  Some(ex::Plus(Box::new(ex::Times(Box::new(ex::Numeral(1.0)),Box::new(ex::Numeral(2.0)))),
                    Box::new(ex::Times(Box::new(ex::Numeral(3.0)),Box::new(ex::Numeral(4.0)))))));
  let r30 = test_expr(1.0, "1*2-3*4".to_string(), 
  Some(ex::Minus(Box::new(ex::Times(Box::new(ex::Numeral(1.0)),Box::new(ex::Numeral(2.0)))),
                    Box::new(ex::Times(Box::new(ex::Numeral(3.0)),Box::new(ex::Numeral(4.0)))))));
  let r31 = test_expr(1.0, "-1*-2--3*-4".to_string(), 
  Some(ex::Minus(Box::new(ex::Times(Box::new(ex::Numeral(-1.0)),Box::new(ex::Numeral(-2.0)))),
                    Box::new(ex::Times(Box::new(ex::Numeral(-3.0)),Box::new(ex::Numeral(-4.0)))))));
  let r32 = test_expr(1.0, "--1".to_string(), None);
  let r_expr = r21+r22+r23+r24+r25+r26+r27+r28+r29+r30+r31+r32;
  // 8 tests of expressions and declarations
  let r33 = test_expr(1.0, "let var x = y in x".to_string(),
   Some(ex::Let(Box::new(dc::VarDecl("x".to_string(), Box::new(ex::Id("y".to_string())))),Box::new(ex::Id("x".to_string())))));
  let r34a = test_expr(1.0, "let function f(x){y*x} in z".to_string(),
   Some(ex::Let(Box::new(dc::FunDecl("f".to_string(), vec!["x".to_string()], 
    Box::new(ex::Times(Box::new(ex::Id("y".to_string())), Box::new(ex::Id("x".to_string())))))),
    Box::new(ex::Id("z".to_string())))));
  let r34b = test_expr(1.0, "let function f(x){y*x} in f(2)".to_string(),
    Some(ex::Let(Box::new(dc::FunDecl("f".to_string(), vec!["x".to_string()], 
     Box::new(ex::Times(Box::new(ex::Id("y".to_string())), Box::new(ex::Id("x".to_string())))))),
     Box::new(ex::FunCall("f".to_string(), vec![ex::Numeral(2.0)])))));
   let r35 = test_expr(1.0, "let var x = let var y = 1 in y in let var y = x in x".to_string(),
   Some(ex::Let(Box::new(
    dc::VarDecl("x".to_string(),  
      Box::new(ex::Let(Box::new(dc::VarDecl("y".to_string(),Box::new(ex::Numeral(1.0)))),
               Box::new(ex::Id("y".to_string())))))),
   Box::new(ex::Let(Box::new(dc::VarDecl("y".to_string(),Box::new(ex::Id("x".to_string())))),Box::new(ex::Id("x".to_string())))))));
  let r36 = test_expr(1.0, "(let var x = y in x)+(let var x = y in x)".to_string(),
   Some(ex::Plus(Box::new(ex::Let(Box::new(dc::VarDecl("x".to_string(),Box::new(ex::Id("y".to_string())))),Box::new(ex::Id("x".to_string())))),
   Box::new(ex::Let(Box::new(dc::VarDecl("x".to_string(), Box::new(ex::Id("y".to_string())))),Box::new(ex::Id("x".to_string())))))));
  let r37 = test_decl(1.0, "var x = y".to_string(),
   Some(dc::VarDecl("x".to_string(), Box::new(ex::Id("y".to_string())))));
  let r38 = test_decl(1.0, "function f(x){y*x}".to_string(),
   Some(dc::FunDecl("f".to_string(), vec!["x".to_string()], 
    Box::new(ex::Times(Box::new(ex::Id("y".to_string())), Box::new(ex::Id("x".to_string())))))));
  let r39 = test_decl(1.0, "var x = let var y = 1 in y".to_string(),
   Some(dc::VarDecl("x".to_string(),  
      Box::new(ex::Let(Box::new(dc::VarDecl("y".to_string(),Box::new(ex::Numeral(1.0)))),
               Box::new(ex::Id("y".to_string())))))));
  let r_decl = r33+r34a+r34b+r35+r36+r37+r38+r39;

  let r = r_id + r_num + r_expr + r_decl;
  println!("Results: {}/40 tests succesfully completed", r)
}