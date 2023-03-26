use std::io::Write;

use actix_web::{error, get, web, Error};

use crate::services::MangadexService;

#[get("/cover/{manga_id}/{filename}")]
pub async fn cover(
  path: web::Path<(String, String)>,
  mangadex: web::Data<MangadexService>,
) -> Result<actix_files::NamedFile, Error> {
  let (manga_id, filename) = path.into_inner();

  let cover_path_str = format!("cover/{}/{}", &manga_id, &filename);
  let cover_path = std::path::Path::new(&cover_path_str);

  if !cover_path.exists() {
    info!("Downloading cover...");

    let cover_dir = format!("cover/{}", &manga_id);
    std::fs::create_dir_all(&cover_dir).map_err(error::ErrorInternalServerError)?;

    let bytes = mangadex
      .download_cover_art(&manga_id, &filename)
      .await
      .map_err(error::ErrorInternalServerError)?;

    {
      let cover_path = cover_path.to_path_buf();

      tokio::task::spawn_blocking(move || {
        let mut file = std::fs::OpenOptions::new()
          .create(true)
          .truncate(true)
          .write(true)
          .open(&cover_path)
          .map_err(|err| {
            error!("Failed to save cover: {:?}", &err);

            err
          })?;

        file.write_all(&bytes).map_err(|err| {
          error!("Failed to write cover: {:?}", &err);

          err
        })?;

        Ok::<(), std::io::Error>(())
      })
      .await
      .map_err(error::ErrorInternalServerError)?
      .map_err(error::ErrorInternalServerError)?;
    }
  }

  Ok(actix_files::NamedFile::open(cover_path)?)
}
