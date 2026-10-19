use cf_types::ContextEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectionPlan {
    pub selected: Vec<ContextEvent>,
    pub used_tokens: u32,
    pub covered_kinds: usize,
}
#[must_use]
pub fn select(events: &[ContextEvent], budget: u32) -> SelectionPlan {
    let mut ranked = events.to_vec();
    ranked.sort_by(|a, b| {
        let ar = a.importance / f64::from(a.tokens.max(1));
        let br = b.importance / f64::from(b.tokens.max(1));
        br.total_cmp(&ar)
            .then_with(|| a.id.to_string().cmp(&b.id.to_string()))
    });
    let mut chosen = Vec::new();
    let mut used = 0;
    let mut kinds = HashSet::new();
    while !ranked.is_empty() {
        let best = ranked
            .iter()
            .enumerate()
            .filter(|(_, e)| used + e.tokens <= budget)
            .max_by(|(_, a), (_, b)| {
                let au = a.importance + if kinds.contains(&a.kind) { 0.0 } else { 0.20 };
                let bu = b.importance + if kinds.contains(&b.kind) { 0.0 } else { 0.20 };
                au.total_cmp(&bu)
            })
            .map(|(i, _)| i);
        let Some(i) = best else { break };
        let e = ranked.remove(i);
        used += e.tokens;
        kinds.insert(e.kind.clone());
        chosen.push(e)
    }
    SelectionPlan {
        selected: chosen,
        used_tokens: used,
        covered_kinds: kinds.len(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cf_types::Sensitivity;
    #[test]
    fn respects_budget_and_diversity() {
        let a = ContextEvent {
            tokens: 4,
            ..ContextEvent::new("a", "x", "a", Sensitivity::Public, 0.8).unwrap()
        };
        let b = ContextEvent {
            tokens: 4,
            ..ContextEvent::new("b", "y", "b", Sensitivity::Public, 0.7).unwrap()
        };
        let p = select(&[a, b], 4);
        assert_eq!(p.used_tokens, 4);
        assert_eq!(p.covered_kinds, 1);
    }
}
