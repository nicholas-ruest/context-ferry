use cf_selection::SelectionPlan;
#[must_use]
pub fn omission_loss(plan: &SelectionPlan, total_importance: f64) -> f64 {
    if total_importance <= f64::EPSILON {
        return 0.0;
    }
    let selected: f64 = plan.selected.iter().map(|e| e.importance).sum();
    (1.0 - selected / total_importance).clamp(0.0, 1.0)
}
#[must_use]
pub fn upper_bound(loss: f64, calibration: &[f64], alpha: f64) -> f64 {
    if calibration.is_empty() {
        return 1.0;
    }
    let mut c = calibration.to_vec();
    c.sort_by(f64::total_cmp);
    let n = c.len();
    let rank = (((n + 1) as f64 * (1.0 - alpha)).ceil() as usize).clamp(1, n) - 1;
    (loss + c[rank]).clamp(0.0, 1.0)
}
#[must_use]
pub fn permits(bound: f64, ceiling: f64) -> bool {
    bound <= ceiling
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finite_quantile_is_conservative() {
        assert!(upper_bound(0.2, &[0.01, 0.03, 0.05, 0.08], 0.1) >= 0.25);
    }
}
