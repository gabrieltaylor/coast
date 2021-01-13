use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgDone, PgPool};
use sqlx::types::Uuid;
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub inserted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct NewUserChangeset {
    #[validate(length(min = 1, max = 255))]
    name: String,
}

impl User {
    pub async fn find(id: Uuid, pool: &PgPool) -> Result<User, sqlx::Error> {
        sqlx::query_as!(Self, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(pool)
            .await
    }

    // pub async fn find_with_ids(db: &PgPool, ids: Vec<Uuid>) -> Result<Vec<User>, sqlx::Error> {
    //     sqlx::query_as!(Self, "SELECT * FROM users WHERE id = ANY(ARRAY $1)", ids)
    //         .fetch_all(db)
    //         .await
    // }

    // pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    // }

    // pub async fn find_each(pool: &PgPool) -> Result<Vec<user>, sqlx::Error> {
    //
    // }

    // pub async fn find_in_batches(pool: &PgPool) -> Result<Vec<user>, sqlx::Error> {
    //
    // }

    pub async fn all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(Self, "SELECT * FROM users")
            .fetch_all(pool)
            .await
    }

    pub async fn delete(id: Uuid, pool: &PgPool) -> Result<PgDone, sqlx::Error> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(pool)
            .await
    }

    pub async fn create(cs: NewUserChangeset, pool: &PgPool) -> Result<User, sqlx::Error> {
        let now = Utc::now();
        sqlx::query_as!(
            Self,
            "INSERT INTO users (id, name, inserted_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING *",
            Uuid::new_v4(),
            cs.name,
            now,
            now
        ).fetch_one(pool).await
    }
}
