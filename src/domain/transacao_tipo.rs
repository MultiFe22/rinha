use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct TransacaoTipo(char);

impl TransacaoTipo {
    pub fn new(c: char) -> Result<TransacaoTipo, String> {
        match c {
            'c' | 'd' => Ok(Self(c)),
            _ => Err(format!("{} is not a valid transaction type", c)),
        }
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn to_char(&self) -> char {
        self.0
    }
}

impl std::fmt::Display for TransacaoTipo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Forward to the Display implementation of the wrapped char.
        write!(f, "{}", self.0)
    }
}

impl AsRef<char> for TransacaoTipo {
    fn as_ref(&self) -> &char {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::TransacaoTipo;
    use claims::{assert_err, assert_ok};

    #[test]
    fn character_not_c_or_d_is_rejected() {
        let tipo = TransacaoTipo::new('x');
        assert_err!(tipo);
    }

    #[test]
    fn character_c_is_accepted() {
        let tipo = TransacaoTipo::new('c');
        assert_ok!(tipo);
    }

    #[test]
    fn character_d_is_accepted() {
        let tipo = TransacaoTipo::new('d');
        assert_ok!(tipo);
    }
}
