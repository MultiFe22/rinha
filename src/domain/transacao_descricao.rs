use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TransacaoDescricao(String);

impl TransacaoDescricao {
    pub fn new(s: String) -> Result<TransacaoDescricao, String> {
        if s.len() >= 1 && s.len() <= 10 {
            Ok(Self(s))
        } else {
            Err(format!(
                "Description length must be between 1 and 10 characters, got {}",
                s.len()
            ))
        }
    }
}

impl std::fmt::Display for TransacaoDescricao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Forward to the Display implementation of the wrapped String.
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for TransacaoDescricao {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::TransacaoDescricao;
    use claims::{assert_err, assert_ok};

    #[test]
    fn description_too_short_is_rejected() {
        let desc = TransacaoDescricao::new("".to_string());
        assert_err!(desc);
    }

    #[test]
    fn description_too_long_is_rejected() {
        let desc = TransacaoDescricao::new("This is way too long".to_string());
        assert_err!(desc);
    }

    #[test]
    fn valid_description_is_accepted() {
        let valid_descs = vec!["a", "ab", "Correct!", "1234567890"];
        for desc in valid_descs {
            let desc = TransacaoDescricao::new(desc.to_string());
            assert_ok!(desc);
        }
    }
}
