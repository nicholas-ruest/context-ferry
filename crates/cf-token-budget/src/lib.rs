use thiserror::Error;
#[derive(Debug, Error)]
pub enum BudgetError {
    #[error("token budget exceeded: {used}+{requested}>{limit}")]
    Exceeded {
        used: u32,
        requested: u32,
        limit: u32,
    },
}
#[derive(Clone, Debug)]
pub struct BudgetLedger {
    limit: u32,
    used: u32,
}
impl BudgetLedger {
    #[must_use]
    pub const fn new(limit: u32) -> Self {
        Self { limit, used: 0 }
    }
    pub fn reserve(&mut self, tokens: u32) -> Result<(), BudgetError> {
        if self.used.saturating_add(tokens) > self.limit {
            return Err(BudgetError::Exceeded {
                used: self.used,
                requested: tokens,
                limit: self.limit,
            });
        }
        self.used += tokens;
        Ok(())
    }
    #[must_use]
    pub const fn used(&self) -> u32 {
        self.used
    }
    #[must_use]
    pub const fn remaining(&self) -> u32 {
        self.limit - self.used
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_overflow() {
        let mut b = BudgetLedger::new(5);
        b.reserve(4).unwrap();
        assert!(b.reserve(2).is_err());
    }
}
