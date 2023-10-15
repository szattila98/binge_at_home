use async_trait::async_trait;
#[cfg(test)]
use fake::Dummy;
use serde::Deserialize;
use sqlx::PgExecutor;
use tracing::{error, instrument};

use crate::model::{EntityId, Video};

use super::{build_find_all_query, Entity, Pagination, Sort};

#[derive(Debug, Default)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateVideoRequest {
    pub path: String,
    pub display_name: String,
    pub short_desc: String,
    pub long_desc: String,
    pub catalog_id: EntityId,
    pub sequent_id: Option<EntityId>,
    pub metadata_id: EntityId,
}

impl CreateVideoRequest {
    pub fn new(path: String, catalog_id: EntityId, metadata_id: EntityId) -> Self {
        Self {
            path: path.clone(),
            display_name: path,
            catalog_id,
            metadata_id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[cfg_attr(test, derive(Dummy))]
pub enum VideoSort {
    DisplayName,
    Size,
    Duration,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Dummy))]
pub struct UpdateVideoRequest {
    pub id: EntityId,
    pub path: String,
    pub display_name: String,
    pub short_desc: String,
    pub long_desc: String,
    pub catalog_id: EntityId,
    pub sequent_id: Option<EntityId>,
    pub metadata_id: EntityId,
}

#[async_trait]
impl Entity<Self> for Video {
    type CreateRequest = CreateVideoRequest;
    type Ordering = VideoSort;
    type UpdateRequest = UpdateVideoRequest;

    #[instrument(skip(executor))]
    async fn create<'a>(
        executor: impl PgExecutor<'a>,
        request: CreateVideoRequest,
    ) -> Result<Self, sqlx::Error> {
        let video = sqlx::query_as!(
            Self,
            r#"
                INSERT INTO video ( 
                    path, display_name, short_desc, long_desc, catalog_id, sequent_id, metadata_id
                ) 
                VALUES ( $1, $2, $3, $4, $5, $6, $7 ) 
                RETURNING *
            "#,
            request.path,
            request.display_name,
            request.short_desc,
            request.long_desc,
            request.catalog_id,
            request.sequent_id,
            request.metadata_id
        )
        .fetch_one(executor)
        .await
        .map_err(|e| {
            error!("error while creating video: {e}");
            e
        })?;
        Ok(video)
    }

    #[instrument(skip(executor))]
    async fn create_many<'a>(
        executor: impl PgExecutor<'a>,
        requests: Vec<CreateVideoRequest>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let mut paths = vec![];
        let mut display_names = vec![];
        let mut short_descs = vec![];
        let mut long_descs = vec![];
        let mut catalog_ids = vec![];
        let mut metadata_ids = vec![];

        for item in requests {
            paths.push(item.path);
            display_names.push(item.display_name.clone());
            short_descs.push(item.short_desc.clone());
            long_descs.push(item.long_desc.clone());
            catalog_ids.push(item.catalog_id);
            metadata_ids.push(item.metadata_id);
        }

        let videos = sqlx::query_as!(
            Self,
            r#"
                INSERT INTO video ( 
                    path, display_name, short_desc, long_desc, catalog_id, metadata_id
                ) 
                SELECT * FROM UNNEST(
                    $1::text[], $2::text[], $3::text[], $4::text[], $5::int8[], $6::int8[]
                )
                RETURNING *
            "#,
            &paths[..],
            &display_names[..],
            &short_descs[..],
            &long_descs[..],
            &catalog_ids[..],
            &metadata_ids[..]
        )
        .fetch_all(executor)
        .await
        .map_err(|e| {
            error!("error while creating videos: {e}");
            e
        })?;

        Ok(videos)
    }

    #[instrument(skip(executor))]
    async fn find<'a>(
        executor: impl PgExecutor<'a>,
        id: EntityId,
    ) -> Result<Option<Self>, sqlx::Error> {
        let video = sqlx::query_as!(Self, "SELECT * FROM video WHERE id = $1", id)
            .fetch_optional(executor)
            .await
            .map_err(|e| {
                error!("error while finding video: {e}");
                e
            })?;
        Ok(video)
    }

    #[instrument(skip(executor))]
    async fn find_all<'a>(
        executor: impl PgExecutor<'a>,
        ordering: Vec<Sort<VideoSort>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let query = build_find_all_query("video", &ordering, pagination);
        let videos = sqlx::query_as(&query)
            .fetch_all(executor)
            .await
            .map_err(|e| {
                error!("error while finding videos: {e}");
                e
            })?;
        Ok(videos)
    }

    #[instrument(skip(executor))]
    async fn update<'a>(
        executor: impl PgExecutor<'a>,
        request: UpdateVideoRequest,
    ) -> Result<Option<Self>, sqlx::Error> {
        let video = sqlx::query_as!(
            Self,
            r#"
                UPDATE video SET 
                    display_name = $1, short_desc = $2, long_desc = $3, catalog_id = $4, sequent_id = $5, metadata_id = $6
                WHERE id = $7
                RETURNING *
            "#,
            request.display_name,
            request.short_desc,
            request.long_desc,
            request.catalog_id,
            request.sequent_id,
            request.metadata_id,
            request.id
        )
        .fetch_optional(executor)
        .await.map_err(|e| {
            error!("error while updating video: {e}");
            e
        })?;
        Ok(video)
    }

    #[instrument(skip(executor))]
    async fn delete<'a>(executor: impl PgExecutor<'a>, id: EntityId) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM video WHERE id = $1", id)
            .execute(executor)
            .await
            .map_err(|e| {
                error!("error while deleting video: {e}");
                e
            })?;
        Ok(result.rows_affected() == 1)
    }

    #[instrument(skip(executor))]
    async fn delete_many<'a>(
        executor: impl PgExecutor<'a>,
        ids: Vec<EntityId>,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM video WHERE id = ANY($1)", &ids[..])
            .execute(executor)
            .await
            .map_err(|e| {
                error!("error while deleting videos: {e}");
                e
            })?;
        Ok(result.rows_affected())
    }

    #[instrument(skip(executor))]
    async fn count_all<'a>(executor: impl PgExecutor<'a>) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM video"#)
            .fetch_one(executor)
            .await
            .map_err(|e| {
                error!("error while counting videos: {e}");
                e
            })?;
        Ok(count)
    }
}

impl Video {
    pub async fn find_by_catalog_id<'a>(
        executor: impl PgExecutor<'a>,
        catalog_id: EntityId,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let video = sqlx::query_as!(
            Self,
            "SELECT * FROM video WHERE catalog_id = $1",
            catalog_id
        )
        .fetch_all(executor)
        .await
        .map_err(|e| {
            error!("error while finding video by catalog id: {e}");
            e
        })?;
        Ok(video)
    }
}
