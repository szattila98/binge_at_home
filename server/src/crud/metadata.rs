use async_trait::async_trait;
#[cfg(test)]
use fake::Dummy;
use soa_derive::StructOfArray;
use sqlx::PgExecutor;
use tracing::{error, instrument};

use crate::model::{
    Bytes, BytesPerSecond, EntityId, FramesPerSecond, Metadata, ScreenHeight, ScreenWidth, Seconds,
};

use super::{build_find_all_query, Entity, Pagination, Sort};

#[derive(Debug, Clone, StructOfArray)]
#[soa_derive(Debug)]
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

    #[instrument(skip(executor))]
    async fn create<'a>(
        executor: impl PgExecutor<'a>,
        request: CreateMetadataRequest,
    ) -> Result<Self, sqlx::Error> {
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
        .fetch_one(executor)
        .await
        .map_err(|e| {
            error!("error while creating metadata: {e}");
            e
        })?;
        Ok(metadata)
    }

    #[instrument(skip(executor))]
    async fn create_many<'a>(
        executor: impl PgExecutor<'a>,
        requests: CreateMetadataRequestVec,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let metadatas = sqlx::query_as!(
            Self,
            r#"
                INSERT INTO metadata (size, duration, bitrate, width, height, framerate) 
                SELECT * FROM UNNEST($1::int8[], $2::float8[], $3::text[], $4::text[], $5::text[], $6::text[])
                RETURNING *
            "#,
            &requests.size[..],
            &requests.duration[..],
            &requests.bitrate[..],
            &requests.width[..],
            &requests.height[..],
            &requests.framerate[..],
        )
        .fetch_all(executor)
        .await
        .map_err(|e| {
            error!("error while creating metadatas: {e}");
            e
        })?;

        Ok(metadatas)
    }

    #[instrument(skip(executor))]
    async fn find<'a>(
        executor: impl PgExecutor<'a>,
        id: EntityId,
    ) -> Result<Option<Self>, sqlx::Error> {
        let metadata = sqlx::query_as!(Self, "SELECT * FROM metadata WHERE id = $1", id)
            .fetch_optional(executor)
            .await
            .map_err(|e| {
                error!("error while finding metadata: {e}");
                e
            })?;
        Ok(metadata)
    }

    #[instrument(skip(executor))]
    async fn find_all<'a>(
        executor: impl PgExecutor<'a>,
        ordering: Vec<Sort<()>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let query = build_find_all_query("metadata", &ordering, pagination);
        let metadatas = sqlx::query_as(&query)
            .fetch_all(executor)
            .await
            .map_err(|e| {
                error!("error while finding metadatas: {e}");
                e
            })?;
        Ok(metadatas)
    }

    #[instrument(skip(executor))]
    async fn update<'a>(
        executor: impl PgExecutor<'a>,
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
        .fetch_optional(executor)
        .await
        .map_err(|e| {
            error!("error while updating metadata: {e}");
            e
        })?;
        Ok(metadata)
    }

    #[instrument(skip(executor))]
    async fn delete<'a>(executor: impl PgExecutor<'a>, id: EntityId) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM metadata WHERE id = $1", id)
            .execute(executor)
            .await
            .map_err(|e| {
                error!("error while deleting metadata: {e}");
                e
            })?;
        Ok(result.rows_affected() == 1)
    }

    #[instrument(skip(executor))]
    async fn delete_many<'a>(
        executor: impl PgExecutor<'a>,
        ids: Vec<EntityId>,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM metadata WHERE id = ANY($1)", &ids[..])
            .execute(executor)
            .await
            .map_err(|e| {
                error!("error while deleting metadatas: {e}");
                e
            })?;
        Ok(result.rows_affected())
    }

    #[instrument(skip(executor))]
    async fn count_all<'a>(executor: impl PgExecutor<'a>) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM metadata"#)
            .fetch_one(executor)
            .await
            .map_err(|e| {
                error!("error while counting metadatas: {e}");
                e
            })?;
        Ok(count)
    }
}
