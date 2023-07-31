use axum::Router;
use tracing::{info, instrument};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Binge@Home",
        license(
            name = "MIT",
            url = "https://github.com/szattila98/binge_at_home/blob/main/LICENSE"
        )
    ),
    paths(crate::api::health_check::health_check)
)]
struct ApiDoc;

#[instrument(skip_all)]
pub fn add_swagger_ui(router: Router) -> Router {
    const SWAGGER_PATH: &str = "/swagger-ui";
    let router =
        router.merge(SwaggerUi::new(SWAGGER_PATH).url("/api-docs/openapi.json", ApiDoc::openapi()));
    info!("serving swagger-ui at path '{SWAGGER_PATH}'");
    router
}
