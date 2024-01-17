pub mod typer;
use typer::*;
use typer::Expr as ex;
use typer::Defn as dc;
use typer::Comparison as cmp;
use typer::Type as tp;
use rpds::HashTrieMap;


fn test_expr_internal(score: f64, in_env: HashTrieMap<String,tp>, in_e: Expr, out: Option<tp>) -> f64 {
    match (asgn4::type_check_expr(&in_env, &in_e), out) {
      (e1, e2) =>
        if e1 == e2 {
          return score
        } else {
          {println!("Expression test case {:?}; {:?} expected expr {:?} but got {:?}", in_env,in_e, e2,e1); }
          return 0.0
        },      
    }
  }

  fn test_expr(score: f64, in_e: Expr, out: Option<tp>) -> f64 {
    test_expr_internal(score, HashTrieMap::new(), in_e, out)
  }

  fn test_defn_internal(score: f64, in_env: HashTrieMap<String,tp>, in_d: Defn, out: Option<(String,tp)>) -> f64 {
    match (asgn4::type_check_defn(&in_env, &in_d), out) {
      (e1, e2) =>
        if e1 == e2 {
          return score
        } else {
          {println!("Definition test case {:?}; {:?} expected output environment {:?} but got {:?}", in_env,in_d, e2,e1); }
          return 0.0
        },      
    }
  }

  fn test_defn(score: f64,  in_d: Defn, out: Option<(String,tp)>) -> f64 {
     test_defn_internal(score, HashTrieMap::new(), in_d, out)
  }

