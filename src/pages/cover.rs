use actix_web::{error, get, web, Error, Responder};

use crate::services::MangadexService;

#[get("/cover/{manga_id}/{filename}")]
pub(super) async fn cover(
  path: web::Path<(String, String)>,
  mangadex: web::Data<MangadexService>,
) -> Result<impl Responder, Error> {
  let (manga_id, filename) = path.into_inner();

  Ok(
    mangadex
      .download_cover_response(&manga_id, &filename)
      .await
      .map_err(error::ErrorInternalServerError)?,
  )
}
