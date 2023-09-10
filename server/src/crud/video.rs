use async_trait::async_trait;
#[cfg(test)]
use fake::Dummy;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::instrument;

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
    Path,
    DisplayName,
    ShortDesc,
    LongDesc,
    CatalogId,
    SequentId,
    Size,
    Duration,
    Bitrate,
    Width,
    Height,
    Framerate,
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
                    path, display_name, short_desc, long_desc, catalog_id, sequent_id, 
                    size, duration, bitrate, width, height, framerate 
                ) 
                VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12 ) 
                RETURNING *
            "#,
            request.path,
            request.display_name,
            request.short_desc,
            request.long_desc,
            request.catalog_id,
            request.sequent_id,
            request.size,
            request.duration,
            request.bitrate,
            request.width,
            request.height,
            request.framerate
        )
        .fetch_one(pool)
        .await?;
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
        let mut sizes = vec![];
        let mut durations = vec![];
        let mut bitrates = vec![];
        let mut widths = vec![];
        let mut heights = vec![];
        let mut framerates = vec![];

        for item in requests {
            paths.push(item.path);
            display_names.push(item.display_name.clone());
            short_descs.push(item.short_desc.clone());
            long_descs.push(item.long_desc.clone());
            catalog_ids.push(item.catalog_id);
            sizes.push(item.size);
            durations.push(item.duration);
            bitrates.push(item.bitrate);
            widths.push(item.width);
            heights.push(item.height);
            framerates.push(item.framerate);
        }

        let videos = sqlx::query_as!(
            Self,
            r#"
                INSERT INTO video ( 
                    path, display_name, short_desc, long_desc, catalog_id, size, 
                    duration, bitrate, width, height, framerate 
                ) 
                SELECT * FROM UNNEST(
                    $1::text[], $2::text[], $3::text[], $4::text[], $5::int8[], $6::int8[], 
                    $7::int8[], $8::int8[], $9::int2[], $10::int2[], $11::float8[]
                )
                RETURNING *
            "#,
            &paths[..],
            &display_names[..],
            &short_descs[..],
            &long_descs[..],
            &catalog_ids[..],
            &sizes[..],
            &durations[..],
            &bitrates[..],
            &widths[..],
            &heights[..],
            &framerates[..]
        )
        .fetch_all(pool)
        .await?;

        Ok(videos)
    }

    #[instrument(skip(pool))]
    async fn find(pool: &PgPool, id: EntityId) -> Result<Option<Self>, sqlx::Error> {
        let video = sqlx::query_as!(Self, "SELECT * FROM video WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(video)
    }

    #[instrument(skip(pool))]
    async fn find_all(
        pool: &PgPool,
        ordering: Vec<Sort<VideoSort>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let query = build_find_all_query("video", ordering, pagination);

        let videos = sqlx::query_as(&query).fetch_all(pool).await?;

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
                display_name = $1, short_desc = $2, long_desc = $3, catalog_id = $4, sequent_id = $5, 
                size = $6, duration = $7, bitrate = $8, width = $9, height = $10, framerate = $11
            WHERE id = $12
            RETURNING *
        "#,
        request.display_name,
        request.short_desc,
        request.long_desc,
        request.catalog_id,
        request.sequent_id,
        request.size,
        request.duration,
        request.bitrate,
        request.width,
        request.height,
        request.framerate,
        request.id
    )
    .fetch_optional(pool)
    .await?;
        Ok(video)
    }

    #[instrument(skip(pool))]
    async fn delete(pool: &PgPool, id: EntityId) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM video WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() == 1)
    }

    #[instrument(skip(pool))]
    async fn delete_many(pool: &PgPool, ids: Vec<EntityId>) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM video WHERE id = ANY($1)", &ids[..])
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    #[instrument(skip(pool))]
    async fn count_all(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM video"#)
            .fetch_one(pool)
            .await?;
        Ok(count)
    }
}
