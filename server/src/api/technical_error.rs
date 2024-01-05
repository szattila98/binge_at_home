use std::sync::Arc;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Query, State},
    response::{AppendHeaders, Redirect, Response},
};
use axum_extra::routing::TypedPath;
use hmac::{Hmac, Mac};
use http::StatusCode;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use tap::Tap;
use tracing::{debug, instrument};

use crate::configuration::Configuration;

#[cfg(debug_assertions)]
use super::AppState;

pub const ACCESS_TECHNICAL_ERROR_FLASH_COOKIE: &str = "access-technical-error";

#[derive(TypedPath)]
#[typed_path("/technical-error")]
pub struct Endpoint;

#[derive(Debug, Deserialize)]
pub struct Params {
    reason: String,
    tag: String,
}

#[derive(Serialize, Template)]
#[template(path = "technical-error.html")]
struct HtmlTemplate {
    reason: Option<String>,
}

impl HtmlTemplate {
    fn new(reason: Option<String>) -> Self {
        Self { reason }.tap(|template| debug!("rendered html template:\n{template}"))
    }
}

#[instrument(skip(config/*, cookies*/))]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(
    _: Endpoint,
    params: Option<Query<Params>>,
    State(config): State<Arc<Configuration>>,
    //cookies: CookieJar,
) -> Response {
    // FIXME activate later to not interfere with development, maybe config
    /* let Some(_) = cookies.get(ACCESS_TECHNICAL_ERROR_FLASH_COOKIE) else {
        return Redirect::to(catalogs::Endpoint::PATH).into_response();
    }; */

    let Some(Query(Params { reason, tag })) = params else {
        return (StatusCode::INTERNAL_SERVER_ERROR, HtmlTemplate::new(None)).into_response();
    };

    let Ok(()) = verify_mac_tag(config.hmac_key(), &reason, &tag) else {
        return (StatusCode::INTERNAL_SERVER_ERROR, HtmlTemplate::new(None)).into_response();
    };

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        HtmlTemplate::new(Some(reason)),
    )
        .into_response()
}

pub fn redirect_to_technical_error<T>(config: &Arc<Configuration>, reason: T) -> impl IntoResponse
where
    T: ToString,
{
    let reason = reason.to_string();
    let tag = generate_mac_tag(config.hmac_key(), &reason);
    (
        AppendHeaders([(
            "Set-Cookie",
            format!("{ACCESS_TECHNICAL_ERROR_FLASH_COOKIE}=true; Max-Age=1; HttpOnly=true"),
        )]),
        Redirect::to(&format!("{}?reason={reason}&tag={tag}", Endpoint::PATH)),
    )
}

fn verify_mac_tag(hmac_key: &Secret<String>, str: &str, tag: &str) -> anyhow::Result<()> {
    let tag = hex::decode(tag)?;
    let mut mac = get_mac(hmac_key);
    mac.update(str.as_bytes());
    mac.verify_slice(&tag)?;
    Ok(())
}

fn generate_mac_tag(hmac_key: &Secret<String>, str: &str) -> String {
    let mut mac = get_mac(hmac_key);
    mac.update(str.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

fn get_mac(hmac_key: &Secret<String>) -> impl Mac {
    Hmac::<sha2::Sha256>::new_from_slice(hmac_key.expose_secret().as_bytes())
        .expect("hmac could not be initialised")
}