fn main() -> () {
    // 5 tests of numerals/numbers
    let r1 = test_expr(3.0, ex::Plus(Box::new(ex::Numeral(1)),Box::new(ex::Numeral(2))), Some(tp::Number));
    let r2 = test_expr(3.0, ex::Let(Box::new(dc::VarDefn("x".to_string(), Box::new(ex::Plus(Box::new(ex::Numeral(1)), Box::new(ex::Numeral(2)))))), 
    Box::new(ex::Minus(Box::new(ex::Id("x".to_string())), Box::new(ex::Numeral(3))))), Some(tp::Number));
    let r3 = test_expr(3.0, ex::Plus(Box::new(ex::Numeral(12)), Box::new(ex::True)), None);
    let r4 = test_expr(3.0, ex::Times(Box::new(ex::StringLiteral("UwU".to_string())), Box::new(ex::Numeral(12))), None);
    
    // 1 test of strings
    let r5 = test_expr(3.0, ex::StringLiteral("OwO".to_string()), Some(tp::String));

    // 10 tests of booleans
    let r6 = test_expr(3.0, ex::True, Some(tp::Boolean));
    let r7 = test_expr(3.0, ex::False, Some(tp::Boolean));
    let r8 = test_expr(3.0, ex::Compare(Box::new(ex::Numeral(0)), cmp::Equal, Box::new(ex::Numeral(0))), Some(tp::Boolean));
    let r9 = test_expr(3.0, ex::Compare(Box::new(ex::Numeral(0)), cmp::Less, Box::new(ex::Numeral(0))), Some(tp::Boolean));
    let r10 = test_expr(3.0, ex::Compare(Box::new(ex::Numeral(0)), cmp::LessEqual, Box::new(ex::Numeral(0))), Some(tp::Boolean));
    let r11 = test_expr(3.0, ex::Compare(Box::new(ex::Numeral(0)), cmp::Greater, Box::new(ex::Numeral(0))), Some(tp::Boolean));
    let r12 = test_expr(3.0, ex::Compare(Box::new(ex::Numeral(0)), cmp::GreaterEqual, Box::new(ex::Numeral(0))), Some(tp::Boolean));
    let r13 = test_expr(3.0, ex::Compare(Box::new(ex::Numeral(0)), cmp::NotEqual, Box::new(ex::Numeral(0))), Some(tp::Boolean));
    let r14 = test_expr(3.0, ex::Compare(Box::new(ex::StringLiteral("nya".to_string())), cmp::NotEqual, Box::new(ex::Numeral(0))), None);
    let r15 = test_expr(3.0, ex::Compare(Box::new(ex::Numeral(0)), cmp::NotEqual, Box::new(ex::True)), None);

    // 9 tests of functions
    // correct types, no args
    let r16 = test_expr(3.0, 
        ex::Let(Box::new(dc::FunDefn("f".to_string(), vec![], tp::Boolean,
        Box::new(ex::Compare(Box::new(ex::Numeral(0)), cmp::Equal, Box::new(ex::Numeral(0)))))),
        Box::new(ex::Call("f".to_string(), vec![]))), Some(tp::Boolean));
    // correct type, one arg
    let r17 = test_expr(3.0, 
        ex::Let(Box::new(dc::FunDefn("f".to_string(), vec![], tp::String,
        Box::new(ex::StringLiteral("brrr".to_string())))),
        Box::new(ex::Call("f".to_string(), vec![]))), Some(tp::String));
    // correct types, two args
    let r18 = test_expr(3.0, 
        ex::Let(Box::new(dc::FunDefn("ssqr".to_string(), vec![("x".to_string(), tp::Number), ("y".to_string(), tp::Number)],
        tp::Number,
        Box::new(ex::Plus(Box::new(ex::Times(Box::new(ex::Id("x".to_string())),Box::new(ex::Id("x".to_string())))), 
        Box::new(ex::Times(Box::new(ex::Id("x".to_string())),Box::new(ex::Id("x".to_string())))))))),
        Box::new(ex::Call("ssqr".to_string(), vec![ex::Numeral(1), ex::Numeral(2)]))), Some(tp::Number));
    // correct recursive call
    let r19 = test_expr(3.0,
        ex::Let(Box::new(dc::FunDefn("f".to_string(), vec![("y".to_string(), tp::Function(vec![tp::Boolean], Box::new(tp::String)))], tp::Number,
            Box::new(ex::Call("f".to_string(), vec![ex::Id("y".to_string())])))), Box::new(ex::Numeral(3))), Some(tp::Number));
    // incorrect - ill-typed body
    let r20 = test_expr(3.0,
    ex::Let(Box::new(dc::FunDefn("f".to_string(), vec![("y".to_string(), tp::Function(vec![tp::Boolean], Box::new(tp::String)))],tp::String,
    Box::new(ex::Compare(Box::new(ex::StringLiteral("hi".to_string())), cmp::Less, Box::new(ex::True))))), Box::new(ex::Numeral(3))), None);
    let r21 = test_expr(3.0, 
    // incorrect - too few args
    ex::Let(Box::new(dc::FunDefn("ssqr".to_string(), vec![("x".to_string(), tp::Number), ("y".to_string(), tp::Number)],tp::Number,
    Box::new(ex::Plus(Box::new(ex::Times(Box::new(ex::Id("x".to_string())),Box::new(ex::Id("x".to_string())))), 
    Box::new(ex::Times(Box::new(ex::Id("x".to_string())),Box::new(ex::Id("x".to_string())))))))),
    Box::new(ex::Call("ssqr".to_string(), vec![ex::Numeral(1)]))), None);
// incorrect - too many args
    let r22 = test_expr(3.0, 
        ex::Let(Box::new(dc::FunDefn("ssqr".to_string(), vec![("x".to_string(), tp::Number), ("y".to_string(), tp::Number)],tp::Number,
        Box::new(ex::Plus(Box::new(ex::Times(Box::new(ex::Id("x".to_string())),Box::new(ex::Id("x".to_string())))), 
        Box::new(ex::Times(Box::new(ex::Id("x".to_string())),Box::new(ex::Id("x".to_string())))))))),
        Box::new(ex::Call("ssqr".to_string(), vec![ex::Numeral(1), ex::Numeral(2), ex::Numeral(3)]))), None);
    
    // incorrect - mistyped argument
    let r23= test_expr(3.0,
        ex::Let(Box::new(dc::FunDefn("f".to_string(), vec![("y".to_string(), tp::Function(vec![tp::Boolean], Box::new(tp::String)))],tp::Boolean,
    Box::new(ex::Call("f".to_string(), vec![ex::True])))), Box::new(ex::Call("f".to_string(), vec![ex::True]))), None);

    // incorrect - ill-typed recursive call
    let r24 = test_expr(3.0, 
        ex::Let(Box::new(dc::FunDefn("f".to_string(), vec![("y".to_string(), tp::Function(vec![tp::Boolean], Box::new(tp::String)))],tp::Number,
    Box::new(ex::Call("f".to_string(), vec![ex::True])))), Box::new(ex::Numeral(3))), None);

    // correct type of function defn
    let r25 = test_defn(3.0,
      dc::FunDefn("f".to_string(), 
      vec![("h".to_string(), tp::Number)], tp::Boolean,Box::new(ex::Compare(Box::new(ex::Id("h".to_string())),cmp::Greater, Box::new(ex::Numeral(0))))),
    Some(("f".to_string(), tp::Function(vec![tp::Number], Box::new(tp::Boolean)))));

    let r = r1+r2+r3+r4+r5+r6+r7+r8+r9+r10+r11+r12+r13+r14+r15+r16+r17+r18+r19+r20+r21+r22+r23+r24+r25;
    println!("Results: {}/25 tests succesfully completed", r)
  }

