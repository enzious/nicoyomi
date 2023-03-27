use actix_web::*;
use askama_actix::{Template, TemplateToResponse};

#[derive(Template)]
#[template(path = "downloads.html")]
struct DownloadsTemplate {
  // active_downloads: Vec<Download>,
  downloads: Vec<Download>,
}

#[get("/downloads")]
pub(super) async fn downloads() -> Result<HttpResponse, Error> {
  Ok(
    DownloadsTemplate {
      // active_downloads: vec![],
      downloads: vec![],
    }
    .to_response(),
  )
}

pub struct Download {
  // name: String,
}
