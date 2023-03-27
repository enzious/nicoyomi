use serde::Deserialize;

use crate::dto::mangadex::Volumes;

use super::{MangadexService, MangadexServiceError};

#[derive(Debug, Deserialize)]
pub struct MangaVolumesResponse {
  volumes: Volumes,
}

impl MangadexService {
  pub async fn get_manga_volumes(&self, manga_id: &str) -> Result<Volumes, MangadexServiceError> {
    let uri = format!(
      "https://api.mangadex.org/manga/{}/aggregate?translatedLanguage[]=en",
      &manga_id
    );

    let semaphore = Self::acquire_permit().await?;

    let res = self.client.get(uri).send().await?.body().await?;

    drop(semaphore);

    let MangaVolumesResponse { volumes, .. } =
      serde_json::from_slice::<MangaVolumesResponse>(&res[..])?;

    Ok(volumes)
  }
}
