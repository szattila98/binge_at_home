use std::fmt::{self, Debug};

use async_trait::async_trait;
use convert_case::{Case, Casing};
use sqlx::PgPool;

use crate::model::EntityId;

pub mod catalog;
pub mod video;

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct OrderBy<T: fmt::Debug>(T, Direction);

impl<T: fmt::Debug> OrderBy<T> {
    pub fn new(field: T, direction: Direction) -> Self {
        Self(field, direction)
    }
}

impl<T: fmt::Debug> fmt::Display for OrderBy<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ordering = format!("{:?}", self.0).to_case(Case::Snake);
        let direction = &self.1;
        write!(f, "{ordering} {direction}")
    }
}

#[derive(Debug, PartialEq, Eq)]
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

fn build_find_all_query<T: fmt::Debug>(
    table_name: &'static str,
    ordering: Vec<OrderBy<T>>,
    pagination: Option<Pagination>,
) -> String {
    let ordering_part = ordering
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    let pagination_part = pagination.map_or_else(String::new, |p| p.to_string());
    format!(
        "SELECT * FROM {}{}{}{}{}",
        table_name,
        if ordering_part.is_empty() {
            ""
        } else {
            " ORDER BY "
        },
        ordering_part,
        if pagination_part.is_empty() { "" } else { " " },
        pagination_part
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};
    use pretty_assertions::assert_eq;

    #[test]
    fn new_pagination() {
        let size: u64 = Faker.fake::<u8>().into();
        let page: u64 = Faker.fake::<u8>().into();
        assert_eq!(
            Pagination::new(size, page),
            Pagination {
                limit: size,
                offset: size * (page - 1)
            }
        );
    }

    #[test]
    fn display_pagination() {
        let size: u64 = Faker.fake::<u8>().into();
        let page: u64 = Faker.fake::<u8>().into();
        assert_eq!(
            Pagination::new(size, page).to_string(),
            format!("LIMIT {} OFFSET {}", size, size * (page - 1))
        );
    }

    #[test]
    fn new_order_by() {
        let field: String = Faker.fake();
        assert_eq!(
            OrderBy::new(&field, Direction::Asc),
            OrderBy(&field, Direction::Asc)
        );
        assert_eq!(
            OrderBy::new(&field, Direction::Desc),
            OrderBy(&field, Direction::Desc)
        );
    }

    #[test]
    fn display_order_by() {
        let field: String = Faker.fake();
        let snake_field = field.to_case(Case::Snake);
        assert_eq!(
            OrderBy::new(&field, Direction::Asc).to_string(),
            format!("{snake_field:?} {}", Direction::Asc)
        );
        assert_eq!(
            OrderBy::new(&field, Direction::Desc).to_string(),
            format!("{snake_field:?} {}", Direction::Desc)
        );
    }

    #[test]
    fn display_direction_asc() {
        assert_eq!(Direction::Asc.to_string(), "ASC")
    }

    #[test]
    fn display_direction_desc() {
        assert_eq!(Direction::Desc.to_string(), "DESC")
    }

    #[test]
    fn build_find_all_query_empty_params() {
        let ordering: Vec<OrderBy<&str>> = vec![];
        let pagination: Option<Pagination> = None;
        let query = build_find_all_query("table", ordering, pagination);
        assert_eq!(query, "SELECT * FROM table");
    }

    #[test]
    fn build_find_all_query_only_ordering() {
        let field: String = Faker.fake();
        let ordering = vec![OrderBy(field.clone(), Direction::Asc)];
        let pagination: Option<Pagination> = None;
        let query = build_find_all_query("table", ordering, pagination);
        assert_eq!(
            query,
            format!(
                "SELECT * FROM table ORDER BY {:?} ASC",
                field.to_case(Case::Snake)
            )
        );
    }

    #[test]
    fn build_find_all_query_only_pagination() {
        let size: u64 = Faker.fake::<u8>().into();
        let page: u64 = Faker.fake::<u8>().into();
        let ordering: Vec<OrderBy<&str>> = vec![];
        let pagination = Some(Pagination::new(size, page));
        let query = build_find_all_query("table", ordering, pagination);
        assert_eq!(
            query,
            format!(
                "SELECT * FROM table LIMIT {size:?} OFFSET {}",
                size * (page - 1)
            )
        );
    }

    #[test]
    fn build_find_all_query_both_params() {
        let field: String = Faker.fake();
        let size: u64 = Faker.fake::<u8>().into();
        let page: u64 = Faker.fake::<u8>().into();
        let ordering = vec![OrderBy(field.clone(), Direction::Asc)];
        let pagination = Some(Pagination::new(size, page));
        let query = build_find_all_query("table", ordering, pagination);
        assert_eq!(
            query,
            format!(
                "SELECT * FROM table ORDER BY {:?} ASC LIMIT {size} OFFSET {}",
                field.to_case(Case::Snake),
                size * (page - 1)
            )
        );
    }

    #[test]
    fn build_find_all_query_multiple_ordering_params_with_pagination() {
        let field1: String = Faker.fake();
        let field2: String = Faker.fake();
        let field3: String = Faker.fake();
        let size: u64 = Faker.fake::<u8>().into();
        let page: u64 = Faker.fake::<u8>().into();
        let ordering = vec![
            OrderBy(field1.clone(), Direction::Asc),
            OrderBy(field2.clone(), Direction::Desc),
            OrderBy(field3.clone(), Direction::Asc),
        ];
        let pagination = Some(Pagination::new(size, page));
        let query = build_find_all_query("table", ordering, pagination);
        assert_eq!(
            query,
            format!(
                "SELECT * FROM table ORDER BY {:?} ASC, {:?} DESC, {:?} ASC LIMIT {size} OFFSET {}",
                field1.to_case(Case::Snake),
                field2.to_case(Case::Snake),
                field3.to_case(Case::Snake),
                size * (page - 1)
            )
        );
    }
}
