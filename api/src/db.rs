use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::PersonEntity;

#[derive(Clone)]
pub struct Db {
    pub pool: Pool<Postgres>,
}

impl Db {
    pub async fn insert(&self, person: &PersonEntity) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "insert into person (id, handle, payload, search) values($1, $2, $3, $4)",
            &person.id,
            person.handle.as_str(),
            &person.payload,
            &person.search
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_by_id(&self, id: &Uuid) -> Result<Option<String>, sqlx::Error> {
        let record = sqlx::query_scalar!("select payload from person where id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(record)
    }

    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!("select count(*) from person")
            .fetch_one(&self.pool)
            .await?
            .unwrap_or(0);

        Ok(count)
    }

    pub async fn search(&self, term: String) -> Result<Vec<String>, sqlx::Error> {
        let matches = sqlx::query_scalar!(
            "select payload from person where search ilike $1 limit 50",
            format!("%{}%", term)
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(matches)
    }
}
