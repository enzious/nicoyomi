use actix_web::{get, web, Error, HttpResponse};
use askama::Template;
use askama_actix::TemplateToResponse;
use serde::Deserialize;
use std::borrow::Borrow;

use crate::{
  dto::mangadex::{MangaWithCoverArt, Volumes},
  services::MangadexService,
};

#[derive(Template)]
#[template(path = "manga.html")]
struct MangaTemplate {
  manga: Option<MangaWithCoverArt>,
  previous: Option<String>,
  volumes: Option<Volumes>,
}

#[derive(Debug, Deserialize)]
pub(super) struct MangaQuery {
  #[serde(rename = "prev")]
  previous: Option<String>,
}

#[get("/manga/{manga_id}")]
pub(super) async fn manga(
  path: web::Path<(String,)>,
  web::Query(MangaQuery { previous }): web::Query<MangaQuery>,
  mangadex: web::Data<MangadexService>,
) -> Result<HttpResponse, Error> {
  let (manga_id,) = path.into_inner();

  let manga = mangadex
    .query_manga_by_id(&manga_id)
    .await
    .ok()
    .and_then(|res| res);

  let volumes = match manga {
    Some(ref manga) => mangadex.get_manga_volumes(&manga.manga.id).await.ok(),
    _ => None,
  };

  Ok(
    MangaTemplate {
      manga,
      previous,
      volumes,
    }
    .to_response(),
  )
}
