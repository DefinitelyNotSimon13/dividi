use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::uploaded_file::UploadedFile;

pub struct HtmlTemplate<T>(pub T);

pub struct Error {
    pub message: String,
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self {
            message: value.into(),
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self { message: value }
    }
}

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate;

#[derive(Template)]
#[template(path = "components/file-list.html")]
pub struct FileList {
    pub files: Vec<UploadedFile>,
    pub error: Option<Error>,
}

impl FileList {
    pub fn ok(files: Vec<UploadedFile>) -> Self {
        Self { files, error: None }
    }

    pub fn err(error: Error, files: Vec<UploadedFile>) -> Self {
        Self {
            files,
            error: Some(error),
        }
    }
}
