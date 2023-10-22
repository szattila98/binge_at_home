use async_trait::async_trait;
#[cfg(test)]
use fake::Dummy;
use serde::Deserialize;
use sqlx::PgExecutor;
use tracing::{error, instrument};

use crate::model::{Catalog, EntityId};

use super::{build_find_all_query, Entity, Pagination, Sort};

#[derive(Debug, Default, Clone)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateCatalogRequest {
    path: String,
    display_name: String,
    short_desc: String,
    long_desc: String,
}

impl CreateCatalogRequest {
    pub fn new(path: String) -> Self {
        Self {
            path: path.clone(),
            display_name: path,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[cfg_attr(test, derive(Dummy))]
pub enum CatalogSort {
    DisplayName,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
pub struct UpdateCatalogRequest {
    id: EntityId,
    display_name: String,
    short_desc: String,
    long_desc: String,
}

#[async_trait]
impl Entity<Self> for Catalog {
    type CreateRequest = CreateCatalogRequest;
    type Ordering = CatalogSort;
    type UpdateRequest = UpdateCatalogRequest;

    #[instrument(skip(executor))]
    async fn create<'a>(
        executor: impl PgExecutor<'a>,
        request: CreateCatalogRequest,
    ) -> Result<Self, sqlx::Error> {
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
        .fetch_one(executor)
        .await
        .map_err(|e| {
            error!("error while creating catalog: {e}");
            e
        })?;
        Ok(catalog)
    }

    #[instrument(skip(executor))]
    async fn create_many<'a>(
        executor: impl PgExecutor<'a>,
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
        .fetch_all(executor)
        .await
        .map_err(|e| {
            error!("error while creating catalogs: {e}");
            e
        })?;

        Ok(catalogs)
    }

    #[instrument(skip(executor))]
    async fn find<'a>(
        executor: impl PgExecutor<'a>,
        id: EntityId,
    ) -> Result<Option<Self>, sqlx::Error> {
        let catalog = sqlx::query_as!(Self, "SELECT * FROM catalog WHERE id = $1", id)
            .fetch_optional(executor)
            .await
            .map_err(|e| {
                error!("error while finding catalog: {e}");
                e
            })?;
        Ok(catalog)
    }

    #[instrument(skip(executor))]
    async fn find_all<'a>(
        executor: impl PgExecutor<'a>,
        ordering: Vec<Sort<CatalogSort>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let query = build_find_all_query("catalog", &ordering, pagination);
        let catalogs = sqlx::query_as(&query)
            .fetch_all(executor)
            .await
            .map_err(|e| {
                error!("error while finding catalogs: {e}");
                e
            })?;
        Ok(catalogs)
    }

    #[instrument(skip(executor))]
    async fn update<'a>(
        executor: impl PgExecutor<'a>,
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
        .fetch_optional(executor)
        .await.map_err(|e| {
            error!("error while updating catalog: {e}");
            e
        })?;
        Ok(catalog)
    }

    #[instrument(skip(executor))]
    async fn delete<'a>(executor: impl PgExecutor<'a>, id: EntityId) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM catalog WHERE id = $1", id)
            .execute(executor)
            .await
            .map_err(|e| {
                error!("error while deleting catalog: {e}");
                e
            })?;
        Ok(result.rows_affected() == 1)
    }

    #[instrument(skip(executor))]
    async fn delete_many<'a>(
        executor: impl PgExecutor<'a>,
        ids: Vec<EntityId>,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM catalog WHERE id = ANY($1)", &ids[..])
            .execute(executor)
            .await
            .map_err(|e| {
                error!("error while deleting catalogs: {e}");
                e
            })?;
        Ok(result.rows_affected())
    }

    #[instrument(skip(executor))]
    async fn count_all<'a>(executor: impl PgExecutor<'a>) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM catalog"#)
            .fetch_one(executor)
            .await
            .map_err(|e| {
                error!("error while counting catalogs: {e}");
                e
            })?;
        Ok(count)
    }
}

impl Catalog {
    #[instrument(skip(executor))]
    pub async fn find_by_path<'a>(
        executor: impl PgExecutor<'a>,
        path: &str,
    ) -> Result<Option<Catalog>, sqlx::Error> {
        let catalog = sqlx::query_as!(Self, "SELECT * FROM catalog WHERE path = $1", path)
            .fetch_optional(executor)
            .await
            .map_err(|e| {
                error!("error while finding catalog by path: {e}");
                e
            })?;
        Ok(catalog)
    }
}

/* #[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};
    use pretty_assertions::assert_eq;

    #[sqlx::test]
    async fn create_one_catalog_created(executor: PgPool) -> Result<(), sqlx::Error> {
        let request: CreateCatalogRequest = Faker.fake();

        let catalog = Catalog::create(&executor, request.clone()).await?;
        let count = Catalog::count_all(&executor).await?;

        assert_eq!(count, 1);
        assert_eq!(catalog.id, 1);
        assert_eq!(catalog.path, request.path);
        assert_eq!(catalog.display_name, request.display_name);
        assert_eq!(catalog.short_desc, request.short_desc);
        assert_eq!(catalog.long_desc, request.long_desc);

        Ok(())
    }

    #[sqlx::test]
    async fn create_many_three_catalogs_created(executor: PgPool) -> Result<(), sqlx::Error> {
        let request1: CreateCatalogRequest = Faker.fake();
        let request2: CreateCatalogRequest = Faker.fake();
        let request3: CreateCatalogRequest = Faker.fake();

        let requests = vec![request1.clone(), request2.clone(), request3.clone()];

        let catalogs = Catalog::create_many(&executor, requests).await?;
        let count = Catalog::count_all(&executor).await?;

        assert_eq!(count, 3);

        assert_eq!(catalogs[0].path, request1.path);
        assert_eq!(catalogs[0].display_name, request1.display_name);
        assert_eq!(catalogs[0].short_desc, request1.short_desc);
        assert_eq!(catalogs[0].long_desc, request1.long_desc);

        assert_eq!(catalogs[1].path, request2.path);
        assert_eq!(catalogs[1].display_name, request2.display_name);
        assert_eq!(catalogs[1].short_desc, request2.short_desc);
        assert_eq!(catalogs[1].long_desc, request2.long_desc);

        assert_eq!(catalogs[2].path, request3.path);
        assert_eq!(catalogs[2].display_name, request3.display_name);
        assert_eq!(catalogs[2].short_desc, request3.short_desc);
        assert_eq!(catalogs[2].long_desc, request3.long_desc);

        Ok(())
    }

    #[sqlx::test(fixtures("catalogs"))]
    async fn find_correct_id_found(executor: PgPool) -> Result<(), sqlx::Error> {
        let catalog = Catalog::find(&executor, 3).await?.expect("no catalog found");

        assert_eq!(catalog.id, 3);
        assert_eq!(catalog.path, "/movies/2");
        assert_eq!(catalog.display_name, "Inception");
        assert_eq!(catalog.short_desc, "Science Fiction movie");
        assert_eq!(
            catalog.long_desc,
            "A thief enters the dreams of others to steal their secrets."
        );

        Ok(())
    }
} */
