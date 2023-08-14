use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::Person;

#[derive(Clone)]
pub struct Db {
    pub pool: Pool<Postgres>,
}

impl Db {
    pub async fn insert(&self, person: &Person) -> Result<(), sqlx::Error> {
        sqlx::query::<Postgres>(
            "insert into person (id, handle, payload, search_vector) values($1, $2, $3, to_tsvector($4))"
        )
        .bind(person.id)
        .bind(person.handle.as_str())
        .bind(&person.payload)
        .bind(&person.search_vector)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<String>, sqlx::Error> {
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
}
