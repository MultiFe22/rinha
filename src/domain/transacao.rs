use crate::domain::transacao_descricao::TransacaoDescricao;
use crate::domain::transacao_tipo::TransacaoTipo;

pub struct Transacao {
    pub id: i32,                       // SERIAL in PostgreSQL maps to i32 in Rust
    pub valor: i32,                    // INTEGER maps to i32
    pub tipo: TransacaoTipo, // CHAR maps to char, but consider using String if dealing with database ORMs
    pub descricao: TransacaoDescricao, // VARCHAR maps to String
    pub cliente_id: i16,     // SMALLINT maps to i16
}
