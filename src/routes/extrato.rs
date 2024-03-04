use actix_web::{web, HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use serde::Serialize;
use sqlx::{PgPool, Pool, Postgres};

use crate::routes::error_chain_fmt;

#[derive(Serialize)]
pub struct Saldo {
    total: i32,
    data_extrato: String,
    limite: i32,
}

#[derive(Serialize)]
pub struct TransacaoExtrato {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
    pub realizada_em: String,
}

#[derive(Serialize)]
pub struct Extrato {
    saldo: Saldo,
    ultimas_transacoes: Vec<TransacaoExtrato>,
}


#[derive(thiserror::Error)]

pub enum ExtratoError{  
    #[error("Client not found")]
    NotFound,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ExtratoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ExtratoError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn get_extrato(
    pool: web::Data<Pool<Postgres>>,
    id: web::Path<i16>,
) -> Result<HttpResponse, ExtratoError> {
    let client_id = id.into_inner();
    // if the client_id is different from 1 to 5, return 404
    if client_id < 1 || client_id > 5 {
        return Err(ExtratoError::NotFound);
    }
    let (saldo_total, limite, data_extrato) = get_saldo_limite(&pool, client_id).await?;
    let last_transacoes = get_last_10_transacoes(&pool, client_id).await?;
    let saldo = Saldo {
        total: saldo_total,
        data_extrato,
        limite,
    };
    let ultimas_transacoes = last_transacoes.into_iter().map(|transacao| {
        TransacaoExtrato {
            valor: transacao.valor,
            tipo: transacao.tipo,
            descricao: transacao.descricao,
            realizada_em: transacao.realizada_em,
        }
    }).collect();
    Ok(HttpResponse::Ok().json(Extrato {
        saldo,
        ultimas_transacoes,
    }))
}

pub async fn get_saldo_limite(pool: &PgPool, id: i16) -> Result<(i32, i32, String), anyhow::Error> {
    let saldo_limite = sqlx::query!(
        "SELECT saldo, limite, NOW() FROM cliente WHERE id = $1;",
        id
    )
    .fetch_one(pool)
    .await?;
    Ok((saldo_limite.saldo, saldo_limite.limite, saldo_limite.now.unwrap().to_string()))
}

pub async fn get_last_10_transacoes(
    pool: &PgPool,
    id: i16,
) -> Result<Vec<TransacaoExtrato>, anyhow::Error> {
    let last_transacoes = sqlx::query!(
        "SELECT valor, tipo, descricao, realizada_em FROM transacao
    WHERE cliente_id = $1
    ORDER BY realizada_em DESC, id DESC
    LIMIT 10;
    ",
        id
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|t| TransacaoExtrato {
        valor: t.valor,
        tipo: t.tipo,
        descricao: t.descricao,
        realizada_em: t.realizada_em.to_string(),
    })
    .collect();
    Ok(last_transacoes)
}
