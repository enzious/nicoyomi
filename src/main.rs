#[macro_use]
extern crate log;
#[macro_use(slog_o)]
extern crate slog;

use actix_files::Files;
use actix_web::web;
use actix_web::{App, HttpServer};
use pages::cover;
use pages::download_chapter;
use pages::downloads;
use pages::index;
use pages::manga;
use services::mangadex::MangadexService;

mod dto;
mod logging;
mod pages;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
  logging::init();

  let mangadex = MangadexService::builder();

  std::fs::create_dir_all("./public").expect("failed to create public folder");

  HttpServer::new(move || {
    let mangadex = mangadex.to_owned();

    App::new()
      .app_data(web::Data::new(mangadex.build()))
      .service(cover)
      .service(download_chapter)
      .service(downloads)
      .service(index)
      .service(manga)
      .service(Files::new("/", "./public").index_file("index.html"))
  })
  .bind(("0.0.0.0", 6969))?
  .run()
  .await
}
