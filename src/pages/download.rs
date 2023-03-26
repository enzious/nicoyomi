use actix_web::{error, get, http::header::DispositionParam, web, Error};
use tokio::process::Command;

use crate::services::{mangadex::MangadexServiceError, MangadexService};

#[get("/download/chapter/{chapter_id}/{manga_name}/{chapter_name}")]
pub async fn download_chapter(
  path: web::Path<(String, String, String)>,
) -> Result<actix_files::NamedFile, Error> {
  let (chapter_id, manga_name, chapter_name) = path.into_inner();

  let download_path_epub_str = format!("downloads/{id}/{id}.epub", id = &chapter_id);
  let download_path_epub = std::path::Path::new(&download_path_epub_str);

  if !download_path_epub.exists() {
    info!("Downloading chapter...");

    let semaphore = MangadexService::lock_semaphore()
      .await
      .map_err(error::ErrorInternalServerError)?;

    Command::new("mangadex-dl")
      .arg("-f")
      .arg("epub")
      .arg(&format!("https://mangadex.org/chapter/{}", &chapter_id))
      .arg("-d")
      .arg(&format!("downloads/{}", &chapter_id))
      .arg("-ucc")
      .arg("-uvc")
      .arg("-ncf")
      .arg("-nmf")
      .arg("-fn")
      .arg(&format!("{}", &chapter_id))
      .arg("--no-track")
      .output()
      .await
      .map_err(error::ErrorInternalServerError)?;

    drop(semaphore);

    if !download_path_epub.exists() {
      return Err(MangadexServiceError::ChapterDownloadError)
        .map_err(error::ErrorInternalServerError)?;
    }
  }

  let download_path_azw3_str = format!("downloads/{id}/{id}.azw3", id = &chapter_id);
  let download_path_azw3 = std::path::Path::new(&download_path_azw3_str);

  if !download_path_azw3.exists() {
    Command::new("ebook-convert")
      .arg(&download_path_epub)
      .arg(&download_path_azw3)
      .output()
      .await
      .map_err(error::ErrorInternalServerError)?;
  }

  let named_file = actix_files::NamedFile::open(download_path_azw3)?;

  let new_filename = format!("{}_{}.azw3", &manga_name, &chapter_name);

  let mut content_disposition = named_file.content_disposition().to_owned();
  content_disposition.parameters = content_disposition.parameters.drain(..).fold(
    vec![DispositionParam::Filename(new_filename)],
    |mut acc, param| {
      acc.push(param);

      acc
    },
  );

  Ok(named_file.set_content_disposition(content_disposition))
}
