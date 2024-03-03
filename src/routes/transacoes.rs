use crate::domain::{Transacao, TransacaoDescricao, TransacaoTipo};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::{Pool, Postgres, Transaction};
use serde::Serialize;

#[derive(serde::Deserialize)]
pub struct TransacaoJson {
    pub valor: i32,
    pub tipo: char,
    pub descricao: String,
}

#[derive(thiserror::Error)]

pub enum TransactionError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("User doesn't exist")]
    UserDoesNotExist,
    #[error("Failed to update client balance")]
    BalanceUpdateError,
}

impl std::fmt::Debug for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for TransactionError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UserDoesNotExist => StatusCode::NOT_FOUND,
            Self::BalanceUpdateError => StatusCode::UNPROCESSABLE_ENTITY, // 422 status code
        }
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by: {}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[derive(Serialize)]
struct TransacaoResponse {
    limite: i32,
    saldo: i32,
}

impl TryFrom<TransacaoJson> for Transacao {
    type Error = String;
    fn try_from(value: TransacaoJson) -> Result<Self, Self::Error> {
        let tipo = TransacaoTipo::new(value.tipo)
            .map_err(|e| format!("Failed to parse the transaction type: {}", e))?;
        let descricao = TransacaoDescricao::new(value.descricao)
            .map_err(|e| format!("Failed to parse the transaction description: {}", e))?;
        Ok(Self {
            id: 0, // todo: we don't know the ID yet
            valor: value.valor,
            tipo,
            descricao: descricao,
            cliente_id: 0, // todo: we don't know the client ID yet
        })
    }
}

pub async fn register_transacao(
    transacao: web::Json<TransacaoJson>,
    pool: web::Data<Pool<Postgres>>,
    client_id: web::Path<i16>,
) -> Result<HttpResponse, TransactionError> {
    let client_id = client_id.into_inner();
    // if the client_id is different from 1 to 5, return 404
    if client_id < 1 || client_id > 5 {
        return Err(TransactionError::UserDoesNotExist);
    }
    // using try_into to convert the JSON payload into a domain object
    let new_transacao =
        Transacao::try_from(transacao.0).map_err(TransactionError::ValidationError)?;
    // unwrap the path parameter

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the Pool.")?;

    insert_transacao(&mut transaction, &new_transacao, client_id)
        .await
        .context("Failed to insert a new transaction.")?;

    let new_transacao_valor = if new_transacao.tipo.to_char() == 'c' {
        new_transacao.valor * 1
    } else if new_transacao.tipo.to_char() == 'd' {
        new_transacao.valor * -1
    } else {
        return Err(TransactionError::ValidationError(
            "Invalid transaction type".to_string(),
        ));
    };

    let (limite, saldo) = update_saldo_and_fetch(&mut transaction, client_id, new_transacao_valor)
    .await
    .map_err(|_| TransactionError::BalanceUpdateError)?; // Handle potential errors

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")?;
    Ok(HttpResponse::Ok().json(TransacaoResponse { limite, saldo }))
}

pub async fn update_saldo_and_fetch(
    transaction: &mut Transaction<'_, Postgres>,
    client_id: i16,
    valor: i32,
) -> Result<(i32,i32), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE cliente
        SET saldo = saldo + $1
        WHERE id = $2
        RETURNING limite, saldo;"#,
        valor,
        client_id
    )
    .fetch_one(transaction)
    .await
    .map(|record| (record.limite, record.saldo))
}

pub async fn insert_transacao(
    transaction: &mut Transaction<'_, Postgres>,
    transacao: &Transacao,
    client_id: i16,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO transacao (valor, tipo, descricao, cliente_id)
        VALUES ($1, $2, $3, $4)
        "#,
        transacao.valor,
        transacao.tipo.to_string(),
        transacao.descricao.as_ref(),
        client_id
    )
    .execute(transaction)
    .await?;
    Ok(())
}
