use async_trait::async_trait;
#[cfg(test)]
use fake::Dummy;
use sqlx::PgPool;
use tracing::{error, instrument};

use crate::model::{
    Bytes, BytesPerSecond, EntityId, FramesPerSecond, Metadata, ScreenHeight, ScreenWidth, Seconds,
};

use super::{build_find_all_query, Entity, Pagination, Sort};

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateMetadataRequest {
    pub size: Bytes,
    pub duration: Seconds,
    pub bitrate: BytesPerSecond,
    pub width: ScreenWidth,
    pub height: ScreenHeight,
    pub framerate: FramesPerSecond,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
pub struct UpdateMetadataRequest {
    pub id: EntityId,
    pub size: Bytes,
    pub duration: Seconds,
    pub bitrate: BytesPerSecond,
    pub width: ScreenWidth,
    pub height: ScreenHeight,
    pub framerate: FramesPerSecond,
}

#[async_trait]
impl Entity<Self> for Metadata {
    type CreateRequest = CreateMetadataRequest;
    type Ordering = ();
    type UpdateRequest = UpdateMetadataRequest;

    #[instrument(skip(pool))]
    async fn create(pool: &PgPool, request: CreateMetadataRequest) -> Result<Self, sqlx::Error> {
        let metadata = sqlx::query_as!(
            Self,
            r#"
                INSERT INTO metadata (size, duration, bitrate, width, height, framerate) 
                VALUES ($1, $2, $3, $4, $5, $6) 
                RETURNING *
            "#,
            request.size,
            request.duration,
            request.bitrate,
            request.width,
            request.height,
            request.framerate
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            error!("error while creating metadata: {e}");
            e
        })?;
        Ok(metadata)
    }

    #[instrument(skip(pool))]
    async fn create_many(
        pool: &PgPool,
        requests: Vec<CreateMetadataRequest>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let mut sizes = vec![];
        let mut durations = vec![];
        let mut bitrates = vec![];
        let mut widths = vec![];
        let mut heights = vec![];
        let mut framerates = vec![];

        for item in requests {
            sizes.push(item.size);
            durations.push(item.duration);
            bitrates.push(item.bitrate);
            widths.push(item.width);
            heights.push(item.height);
            framerates.push(item.framerate);
        }

        let metadatas = sqlx::query_as!(
            Self,
            r#"
                INSERT INTO metadata (size, duration, bitrate, width, height, framerate) 
                SELECT * FROM UNNEST($1::int8[], $2::int8[], $3::text[], $4::text[], $5::text[], $6::text[])
                RETURNING *
            "#,
            &sizes[..],
            &durations[..],
            &bitrates[..],
            &widths[..],
            &heights[..],
            &framerates[..],
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            error!("error while creating metadatas: {e}");
            e
        })?;

        Ok(metadatas)
    }

    #[instrument(skip(pool))]
    async fn find(pool: &PgPool, id: EntityId) -> Result<Option<Self>, sqlx::Error> {
        let metadata = sqlx::query_as!(Self, "SELECT * FROM metadata WHERE id = $1", id)
            .fetch_optional(pool)
            .await
            .map_err(|e| {
                error!("error while finding metadata: {e}");
                e
            })?;
        Ok(metadata)
    }

    #[instrument(skip(pool))]
    async fn find_all(
        pool: &PgPool,
        ordering: Vec<Sort<()>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let query = build_find_all_query("metadata", ordering, pagination);
        let metadatas = sqlx::query_as(&query).fetch_all(pool).await.map_err(|e| {
            error!("error while finding metadatas: {e}");
            e
        })?;
        Ok(metadatas)
    }

    #[instrument(skip(pool))]
    async fn update(
        pool: &PgPool,
        request: UpdateMetadataRequest,
    ) -> Result<Option<Self>, sqlx::Error> {
        let metadata = sqlx::query_as!(
            Self,
            r#"
                UPDATE metadata 
                SET size = $1, duration = $2, bitrate = $3, width = $4, height = $5, framerate = $6 
                WHERE id = $7 
                RETURNING *
            "#,
            request.size,
            request.duration,
            request.width,
            request.height,
            request.height,
            request.framerate,
            request.id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            error!("error while updating metadata: {e}");
            e
        })?;
        Ok(metadata)
    }

    #[instrument(skip(pool))]
    async fn delete(pool: &PgPool, id: EntityId) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM metadata WHERE id = $1", id)
            .execute(pool)
            .await
            .map_err(|e| {
                error!("error while deleting metadata: {e}");
                e
            })?;
        Ok(result.rows_affected() == 1)
    }

    #[instrument(skip(pool))]
    async fn delete_many(pool: &PgPool, ids: Vec<EntityId>) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM metadata WHERE id = ANY($1)", &ids[..])
            .execute(pool)
            .await
            .map_err(|e| {
                error!("error while deleting metadatas: {e}");
                e
            })?;
        Ok(result.rows_affected())
    }

    #[instrument(skip(pool))]
    async fn count_all(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM metadata"#)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                error!("error while counting metadatas: {e}");
                e
            })?;
        Ok(count)
    }
}
