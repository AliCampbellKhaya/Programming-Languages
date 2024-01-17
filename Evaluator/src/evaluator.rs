use rpds::HashTrieMap;
/**
 * An implementation of an evaluator that evaluates declarations and expressions
 * 
 * The evaluator follows the following pseudocode specifications:
 *  eval_expr(E, Id(x)) = E(x)
 *  eval_expr(E, Number(n)) = n
 *  eval_expr(E, Times(e1,e2)) = interp_expr(E, e1) * interp_expr(E, e2)
 *  eval_expr(E, Plus(e1,e2)) = interp_expr(E, e1) + interp_expr(E, e2)
 *  eval_expr(E, Minus(e1,e2)) = interp_expr(E, e1) - interp_expr(E, e2)
 *  eval_expr(E, Let(d,e)) = interp_expr(interp_defn(E,d), e)
 *  eval_expr(E, Call(f,e1)) = interp_expr(E[x↦interp_expr(E,e1)], e2)
 *                               where E(f(x))=e2
 *  eval_defn(E,Var(x,e)) = E[x ↦ interp_expr(E, e)]
 *  eval_defn(E,Fun(f,x,e)) = E[f(x)↦e]
 */

/* EnvRecord defines a single record stored in the environment.
 * The name of a function or variable is its key in the environment.
 * FunRecord stores the argument names and the function body expression
 * VarRecord stores the value of the variable */
#[derive(Hash,Eq, PartialEq, Debug)]
pub enum EnvRecord {
    FunRecord(Vec<String>, Box<Expr>),
    VarRecord(Value),
}

/* Values are programs that are pure data and require no further
 * computation (i.e. numerals) */
#[derive(Hash,Eq, PartialEq, Debug, Clone)]
pub enum Value {
    Numeral(i64),
}

/* Expressions are programs that we can evaluate. If they terminate,
* they return a value. Expressions can be:
* Id: identifiers (variable names)
* Numeral: literal numbers
* Times: e1 * e2
* Plus: e1 + e2
* Let: let d in e  (see Defn for the different kinds of d)
* Call:  f(arg1, ..., argN)  (function calls, any number of args)
*/
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Expr {
    Id(String),
    Numeral(i64),
    Times(Box<Expr>,Box<Expr>),
    Plus(Box<Expr>,Box<Expr>),
    Minus(Box<Expr>,Box<Expr>),
    Let(Box<Defn>,Box<Expr>),
    Call(String, Vec<Expr>),
} 

/** Definitions are programs that, when we run them,
 * they define something, like a variable or function.
 * They can be:
 *   VarDefn(x,e) = defines x to equal the value of e
 *   FunDefn(f,[x1,...,xN],e) = defines function f(x1,...,xN)=e
 */
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Defn {
    VarDefn(String, Box<Expr>),
    FunDefn(String, Vec<String>, Box<Expr>),
}

pub fn eval_defn(env: &HashTrieMap<String,EnvRecord>, d: &Defn) -> HashTrieMap<String,EnvRecord> {
  // match definition type to variable or function
  match d {
    Defn::VarDefn(var, val) => {
      HashTrieMap::insert(env, var.to_string(), EnvRecord::VarRecord(eval_expr(env, &*val)))
    },
    Defn::FunDefn(func, param , val) => {
      HashTrieMap::insert(env, func.to_string(), EnvRecord::FunRecord(param.to_vec(), val.clone()))
    },
  }
}

pub fn eval_expr(env: &HashTrieMap<String,EnvRecord>, e: &Expr) -> Value {
  // match expression type to correct type
  match e {
    Expr::Id(s) => {
      match HashTrieMap::get(env, s) {
        Some(env_record) => {
          match env_record {
              EnvRecord::VarRecord(v) => {
                v.clone()
              },
              EnvRecord::FunRecord(f, e) => {
                Value::Numeral(0)
              }
          }
        },
        None => {
          Value::Numeral(0)
        }
      }
    },
    Expr::Numeral(n) => {
      Value::Numeral(*n)
    },
    Expr::Times(l, r) => {
      let Value::Numeral(l_ret) = eval_expr(env, &*l);
      let Value::Numeral(r_ret) = eval_expr(env, &*r);
      Value::Numeral(l_ret * r_ret)
    },
    Expr::Plus(l, r) => {
      let Value::Numeral(l_ret) = eval_expr(env, &*l);
      let Value::Numeral(r_ret) = eval_expr(env, &*r);
      Value::Numeral(l_ret + r_ret)
    },
    Expr::Minus(l, r) => {
      let Value::Numeral(l_ret) = eval_expr(env, &*l);
      let Value::Numeral(r_ret) = eval_expr(env, &*r);
      Value::Numeral(l_ret - r_ret)
    },
    Expr::Let(d, e) => {
      let update_env = eval_defn(env, d);
      eval_expr(&update_env, e)
    },
    Expr::Call(f, a) => {
      match HashTrieMap::get(env, f) {
        Some(env_record) => {
          match env_record {
              EnvRecord::VarRecord(v) => {
                v.clone()
              },
              EnvRecord::FunRecord(p, e) => {
                let mut update_env = env.clone();
                for (pi, ai) in p.iter().zip(a.iter()) {
                  update_env = eval_defn(&update_env, &Defn::VarDefn(pi.clone(), Box::new(ai.clone())))
                }
                eval_expr(&update_env, e)
              }
          }
        },
        None => {
          Value::Numeral(0)
        }
      }
    },
  }
}

/* Materials Copyright Rose Bohrer 2023, Completed and Edited by Alasdair Campbell 2023 */