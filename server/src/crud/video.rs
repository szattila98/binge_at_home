use async_trait::async_trait;
#[cfg(test)]
use fake::Dummy;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{error, instrument};

use crate::model::{
    Bytes, BytesPerSecond, EntityId, FramesPerSecond, ScreenHeight, ScreenWidth, Seconds, Video,
};

use super::{build_find_all_query, Entity, Pagination, Sort};

#[derive(Debug)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateVideoRequest {
    pub path: String,
    pub display_name: String,
    pub short_desc: String,
    pub long_desc: String,
    pub catalog_id: EntityId,
    pub sequent_id: Option<EntityId>,
    pub metadata_id: EntityId,

    pub size: Bytes,
    pub duration: Seconds,
    pub bitrate: BytesPerSecond,
    pub width: ScreenWidth,
    pub height: ScreenHeight,
    pub framerate: FramesPerSecond,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
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
    pub metadata_id: Option<EntityId>,

    pub size: Bytes,
    pub duration: Seconds,
    pub bitrate: BytesPerSecond,
    pub width: ScreenWidth,
    pub height: ScreenHeight,
    pub framerate: FramesPerSecond,
}

#[async_trait]
impl Entity<Self> for Video {
    type CreateRequest = CreateVideoRequest;
    type Ordering = VideoSort;
    type UpdateRequest = UpdateVideoRequest;

    #[instrument(skip(pool))]
    async fn create(pool: &PgPool, request: CreateVideoRequest) -> Result<Self, sqlx::Error> {
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
        .fetch_one(pool)
        .await
        .map_err(|e| {
            error!("error while creating video: {e}");
            e
        })?;
        Ok(video)
    }

    #[instrument(skip(pool))]
    async fn create_many(
        pool: &PgPool,
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
        .fetch_all(pool)
        .await
        .map_err(|e| {
            error!("error while creating videos: {e}");
            e
        })?;

        Ok(videos)
    }

    #[instrument(skip(pool))]
    async fn find(pool: &PgPool, id: EntityId) -> Result<Option<Self>, sqlx::Error> {
        let video = sqlx::query_as!(Self, "SELECT * FROM video WHERE id = $1", id)
            .fetch_optional(pool)
            .await
            .map_err(|e| {
                error!("error while finding video: {e}");
                e
            })?;
        Ok(video)
    }

    #[instrument(skip(pool))]
    async fn find_all(
        pool: &PgPool,
        ordering: Vec<Sort<VideoSort>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let query = build_find_all_query("video", ordering, pagination);
        let videos = sqlx::query_as(&query).fetch_all(pool).await.map_err(|e| {
            error!("error while finding videos: {e}");
            e
        })?;
        Ok(videos)
    }

    #[instrument(skip(pool))]
    async fn update(
        pool: &PgPool,
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
        .fetch_optional(pool)
        .await.map_err(|e| {
            error!("error while updating video: {e}");
            e
        })?;
        Ok(video)
    }

    #[instrument(skip(pool))]
    async fn delete(pool: &PgPool, id: EntityId) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM video WHERE id = $1", id)
            .execute(pool)
            .await
            .map_err(|e| {
                error!("error while deleting video: {e}");
                e
            })?;
        Ok(result.rows_affected() == 1)
    }

    #[instrument(skip(pool))]
    async fn delete_many(pool: &PgPool, ids: Vec<EntityId>) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM video WHERE id = ANY($1)", &ids[..])
            .execute(pool)
            .await
            .map_err(|e| {
                error!("error while deleting videos: {e}");
                e
            })?;
        Ok(result.rows_affected())
    }

    #[instrument(skip(pool))]
    async fn count_all(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM video"#)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                error!("error while counting videos: {e}");
                e
            })?;
        Ok(count)
    }
}

impl Video {
    pub async fn find_by_catalog_id(
        pool: &PgPool,
        catalog_id: EntityId,
    ) -> Result<Vec<Video>, sqlx::Error> {
        let video = sqlx::query_as!(
            Self,
            "SELECT * FROM video WHERE catalog_id = $1",
            catalog_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            error!("error while finding video by catalog id: {e}");
            e
        })?;
        Ok(video)
    }
}
