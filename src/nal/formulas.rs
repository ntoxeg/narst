//! Formulas

use super::TruthValue;

// (= (Truth_w2c $w) (/ $w (+ $w 1)))
pub fn truth_w2c(w: f32) -> f32 {
    w / (w + 1.0)
}

pub fn deduction(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    TruthValue::new(
        tv1.strength() * tv2.strength(),
        tv1.strength() * tv2.strength() * tv1.confidence() * tv2.confidence(),
    )
}

pub fn abduction(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    TruthValue::new(
        tv2.strength(),
        truth_w2c(tv1.strength() * tv1.confidence() * tv2.confidence()),
    )
}

pub fn induction(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    abduction(tv2, tv1)
}

pub fn exemplification(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    TruthValue::new(
        1.0,
        truth_w2c(tv1.strength() * tv2.strength() * tv1.confidence() * tv2.confidence()),
    )
}

pub fn structural_deduction(tv1: TruthValue) -> TruthValue {
    let tv = TruthValue::new(1.0, 0.9);
    deduction(tv1, tv)
}

pub fn negation(tv1: TruthValue) -> TruthValue {
    TruthValue::new(1.0 - tv1.strength(), tv1.confidence())
}

pub fn structural_deduction_negated(tv1: TruthValue) -> TruthValue {
    let tsd = structural_deduction(tv1);
    negation(tsd)
}

pub fn intersection(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    TruthValue::new(
        tv1.strength() * tv2.strength(),
        tv1.confidence() * tv2.confidence(),
    )
}

pub fn structural_intersection(tv1: TruthValue) -> TruthValue {
    let tv = TruthValue::new(1.0, 0.9);
    intersection(tv1, tv)
}

pub fn or(a: f32, b: f32) -> f32 {
    1.0 - (1.0 - a) * (1.0 - b)
}

pub fn comparison(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    let f0 = or(tv1.strength(), tv2.strength());
    let f_ans = if f0 == 0.0 {
        0.0
    } else {
        (tv1.strength() * tv2.strength()) / f0
    };
    TruthValue::new(f_ans, truth_w2c(f0 * tv1.confidence() * tv2.confidence()))
}

pub fn analogy(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    TruthValue::new(
        tv1.strength() * tv2.strength(),
        tv1.confidence() * tv2.confidence() * tv2.strength(),
    )
}

pub fn resemblance(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    TruthValue::new(
        tv1.strength() * tv2.strength(),
        tv1.confidence() * tv2.confidence() * or(tv1.strength(), tv2.strength()),
    )
}

pub fn union(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    TruthValue::new(
        or(tv1.strength(), tv2.strength()),
        tv1.confidence() * tv2.confidence(),
    )
}

pub fn difference(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    TruthValue::new(
        tv1.strength() * (1.0 - tv2.strength()),
        tv1.confidence() * tv2.confidence(),
    )
}

pub fn decompose_pnn(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    let fn_ = tv1.strength() * (1.0 - tv2.strength());
    TruthValue::new(1.0 - fn_, fn_ * tv1.confidence() * tv2.confidence())
}

pub fn decompose_npp(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    let f = (1.0 - tv1.strength()) * tv2.strength();
    TruthValue::new(f, f * tv1.confidence() * tv2.confidence())
}

pub fn decompose_pnp(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    let f = tv1.strength() * (1.0 - tv2.strength());
    TruthValue::new(f, f * tv1.confidence() * tv2.confidence())
}

pub fn decompose_ppp(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    let negtv = negation(tv1);
    decompose_npp(negtv, tv2)
}

pub fn decompose_nnn(tv1: TruthValue, tv2: TruthValue) -> TruthValue {
    let fn_ = (1.0 - tv1.strength()) * (1.0 - tv2.strength());
    TruthValue::new(1.0 - fn_, fn_ * tv1.confidence() * tv2.confidence())
}
