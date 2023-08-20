use std::fmt::{self, Debug};

use async_trait::async_trait;
use convert_case::{Case, Casing};
use sqlx::PgPool;

use crate::model::EntityId;

pub mod catalog;
pub mod video;

#[derive(Debug)]
pub struct Pagination {
    limit: u64,
    offset: u64,
}

impl Pagination {
    pub fn new(size: u64, page: u64) -> Self {
        Self {
            limit: size,
            offset: size * (page - 1),
        }
    }
}

impl fmt::Display for Pagination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LIMIT {} OFFSET {}", self.limit, self.offset)
    }
}

#[derive(Debug)]
pub struct OrderBy<T: fmt::Debug>(T, Direction);

impl<T: fmt::Debug> OrderBy<T> {
    pub fn new(field: T, direction: Direction) -> Self {
        Self(field, direction)
    }
}

impl<T: fmt::Debug> fmt::Display for OrderBy<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ordering = format!("{:?}", self.0).to_case(Case::Snake);
        write!(f, "{ordering} {}", self.1)
    }
}

#[derive(Debug)]
pub enum Direction {
    Asc,
    Desc,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = format!("{self:?}").to_uppercase();
        write!(f, "{formatted}")
    }
}

#[async_trait]
pub trait Entity<T> {
    type CreateRequest;
    type Ordering: Debug;
    type UpdateRequest;

    async fn create(pool: &PgPool, request: Self::CreateRequest) -> Result<T, sqlx::Error>;
    async fn create_many(
        pool: &PgPool,
        request: Vec<Self::CreateRequest>,
    ) -> Result<Vec<T>, sqlx::Error>;
    async fn find(pool: &PgPool, id: EntityId) -> Result<Option<T>, sqlx::Error>;
    async fn find_all(
        pool: &PgPool,
        ordering: Vec<OrderBy<Self::Ordering>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<T>, sqlx::Error>;
    async fn update(pool: &PgPool, request: Self::UpdateRequest) -> Result<Option<T>, sqlx::Error>;
    async fn delete(pool: &PgPool, id: EntityId) -> Result<bool, sqlx::Error>;
    async fn delete_many(pool: &PgPool, ids: Vec<EntityId>) -> Result<u64, sqlx::Error>;
}

/* fn save() {

    let record = sqlx::query!(
        r#"INSERT INTO catalog ( path ,display_name,short_desc, long_desc, created_at, updated_at ) VALUES ( $1, $2, $3, $4, $5, $6) RETURNING id"#,
        "/test",
        "display_name",
        "short_desc",
        "long_desc",
        time::OffsetDateTime::now_utc(),
        time::OffsetDateTime::now_utc()
    )
    .fetch_one(&database)
    .await?;
    println!("{}", record.id);

    let catalog = sqlx::query_as!(Catalog, r#"SELECT * FROM catalog WHERE id = $1"#, record.id)
        .fetch_one(&database)
        .await?;
    println!("{catalog:?}");

    let record = sqlx::query!(
        r#"INSERT INTO video ( path, display_name, short_desc, long_desc, catalog_id, sequent_id, size, duration, bitrate, width, height, framerate ) VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING id"#,
        "/test",
        "display_name",
        "short_desc",
        "long_desc",
        1,
        None as Option<ModelId>,
        100000,
        2400,
        120,
        1280,
        1024,
        60.
    )
    .fetch_one(&database)
    .await?;
    println!("{}", record.id);
} */
