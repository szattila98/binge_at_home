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
        requests: Vec<Self::CreateRequest>,
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

#[macro_export]
macro_rules! build_find_all_query {
    ($table:literal, $ordering:ident, $pagination:ident) => {{
        let ordering_part = $ordering
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        let pagination_part = $pagination.map_or_else(String::new, |p| p.to_string());
        format!(
            "SELECT * FROM {}{}{}{}{}",
            $table,
            if ordering_part.is_empty() {
                ""
            } else {
                " ORDER BY "
            },
            ordering_part,
            if pagination_part.is_empty() { "" } else { " " },
            pagination_part
        )
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_find_all_query() {
        let ordering: Vec<OrderBy<i32>> = vec![];
        let pagination: Option<Pagination> = None;
        let query = build_find_all_query!("test_table1", ordering, pagination);
        assert_eq!(query, "SELECT * FROM test_table1");

        let ordering = vec![OrderBy(12, Direction::Asc)];
        let pagination: Option<Pagination> = None;
        let query = build_find_all_query!("test_table2", ordering, pagination);
        assert_eq!(query, "SELECT * FROM test_table2 ORDER BY 12 ASC");

        let ordering: Vec<OrderBy<i32>> = vec![];
        let pagination = Some(Pagination::new(10, 5));
        let query = build_find_all_query!("test_table3", ordering, pagination);
        assert_eq!(query, "SELECT * FROM test_table3 LIMIT 10 OFFSET 40");

        let ordering = vec![OrderBy(151, Direction::Asc)];
        let pagination = Some(Pagination::new(10, 1));
        let query = build_find_all_query!("test_table4", ordering, pagination);
        assert_eq!(
            query,
            "SELECT * FROM test_table4 ORDER BY 151 ASC LIMIT 10 OFFSET 0"
        );

        let ordering = vec![
            OrderBy(55, Direction::Asc),
            OrderBy(12, Direction::Desc),
            OrderBy(678, Direction::Asc),
        ];
        let pagination = Some(Pagination::new(5, 100));
        let query = build_find_all_query!("test_table5", ordering, pagination);
        assert_eq!(
            query,
            "SELECT * FROM test_table5 ORDER BY 55 ASC, 12 DESC, 678 ASC LIMIT 5 OFFSET 495"
        );
    }
}
