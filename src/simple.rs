use crate::phasedpauli::PhasedPauliEnum;
use egg::*;

define_language! {
    enum PauliLanguage{
        PhasedPauli(PhasedPauliEnum),
        "*" = Mul([Id; 2]),
    }
}

fn make_rules() -> Vec<Rewrite<PauliLanguage, ()>> {
    vec![
        rewrite!("Identity Multiplication"; "(* ?a I)" => "?a"),
        rewrite!("Identity Commutes"; "(* I ?a)" => "(* ?a I)"),
        rewrite!("XZ rule"; "(* X Z)" => "-imY"),
        rewrite!("Unipotency"; "(* ?a ?a)" => "I"),
    ]
}

/// parse an expression, simplify it using egg, and pretty print it back out
pub fn simplify(s: &str) -> String {
    // parse the expression, the type annotation tells it which Language to use
    let expr: RecExpr<PauliLanguage> = s.parse().unwrap();

    // simplify the expression using a Runner, which creates an e-graph with
    // the given expression and runs the given rules over it
    let runner = Runner::default().with_expr(&expr).run(&make_rules());

    // the Runner knows which e-class the expression given with `with_expr` is in
    let root = runner.roots[0];

    // use an Extractor to pick the best element of the root eclass
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best) = extractor.find_best(root);
    println!("Simplified {} to {} with cost {}", expr, best, best_cost);
    best.to_string()
}
