use cf_transfer::TransferEnvelope;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum VerifyError {
    #[error("missing or invalid witness")]
    Witness,
    #[error("capsule exceeds budget")]
    Budget,
}
pub fn verify_envelope(envelope: &TransferEnvelope) -> Result<(), VerifyError> {
    if envelope.capsule.used_tokens > envelope.capsule.budget {
        return Err(VerifyError::Budget);
    }
    let witness = envelope.witness.as_ref().ok_or(VerifyError::Witness)?;
    cf_rvf::verify(witness).map_err(|_| VerifyError::Witness)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn type_is_fallible() {
        let _: fn(&TransferEnvelope) -> Result<(), VerifyError> = verify_envelope;
    }
}
