use std::collections::HashMap;

use actix_web::Responder;
use awc::error::PayloadError;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

use crate::dto::mangadex::{Manga, MangadexResponse, Relationship};

use super::{MangadexService, MangadexServiceError};

#[derive(Serialize)]
struct CoverQuery<'a> {
  ids: Vec<&'a String>,
}

impl MangadexService {
  pub async fn query_cover_art(
    &self,
    mangas: &Vec<Manga>,
  ) -> Result<HashMap<String, String>, MangadexServiceError> {
    let uri = format!(
      "https://api.mangadex.org/cover?ids[]={}",
      &itertools::join(
        Self::get_cover_art(mangas).iter().map(|i| i.to_owned()),
        "&ids[]=",
      )
    );

    let semaphore = Self::acquire_permit().await?;

    let res = self.client.get(uri).send().await?.body().await?;

    drop(semaphore);

    let res = serde_json::from_slice::<MangadexResponse<Cover>>(&res[..])?;

    match res {
      MangadexResponse::Collection { data, .. } => Ok(HashMap::from_iter(
        data.iter().filter_map(|cover| cover.get_entry()),
      )),
      _ => Err(MangadexServiceError::InternalError),
    }
  }

  fn get_cover_art(mangas: &Vec<Manga>) -> Vec<&String> {
    mangas
      .iter()
      .filter_map(|manga| {
        manga
          .relationships
          .iter()
          .find(|Relationship { _type, .. }| _type == "cover_art")
          .map(|Relationship { id, .. }| id)
      })
      .collect()
  }

  pub async fn download_cover(
    &self,
    manga_id: &str,
    filename: &str,
  ) -> Result<impl Stream<Item = Result<Bytes, PayloadError>>, MangadexServiceError> {
    let uri = format!("https://mangadex.org/covers/{}/{}", manga_id, filename);

    let semaphore = Self::acquire_permit().await?;

    let res = self.client.get(uri).send().await;

    drop(semaphore);

    Ok(res.map_err(|err| {
      error!("Failed to get cover: {:?}", &err);
      err
    })?)
  }

  pub async fn download_cover_response(
    &self,
    manga_id: &str,
    filename: &str,
  ) -> Result<impl Responder, MangadexServiceError> {
    let cover_path_str = format!("cover/{}/{}", &manga_id, &filename);
    let cover_path = std::path::Path::new(&cover_path_str);

    if !cover_path.exists() {
      info!("Downloading cover...");

      let cover_dir = format!("cover/{}", &manga_id);
      tokio::fs::create_dir_all(&cover_dir).await?;

      let mut stream = self.download_cover(&manga_id, &filename).await?;

      {
        let cover_path = cover_path.to_path_buf();

        let mut file = tokio::fs::OpenOptions::new()
          .create(true)
          .truncate(true)
          .write(true)
          .open(&cover_path)
          .await
          .map_err(|err| {
            error!("Failed to save cover: {:?}", &err);

            err
          })?;

        while let Some(chunk) = stream.next().await {
          if let Err(err) = file.write_all(&(chunk?)[..]).await {
            let _ = tokio::fs::remove_dir_all(cover_dir).await;

            return Ok(Err(err)?);
          }
        }
      }
    }

    Ok(actix_files::NamedFile::open(cover_path)?)
  }
}

#[derive(Debug, Deserialize)]
pub struct Cover {
  attributes: CoverAttributes,
  relationships: Vec<Relationship>,
}

impl Cover {
  pub fn get_entry(&self) -> Option<(String, String)> {
    self
      .relationships
      .iter()
      .find(|Relationship { _type, .. }| _type == "manga")
      .map(|Relationship { id, .. }| (id.to_owned(), self.attributes.file_name.to_owned()))
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverAttributes {
  file_name: String,
}
