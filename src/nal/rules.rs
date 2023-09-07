//! Syllogistic rules for inference
// NAL-1
// Syllogistic rules for Inheritance

// (= (|- (($a --> $b) $T1) (($b --> $c) $T2)) (($a --> $c) (Truth_Deduction $T1 $T2)))
fn term_rewrite_deduction(fa: f32, ca: f32, fb: f32, cb: f32, fc: f32, cc: f32) -> (f32, f32) {
    truth_deduction(fa, ca, fb, cb, fc, cc)
}
