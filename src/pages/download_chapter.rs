use actix_web::{error, get, web, Error, Responder};

use crate::services::MangadexService;

#[get("/download/chapter/{chapter_id}/{manga_name}/{chapter_name}")]
pub(super) async fn download_chapter(
  path: web::Path<(String, String, String)>,
  mangadex: web::Data<MangadexService>,
) -> Result<impl Responder, Error> {
  let (chapter_id, manga_name, chapter_name) = path.into_inner();

  Ok(
    mangadex
      .download_chapter_response(&chapter_id, &manga_name, &chapter_name)
      .await
      .map_err(error::ErrorInternalServerError)?,
  )
}
