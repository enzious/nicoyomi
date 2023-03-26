use serde::Serialize;

use crate::dto::mangadex::{Manga, MangaWithCoverArt, MangadexResponse};

use super::{MangadexService, MangadexServiceError};

#[derive(Serialize)]
struct ByTitleQuery<'a> {
  title: &'a str,
}

impl MangadexService {
  pub async fn query_manga_by_id(
    &self,
    id: &str,
  ) -> Result<Option<MangaWithCoverArt>, MangadexServiceError> {
    let semaphore = Self::lock_semaphore().await?;

    let res = self
      .client
      .get(&format!("https://api.mangadex.org/manga/{}", id))
      .send()
      .await?
      .body()
      .await?;

    drop(semaphore);

    let res = serde_json::from_slice::<MangadexResponse<Manga>>(&res[..])?;

    match res {
      MangadexResponse::Entity { data, .. } => {
        let mut mangas = vec![data];

        let cover_art = self.query_cover_art(&mangas).await?;

        Ok(
          mangas
            .drain(..)
            .map(|manga| {
              let cover_art = cover_art
                .get(&manga.id)
                .map(|file_name| format!("{}/{}", manga.id, file_name));

              MangaWithCoverArt { manga, cover_art }
            })
            .collect::<Vec<MangaWithCoverArt>>()
            .pop(),
        )
      }
      _ => Err(MangadexServiceError::InternalError),
    }
  }

  pub async fn query_manga_by_title(
    &self,
    title: &str,
  ) -> Result<Vec<MangaWithCoverArt>, MangadexServiceError> {
    let semaphore = Self::lock_semaphore().await?;

    let res = self
      .client
      .get("https://api.mangadex.org/manga")
      .query(&ByTitleQuery { title })?
      .send()
      .await?
      .body()
      .await?;

    drop(semaphore);

    let res = serde_json::from_slice::<MangadexResponse<Manga>>(&res[..]).map_err(|err| {
      error!("Failed to parse json response: {:?}", &err);

      err
    })?;

    match res {
      MangadexResponse::Collection { mut data, .. } => {
        let cover_art = self.query_cover_art(&data).await?;

        Ok(
          data
            .drain(..)
            .map(|manga| {
              let cover_art = cover_art
                .get(&manga.id)
                .map(|file_name| format!("{}/{}", manga.id, file_name));

              MangaWithCoverArt { manga, cover_art }
            })
            .collect(),
        )
      }
      _ => Err(MangadexServiceError::InternalError),
    }
  }
}
