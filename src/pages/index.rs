use actix_web::*;
use askama_actix::{Template, TemplateToResponse};
use serde_derive::Deserialize;
use std::borrow::Borrow;

use crate::{dto::mangadex::MangaWithCoverArt, services::MangadexService};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
  query: String,
  mangas: Option<Vec<MangaWithCoverArt>>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
  query: Option<String>,
}

#[get("/")]
pub async fn index(
  web::Query(SearchQuery { query }): web::Query<SearchQuery>,
  mangadex: web::Data<MangadexService>,
) -> Result<HttpResponse, Error> {
  let mangas = match query.as_ref().map(|q| (q.len(), q)) {
    Some((0, _)) | None => None,
    Some((_, query)) => mangadex.query_manga_by_title(query).await.ok(),
  };

  Ok(
    IndexTemplate {
      query: query.unwrap_or("".to_owned()),
      mangas,
    }
    .to_response(),
  )
}
