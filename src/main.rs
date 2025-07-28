use crate::lib::simple::simplify;

fn main() {
    println!("Hello, world!");
    simplify("(* I (* X Z))");
    simplify("(* Z Z)");
    // assert_eq!(simplify("(* 0 42)"), "0");
    // assert_eq!(simplify("(+ 0 (* 1 foo))"), "foo");
    // simplify("(+ (* foo (+ 1 x)) (* 10 (* foo (+ 1 x))))");
    // // Since parsing can return an error, `unwrap` just panics if the result doesn't return Ok
    // let my_expression: RecExpr<SymbolLang> = "(foo a b)".parse().unwrap();
    // println!("this is my expression {}", my_expression);

    // // let's try to create an e-node, but hmmm, what do I put as the children?
    // let my_enode = SymbolLang::new("bar", vec![]);
}
