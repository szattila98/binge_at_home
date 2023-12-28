use async_trait::async_trait;
#[cfg(test)]
use fake::Dummy;
use serde::Deserialize;
use soa_derive::StructOfArray;
use sqlx::PgExecutor;
use tap::TapFallible;
use tracing::{error, instrument};

use crate::{
    elastic::Indexable,
    model::{EntityId, Video},
};

use super::{build_find_all_query, Entity, Pagination, Sort};

#[derive(Debug, Default, StructOfArray)]
#[soa_derive(Debug)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateVideoRequest {
    pub path: String,
    pub display_name: String,
    pub short_desc: String,
    pub long_desc: String,
    pub catalog_id: EntityId,
    pub sequent_id: Option<EntityId>,
    pub metadata_id: Option<EntityId>,
}

impl CreateVideoRequest {
    pub fn new(path: String, catalog_id: EntityId, metadata_id: Option<EntityId>) -> Self {
        Self {
            path: path.clone(),
            display_name: path,
            catalog_id,
            metadata_id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, strum::Display)]
#[strum(serialize_all = "snake_case")]
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
}

#[async_trait]
impl Entity for Video {
    type CreateRequest = CreateVideoRequest;
    type Ordering = VideoSort;
    type UpdateRequest = UpdateVideoRequest;

    fn id(&self) -> EntityId {
        self.id
    }

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
                VALUES ($1, $2, $3, $4, $5, $6, $7) 
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
        .tap_err(|e| error!("error while creating video: {e}"))?;
        Ok(video)
    }

    #[instrument(skip(executor))]
    async fn create_many<'a>(
        executor: impl PgExecutor<'a>,
        requests: CreateVideoRequestVec,
    ) -> Result<Vec<Self>, sqlx::Error> {
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
            &requests.path[..],
            &requests.display_name[..],
            &requests.short_desc[..],
            &requests.long_desc[..],
            &requests.catalog_id[..],
            &requests.metadata_id[..] as _
        )
        .fetch_all(executor)
        .await
        .tap_err(|e| error!("error while creating videos: {e}"))?;

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
            .tap_err(|e| error!("error while finding video: {e}"))?;
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
            .tap_err(|e| error!("error while finding videos: {e}"))?;
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
            .await
            .tap_err(|e| error!("error while updating video: {e}"))?;
        Ok(video)
    }

    #[instrument(skip(executor))]
    async fn delete<'a>(executor: impl PgExecutor<'a>, id: EntityId) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM video WHERE id = $1", id)
            .execute(executor)
            .await
            .tap_err(|e| error!("error while deleting video: {e}"))?;
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
            .tap_err(|e| error!("error while deleting videos: {e}"))?;
        Ok(result.rows_affected())
    }

    #[instrument(skip(executor))]
    async fn count_all<'a>(executor: impl PgExecutor<'a>) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM video"#)
            .fetch_one(executor)
            .await
            .tap_err(|e| error!("error while counting videos: {e}"))?;
        Ok(count)
    }
}

impl Indexable for Video {
    fn index_name() -> &'static str {
        "videos"
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
        .tap_err(|e| error!("error while finding video by catalog id: {e}"))?;
        Ok(video)
    }
}
