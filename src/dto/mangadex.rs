use std::collections::HashMap;

use serde::Deserialize;

use crate::utils::safe_str;

#[derive(Debug, Deserialize)]
#[serde(tag = "response", rename_all = "snake_case")]
pub enum MangadexResponse<T> {
  Collection { result: String, data: Vec<T> },
  Entity { result: String, data: T },
}

#[derive(Debug, Deserialize)]
pub struct Manga {
  pub id: String,
  #[serde(rename = "type")]
  pub _type: String,
  pub attributes: MangaAttributes,
  pub relationships: Vec<Relationship>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaAttributes {
  pub title: HashMap<String, String>,
  pub alt_titles: Vec<HashMap<String, String>>,
  pub description: HashMap<String, String>,
  pub tags: Vec<MangaTag>,
  pub year: Option<i32>,
  pub state: String,
  pub status: String,
  pub publication_demographic: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MangaTag {
  pub attributes: MangaTagAttributes,
}

impl MangaTag {
  pub fn get_name(&self, locale: &str) -> Option<&str> {
    self
      .attributes
      .name
      .get(locale)
      .map(|desc| desc as &str)
      .or_else(|| {
        self
          .attributes
          .name
          .iter()
          .nth(0)
          .map(|(_, desc)| desc as &str)
      })
  }
}

#[derive(Debug, Deserialize)]
pub struct MangaTagAttributes {
  pub name: HashMap<String, String>,
}

impl MangaAttributes {
  pub fn get_title(&self, locale: &str) -> Option<&str> {
    let main_title = match self.title.iter().next() {
      Some((ilocale, ititle)) => {
        if ilocale as &str == locale {
          return Some(ititle as &str);
        }

        Some(ititle as &str)
      }
      _ => None,
    };

    self
      .alt_titles
      .iter()
      .find(|lstring| (*lstring).contains_key(locale))
      .and_then(|lstring| lstring.iter().next().map(|(_, l)| l as &str))
      .or(main_title)
  }

  pub fn get_link_title(&self, locale: &str) -> Option<String> {
    self.get_title(locale).map(|locale| safe_str(locale))
  }

  pub fn get_description(&self, locale: &str) -> Option<&str> {
    self
      .description
      .get(locale)
      .map(|desc| desc as &str)
      .or_else(|| self.description.iter().nth(0).map(|(_, desc)| desc as &str))
  }

  pub fn get_tags(&self, locale: &str) -> Vec<&str> {
    self
      .tags
      .iter()
      .filter_map(|tag| tag.get_name(locale))
      .collect()
  }
}

pub struct MangaWithCoverArt {
  pub manga: Manga,
  pub cover_art: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Volumes(pub HashMap<String, Volume>);
pub type OrderedVolumes<'a> = Vec<OrderedVolume<'a>>;

#[derive(Debug, Deserialize)]
pub struct Volume {
  pub volume: String,
  pub chapters: HashMap<String, Chapter>,
}

pub struct OrderedVolume<'a> {
  pub volume: &'a String,
  pub chapters: Vec<&'a Chapter>,
}

impl Volumes {
  pub fn get_ordered_volumes(&self) -> OrderedVolumes {
    let mut volumes = self
      .0
      .values()
      .map(|Volume { volume, chapters }| {
        let mut chapters = chapters.values().collect::<Vec<_>>();

        chapters.sort_by(|a, b| natord::compare(&a.chapter, &b.chapter));

        OrderedVolume { volume, chapters }
      })
      .collect::<Vec<_>>();

    volumes.sort_by(|a, b| natord::compare(&a.volume, &b.volume));

    volumes
  }

  pub fn count(&self) -> usize {
    self.0.len()
  }
}

#[derive(Debug, Deserialize)]
pub struct Chapter {
  pub chapter: String,
  pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Relationship {
  pub id: String,
  #[serde(rename = "type")]
  pub _type: String,
  pub relationship: Option<String>,
}
