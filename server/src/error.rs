use axum::response::{IntoResponse, Response};
use std::fmt::{Debug, Display};
use thiserror::Error;

#[derive(Error, Debug)]
pub struct AppError<T: Debug + Display>(#[from] T);

impl<T: Debug + Display> IntoResponse for AppError<T> {
    fn into_response(self) -> Response {
        format!("{}", self.0).into_response()
    }
}
