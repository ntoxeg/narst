//! Non-Axiomatic Logic fuctionality
pub mod parser;

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
pub struct Term {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Judgement {
    pub term: Term,
    pub tv: TruthValue,
    pub tense: Tense,
}

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub term: Term,
    pub tv: TruthValue,
    pub tense: Tense,
}

#[derive(Serialize, Deserialize)]
pub struct Goal {
    pub term: Term,
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

        let term = Term {
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
