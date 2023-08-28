use async_trait::async_trait;
use sqlx::PgPool;
use tracing::instrument;

use crate::model::{Catalog, EntityId};

use super::{Entity, OrderBy, Pagination};

#[derive(Debug)]
pub struct CreateCatalogRequest {
    path: String,
    display_name: String,
    short_desc: String,
    long_desc: String,
}

#[derive(Debug)]
pub enum CatalogOrdering {
    Path,
    DisplayName,
    ShortDesc,
    LongDesc,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug)]
pub struct UpdateCatalogRequest {
    id: EntityId,
    display_name: String,
    short_desc: String,
    long_desc: String,
}

#[async_trait]
impl Entity<Self> for Catalog {
    type CreateRequest = CreateCatalogRequest;
    type Ordering = CatalogOrdering;
    type UpdateRequest = UpdateCatalogRequest;

    #[instrument(skip(pool))]
    async fn create(pool: &PgPool, request: CreateCatalogRequest) -> Result<Self, sqlx::Error> {
        let catalog = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO catalog ( path, display_name, short_desc, long_desc ) 
            VALUES ( $1, $2, $3, $4) 
            RETURNING *
        "#,
            request.path,
            request.display_name,
            request.short_desc,
            request.long_desc
        )
        .fetch_one(pool)
        .await?;
        Ok(catalog)
    }

    #[instrument(skip(pool))]
    async fn create_many(
        pool: &PgPool,
        requests: Vec<CreateCatalogRequest>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let mut paths = vec![];
        let mut display_names = vec![];
        let mut short_descs = vec![];
        let mut long_descs = vec![];

        for item in requests {
            paths.push(item.path);
            display_names.push(item.display_name);
            short_descs.push(item.short_desc);
            long_descs.push(item.long_desc);
        }

        let catalogs = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO catalog ( path, display_name, short_desc, long_desc ) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[])
            RETURNING *
        "#,
            &paths[..],
            &display_names[..],
            &short_descs[..],
            &long_descs[..]
        )
        .fetch_all(pool)
        .await?;

        Ok(catalogs)
    }

    #[instrument(skip(pool))]
    async fn find(pool: &PgPool, id: EntityId) -> Result<Option<Self>, sqlx::Error> {
        let catalog = sqlx::query_as!(Self, " SELECT * FROM catalog WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(catalog)
    }

    #[instrument(skip(pool))]
    async fn find_all(
        pool: &PgPool,
        ordering: Vec<OrderBy<CatalogOrdering>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let ordering_part = ordering
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        let pagination_part = pagination.map_or_else(String::new, |p| p.to_string());
        let query = format!(
            "SELECT * FROM catalog{}{ordering_part}{}{pagination_part}",
            if ordering_part.is_empty() {
                ""
            } else {
                " ORDER BY "
            },
            if pagination_part.is_empty() { "" } else { " " }
        );

        let catalogs = sqlx::query_as(&query).fetch_all(pool).await?;

        Ok(catalogs)
    }

    #[instrument(skip(pool))]
    async fn update(
        pool: &PgPool,
        request: UpdateCatalogRequest,
    ) -> Result<Option<Self>, sqlx::Error> {
        let catalog = sqlx::query_as!(
        Self,
        "UPDATE catalog SET display_name = $1, short_desc = $2, long_desc = $3 WHERE id = $4 RETURNING *",
        request.display_name,
        request.short_desc,
        request.long_desc,
        request.id
    )
    .fetch_optional(pool)
    .await?;
        Ok(catalog)
    }

    #[instrument(skip(pool))]
    async fn delete(pool: &PgPool, id: EntityId) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM catalog WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() == 1)
    }

    #[instrument(skip(pool))]
    async fn delete_many(pool: &PgPool, ids: Vec<EntityId>) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM catalog WHERE id = ANY($1)", &ids[..])
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
