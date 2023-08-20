#![feature(iter_intersperse)]

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use db::Db;
use log::{error, SetLoggerError};
use models::PersonPostDTO;
use serde::Deserialize;
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode};
use sqlx::postgres::PgPoolOptions;
use std::{env::var, net::SocketAddr};
use uuid::Uuid;

use crate::models::PersonEntity;

mod db;
mod models;

#[tokio::main]
async fn main() {
    init_logger().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(32)
        .connect(
            &var("DATABASE_URL")
                .unwrap_or("postgres://postgres:postgres@localhost:5432/rinha".to_owned()),
        )
        .await
        .unwrap();

    let db = Db { pool };

    let app = Router::new()
        .route("/pessoas", post(create))
        .route("/pessoas/:id", get(get_by_id))
        .route("/pessoas", get(search))
        .route("/contagem-pessoas", get(count))
        .with_state(db);

    axum::Server::bind(&SocketAddr::new(
        [0, 0, 0, 0].into(),
        var("PORT")
            .ok()
            .unwrap_or("9999".to_owned())
            .parse::<u16>()
            .unwrap(),
    ))
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn create(State(db): State<Db>, Json(dto): Json<PersonPostDTO>) -> impl IntoResponse {
    match PersonEntity::try_from(dto) {
        Ok(person) => match db.insert(&person).await {
            Ok(_) => Ok((
                StatusCode::CREATED,
                [(header::LOCATION, format!("/pessoas/{}", &person.id))],
            )),
            Err(sqlx::Error::Database(error)) if error.is_unique_violation() => {
                Err(StatusCode::UNPROCESSABLE_ENTITY)
            }
            Err(error) => {
                error!("create: {}", error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}

async fn get_by_id(State(db): State<Db>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let person = db.get_by_id(&id).await;

    match person {
        Ok(Some(payload)) => Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            payload,
        )),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(error) => {
            error!("get_by_id: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct SearchParam {
    pub t: String,
}
async fn search(State(db): State<Db>, Query(query): Query<SearchParam>) -> impl IntoResponse {
    let search_result = db.search(query.t).await;

    match search_result {
        Ok(matches) => {
            let mut response = String::with_capacity(400 * 50);
            response.push('[');
            matches
                .iter()
                .map(|m| m.as_str())
                .intersperse(",")
                .for_each(|p| response.push_str(p));
            response.push(']');

            Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "application/json")],
                response,
            ))
        }
        Err(error) => {
            error!("search: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn count(State(db): State<Db>) -> impl IntoResponse {
    let count = db.count().await;

    match count {
        Ok(count) => Ok(format!("{count}\n")),
        Err(error) => {
            error!("count: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn init_logger() -> Result<(), SetLoggerError> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )])
}
