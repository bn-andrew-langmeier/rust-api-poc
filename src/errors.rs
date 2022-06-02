use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use tokio_pg_mapper::Error as PostgresMapperError;
use tokio_postgres::error::Error as PostgresError;

#[derive(Display, From, Debug)]
pub enum Errors {
    NotFound,
    PostgresError(PostgresError),
    PostgresMapperError(PostgresMapperError),
    PoolError(PoolError),
}
impl std::error::Error for Errors {}

impl ResponseError for Errors {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Errors::NotFound => HttpResponse::NotFound().finish(),
            Errors::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            },
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
