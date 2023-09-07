//! Formulas

// (= (Truth_w2c $w) (/ $w (+ $w 1)))
fn truth_w2c(w: f32) -> f32 {
    w / (w + 1.0)
}

// (= (Truth_Deduction ($f1 $c1) ($f2 $c2)) ((* $f1 $f2) (* (* $f1 $f2) (* $c1 $c2))))
fn truth_deduction(f1: f32, c1: f32, f2: f32, c2: f32) -> TruthValue {
    TruthValue::new(f1 * f2, f1 * f2 * c1 * c2)
}

// (= (Truth_Abduction ($f1 $c1) ($f2 $c2)) ($f2 (Truth_w2c (* (* $f1 $c1) $c2))))
fn truth_abduction(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    (f2, truth_w2c(f1 * c1 * c2))
}

// (= (Truth_Induction $T1 $T2) (Truth_Abduction $T2 $T1))
fn truth_induction(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    truth_abduction(f2, c2, f1, c1)
}

// (= (Truth_Exemplification ($f1 $c1) ($f2 $c2)) (1.0 (Truth_w2c (* (* $f1 $f2) (* $c1 $c2)))))
fn truth_exemplification(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    (1.0, truth_w2c(f1 * f2 * c1 * c2))
}

// (= (Truth_StructuralDeduction $T) (Truth_Deduction $T (1.0 0.9)))
fn truth_structural_deduction(f1: f32, c1: f32) -> (f32, f32) {
    truth_deduction(f1, c1, 1.0, 0.9)
}

// (= (Truth_Negation ($f $c)) ((- 1 $f) $c))
fn truth_negation(f: f32, c: f32) -> (f32, f32) {
    (1.0 - f, c)
}

// (= (Truth StructuralDeductionNegated $T) (Truth_Negation (Truth_StructuralDeduction $T)))
fn truth_structural_deduction_negated(f1: f32, c1: f32) -> (f32, f32) {
    let tsd = truth_structural_deduction(f1, c1);
    truth_negation(tsd.0, tsd.1)
}
// (= (Truth_Intersection ($f1 $c1) ($f2 $c2)) ((* $f1 $f2) (* $c1 $c2)))
fn truth_intersection(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    (f1 * f2, c1 * c2)
}

// (= (Truth_StructuralIntersection $T) (Truth_Intersection $T (1.0 0.9)))
fn truth_structural_intersection(f1: f32, c1: f32) -> (f32, f32) {
    truth_intersection(f1, c1, 1.0, 0.9)
}

// (= (Truth_or $a $b) (- 1 (* (- 1 $a) (- 1 $b))))
fn truth_or(a: f32, b: f32) -> f32 {
    1.0 - (1.0 - a) * (1.0 - b)
}

// TODO: check for bugs from here
// (= (Truth_Comparison ($f1 $c1) ($f2 $c2)) (let $f0 (Truth_or $f1 $f2) ((if (== $f0 0.0) 0.0 (/ (* $f1 $f2) $f0)) (Truth_w2c (* $f0 (* $c1 $c2))))))
fn truth_comparison(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    let f0 = truth_or(f1, f2);
    if f0 == 0.0 {
        (0.0, truth_w2c(f1 * f2 * c1 * c2))
    } else {
        (f0, truth_w2c(f0 * f1 * f2 * c1 * c2))
    }
}

// (= (Truth_Analogy ($f1 $c1) ($f2 $c2)) ((* $f1 $f2) (* (* $c1 $c2) $f2)))
fn truth_analogy(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    (f1 * f2, c1 * f2)
}

// (= (Truth_Resemblance ($f1 $c1) ($f2 $c2)) ((* $f1 $f2) (* (* $c1 $c2) (Truth_or $f1 $f2))))
fn truth_resemblance(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    (f1 * f2, c1 * truth_or(f1, f2))
}

// (= (Truth_Union ($f1 $c1) ($f2 $c2)) ((Truth_or $f1 $f2) (* $c1 $c2)))
fn truth_union(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    (truth_or(f1, f2), c1 * c2)
}

// (= (Truth_Difference ($f1 $c1) ($f2 $c2)) ((* $f1 (- 1 $f2)) (* $c1 $c2)))
fn truth_difference(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    (f1 * (1.0 - f2), c1 * c2)
}

// (= (Truth_DecomposePNN ($f1 $c1) ($f2 $c2)) (let $fn (* $f1 (- 1 $f2)) ((- 1 $fn) (* $fn (* $c1 $c2)))))
fn truth_decompose_pnn(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    let fn_ = f1 * (1.0 - f2);
    (-1.0 - fn_, fn_ * c1 * c2)
}

// (= (Truth_DecomposeNPP ($f1 $c1) ($f2 $c2)) (let $f (* (- 1 $f1) $f2) ($f (* $f (* $c1 $c2)))))
fn truth_decompose_npp(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    let f = f1 * (1.0 - f2);
    (f * (1.0 - f), f * c1 * c2)
}

// (= (Truth_DecomposePNP ($f1 $c1) ($f2 $c2)) (let $f (* $f1 (- 1 $f2)) ($f (* $f (* $c1 $c2)))))
fn truth_decompose_pnp(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    let f = f1 * (1.0 - f2);
    (f * (1.0 - f), f * c1 * c2)
}

// (= (Truth_DecomposePPP $v1 $v2) (Truth_DecomposeNPP (Truth_Negation $v1) $v2))
fn truth_decompose_ppp(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    truth_decompose_npp(truth_negation(f1, c1), f2, c2)
}

// (= (Truth_DecomposeNNN ($f1 $c1) ($f2 $c2)) (let $fn (* (- 1 $f1) (- 1 $f2)) ((- 1 $fn) (* $fn (* $c1 $c2)))))
fn truth_decompose_nnn(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    let fn_ = f1 * (1.0 - f2);
    (-1.0 - fn_, -1.0 - fn_ * c1 * c2)
}
