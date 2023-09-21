//! Syllogistic rules for inference
// NAL-1
// Syllogistic rules for Inheritance

use super::formulas::*;
use super::Term;
use super::TermInfo;
use super::TruthValue;

// (= (|- (($a --> $b) $T1) (($b --> $c) $T2)) (($a --> $c) (Truth_Deduction $T1 $T2)))
pub fn rewrite_deduction(tab: Term, tbc: Term) -> (Term, TruthValue) {
    let expr_deduction: String = format!("<{} --> {}>", tab.info.expr, tbc.info.expr);
    let tv_deduction: TruthValue = deduction(tab.info.tv, tbc.info.tv);
    let term_deduction: Term = Term {
        info: TermInfo {
            id: 0,
            expr: expr_deduction,
            tv: tv_deduction,
        },
    };
    (term_deduction, tv_deduction)
}
