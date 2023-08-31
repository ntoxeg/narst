//! Non-Axiomatic Logic fuctionality
pub mod parser;

use nom::sequence::Tuple;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct EvidentialValue {
    s: f32,
    c: f32,
}

impl EvidentialValue {
    fn new(s: f32, c: f32) -> Self {
        if s < 0.0 || s > 1.0 {
            panic!("The strength value must lie in the interval [0.0; 1.0].");
        }
        if c < 0.0 || c >= 1.0 {
            panic!("The confidence value must lie in the interval [0.0; 1.0).");
        }

        Self { s, c }
    }
}

/// The Truth Value type
/// Contains two floating point numbers needed for all reasoning calculations.
///
/// # Examples
/// ```
/// use narst::nal::TruthValue;
///
/// let frequency = 0.35;
/// let confidence = 0.51;
/// let tv = TruthValue::new(frequency, confidence);
/// assert_eq!(tv.strength(), 0.35);
/// assert_eq!(tv.confidence(), 0.51);
/// ```
///
/// # Panics
/// The strength / frequency must lie in the interval [0.0; 1.0].
/// The confidence must lie in the interval [0.0; 1.0).
#[derive(Serialize, Deserialize)]
pub struct TruthValue {
    ev: EvidentialValue,
}

impl TruthValue {
    pub fn new(strength: f32, confidence: f32) -> Self {
        Self {
            ev: EvidentialValue::new(strength, confidence),
        }
    }

    pub fn strength(&self) -> f32 {
        self.ev.s
    }
    pub fn confidence(&self) -> f32 {
        self.ev.c
    }
    fn from(maybe_tvstr: Option<(&str, &str)>) -> Option<TruthValue> {
        maybe_tvstr.and_then(|(s, c)| {
            let strength = s.parse::<f32>().ok()?;
            let confidence = c.parse::<f32>().ok()?;
            Some(TruthValue::new(strength, confidence))
        })
    }
}

/// Desire value
/// For a virtual judgement S => D,
/// how much the associated statement S implies the overall desired state of NARS, D.
///
/// # Panics
/// The strength / frequency must lie in the interval [0.0; 1.0].
/// The confidence must lie in the interval [0.0; 1.0).
#[derive(Serialize, Deserialize)]
pub struct DesireValue {
    ev: EvidentialValue,
}

impl DesireValue {
    pub fn new(strength: f32, confidence: f32) -> Self {
        Self {
            ev: EvidentialValue::new(strength, confidence),
        }
    }

    pub fn strength(&self) -> f32 {
        self.ev.s
    }
    pub fn confidence(&self) -> f32 {
        self.ev.c
    }
    fn fromtv(tv: &TruthValue) -> Self {
        Self {
            ev: EvidentialValue::new(tv.strength(), tv.confidence()),
        }
    }
}

// sentence structs

#[derive(Serialize, Deserialize)]
pub enum Tense {
    Present,
    Past,
    Future,
    Eternal,
}

impl Tense {
    // TODO: refactor to use this privately without `Result`
    pub fn from(name: Option<&str>) -> Result<Self, String> {
        match name {
            Some(":|:") => Ok(Tense::Present),
            Some(":\\:") => Ok(Tense::Past),
            Some(":/:") => Ok(Tense::Future),
            None => Ok(Tense::Eternal),
            _ => Err(format!("invalid tense: {}", name.unwrap())),
        }
    }
}



#[derive(Serialize, Deserialize)]
pub struct Judgement {
    pub term: AtomicTerm,
    pub tv: TruthValue,
    pub tense: Tense,
}

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub term: AtomicTerm,
    pub tv: TruthValue,
    pub tense: Tense,
}

#[derive(Serialize, Deserialize)]
pub struct Goal {
    pub term: AtomicTerm,
    pub d: DesireValue,
    pub tense: Tense,
}

pub enum Sentence {
    J(Judgement),
    Q(Question),
    G(Goal),
}

impl Sentence {
    /// Produces a Sentence from a tuple containing the sentence itself and its type, signified by punctuation.
    pub fn from(sttv: ((&str, &str), Tense, Option<(&str, &str)>)) -> Result<Self, String> {
        let ((expr, pstr), tense, tvstr) = sttv;

        let term = AtomicTerm {
            name: String::from(expr),
        };
        let punctuation = pstr.chars().last();
        let tv = TruthValue::from(tvstr).unwrap_or(TruthValue::new(1.0, 0.5));
        match punctuation {
            Some('.') => Ok(Sentence::J(Judgement { term, tv, tense })),
            Some('?') => Ok(Sentence::Q(Question { term, tv, tense })),
            Some('!') => Ok(Sentence::G(Goal {
                term,
                d: DesireValue::fromtv(&tv),
                tense,
            })),
            _ => Err(format!("invalid punctuation: {}", pstr)),
        }
    }
}

// terms

#[derive(Serialize, Deserialize)]
struct TermInfo {
    pub id: u32,
    pub expr: String,
}

#[derive(Serialize, Deserialize)]
pub struct AtomicTerm {
    info: TermInfo,
}

impl AtomicTerm {
    pub fn new(expr: &str) -> Self { Self { info: TermInfo { id: 0, expr: String::from(expr) } } }

    pub fn name(&self) -> &str { &self.info.expr }
}

enum VarT {
    Independent,
    Dependent,
    Query,
}

// Formulas

// (= (Truth_w2c $w) (/ $w (+ $w 1)))
fn truth_w2c(w: f32) -> f32 {
    w / (w + 1.0)
}

// (= (Truth_Deduction ($f1 $c1) ($f2 $c2)) ((* $f1 $f2) (* (* $f1 $f2) (* $c1 $c2))))
fn truth_deduction(f1: f32, c1: f32, f2: f32, c2: f32) -> (f32, f32) {
    (f1 * f2, (f1 * f2 * c1 * c2))
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

// NAL-1
// Syllogistic rules for Inheritance

// (= (|- (($a --> $b) $T1) (($b --> $c) $T2)) (($a --> $c) (Truth_Deduction $T1 $T2)))
fn term_rewrite_deduction(fa: f32, ca: f32, fb: f32, cb: f32, fc: f32, cc: f32) -> (f32, f32) {
    truth_deduction(fa, ca, fb, cb, fc, cc)
}
