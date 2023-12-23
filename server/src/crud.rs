use std::fmt::{self, Debug, Display};

use async_trait::async_trait;
use serde::Deserialize;
use soa_derive::StructOfArray;
use sqlx::PgExecutor;

use crate::model::EntityId;

pub mod catalog;
pub mod metadata;
pub mod video;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Pagination {
    size: u64,
    page: u64,
}

impl Display for Pagination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let limit = self.size;
        let offset = self.size * (self.page - 1);
        write!(f, "LIMIT {limit} OFFSET {offset}")
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Sort<T>
where
    T: Debug + Display,
{
    field: T,
    direction: Direction,
}

impl<T> Display for Sort<T>
where
    T: Debug + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.field, self.direction)
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, strum::Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Direction {
    Asc,
    Desc,
}

#[async_trait]
pub trait Entity<T> {
    type CreateRequest: StructOfArray;
    type Ordering: Debug + Display;
    type UpdateRequest;

    async fn create<'a>(
        executor: impl PgExecutor<'a>,
        request: Self::CreateRequest,
    ) -> Result<T, sqlx::Error>;

    async fn create_many<'a>(
        executor: impl PgExecutor<'a>,
        requests: <Self::CreateRequest as StructOfArray>::Type,
    ) -> Result<Vec<T>, sqlx::Error>;

    async fn find<'a>(
        executor: impl PgExecutor<'a>,
        id: EntityId,
    ) -> Result<Option<T>, sqlx::Error>;

    async fn find_all<'a>(
        executor: impl PgExecutor<'a>,
        ordering: Vec<Sort<Self::Ordering>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<T>, sqlx::Error>;

    async fn update<'a>(
        executor: impl PgExecutor<'a>,
        request: Self::UpdateRequest,
    ) -> Result<Option<T>, sqlx::Error>;

    async fn delete<'a>(executor: impl PgExecutor<'a>, id: EntityId) -> Result<bool, sqlx::Error>;

    async fn delete_many<'a>(
        executor: impl PgExecutor<'a>,
        ids: Vec<EntityId>,
    ) -> Result<u64, sqlx::Error>;

    async fn count_all<'a>(executor: impl PgExecutor<'a>) -> Result<i64, sqlx::Error>;
}

fn build_find_all_query<T>(
    table_name: &'static str,
    ordering: &[Sort<T>],
    pagination: Option<Pagination>,
) -> String
where
    T: Debug + Display,
{
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
    fn display_pagination() {
        let size: u64 = Faker.fake::<u8>().into();
        let page: u64 = Faker.fake::<u8>().into();
        assert_eq!(
            Pagination { size, page }.to_string(),
            format!("LIMIT {} OFFSET {}", size, size * (page - 1))
        );
    }

    #[test]
    fn display_order_by() {
        let field: String = Faker.fake::<String>().to_uppercase();
        assert_eq!(
            Sort {
                field: &field,
                direction: Direction::Asc
            }
            .to_string(),
            format!("{field} {}", Direction::Asc)
        );
        assert_eq!(
            Sort {
                field: &field,
                direction: Direction::Desc
            }
            .to_string(),
            format!("{field} {}", Direction::Desc)
        );
    }

    #[test]
    fn display_direction_asc() {
        assert_eq!(Direction::Asc.to_string(), "ASC");
    }

    #[test]
    fn display_direction_desc() {
        assert_eq!(Direction::Desc.to_string(), "DESC");
    }

    #[test]
    fn build_find_all_query_empty_params() {
        let ordering: Vec<Sort<&str>> = vec![];
        let pagination: Option<Pagination> = None;
        let query = build_find_all_query("table", &ordering, pagination);
        assert_eq!(query, "SELECT * FROM table");
    }

    #[test]
    fn build_find_all_query_only_ordering() {
        let field: String = Faker.fake();
        let ordering = vec![Sort {
            field: field.clone(),
            direction: Direction::Asc,
        }];
        let pagination: Option<Pagination> = None;
        let query = build_find_all_query("table", &ordering, pagination);
        assert_eq!(query, format!("SELECT * FROM table ORDER BY {field} ASC"));
    }

    #[test]
    fn build_find_all_query_only_pagination() {
        let size: u64 = Faker.fake::<u8>().into();
        let page: u64 = Faker.fake::<u8>().into();
        let ordering: Vec<Sort<&str>> = vec![];
        let pagination = Some(Pagination { size, page });
        let query = build_find_all_query("table", &ordering, pagination);
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
        let ordering = vec![Sort {
            field: field.clone(),
            direction: Direction::Asc,
        }];
        let pagination = Some(Pagination { size, page });
        let query = build_find_all_query("table", &ordering, pagination);
        assert_eq!(
            query,
            format!(
                "SELECT * FROM table ORDER BY {field} ASC LIMIT {size} OFFSET {}",
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
            Sort {
                field: field1.clone(),
                direction: Direction::Asc,
            },
            Sort {
                field: field2.clone(),
                direction: Direction::Desc,
            },
            Sort {
                field: field3.clone(),
                direction: Direction::Asc,
            },
        ];
        let pagination = Some(Pagination { size, page });
        let query = build_find_all_query("table", &ordering, pagination);
        assert_eq!(
            query,
            format!(
                "SELECT * FROM table ORDER BY {field1} ASC, {field2} DESC, {field3} ASC LIMIT {size} OFFSET {}",
                size * (page - 1)
            )
        );
    }
}
