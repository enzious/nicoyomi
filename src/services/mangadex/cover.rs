use std::collections::HashMap;

use bytes::Bytes;
use serde::{Deserialize, Serialize};

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

    let semaphore = Self::lock_semaphore().await?;

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

  pub async fn download_cover_art(
    &self,
    manga_id: &str,
    filename: &str,
  ) -> Result<Bytes, MangadexServiceError> {
    let uri = format!("https://mangadex.org/covers/{}/{}", manga_id, filename);

    let semaphore = Self::lock_semaphore().await?;

    let res = self
      .client
      .get(uri)
      .send()
      .await
      .map_err(|err| {
        error!("Failed to get cover: {:?}", &err);
        err
      })?
      .body()
      .await
      .map_err(|err| {
        error!("Failed to read cover response: {:?}", &err);
        err
      })?;

    drop(semaphore);

    Ok(res)
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
