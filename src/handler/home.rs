
use axum::response::IntoResponse;

use crate::templates::{HomeTemplate, HtmlTemplate};

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate {};
    HtmlTemplate(template)
}
