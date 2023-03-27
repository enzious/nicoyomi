use std::sync::Arc;
use std::time::Duration;

use actix_web::error::PayloadError;
use awc::error::SendRequestError;
use futures::lock::Mutex;
use lazy_static::lazy_static;
use thiserror::Error;
use tokio::sync::{AcquireError, SemaphorePermit};
use tokio::task::JoinError;
use tokio::time::{sleep, Sleep};

pub mod chapter;
pub mod cover;
pub mod manga;
pub mod volume;

lazy_static! {
  static ref RUSTLS_CONFIG: rustls::ClientConfig = {
    let mut roots = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().expect("could not load native certs") {
      roots.add(&rustls::Certificate(cert.0)).unwrap();
    }

    rustls::ClientConfig::builder()
      .with_safe_defaults()
      .with_root_certificates(roots)
      .with_no_client_auth()
  };
  static ref PERMITS_PER_SECOND: usize = 5;
  static ref SEMAPHORE: tokio::sync::Semaphore = tokio::sync::Semaphore::new(*PERMITS_PER_SECOND);
}

pub struct MangadexPermit<'a> {
  sleep: Option<Sleep>,
  permit: Option<SemaphorePermit<'a>>,
}

impl<'a> MangadexPermit<'a> {
  pub fn new(permit: SemaphorePermit<'a>) -> Self {
    Self {
      sleep: Some(sleep(Duration::from_secs(1))),
      permit: Some(permit),
    }
  }
}

impl<'a> Drop for MangadexPermit<'a> {
  fn drop(&mut self) {
    if let Some(sleep) = self.sleep.take() {
      let permit = self.permit.take();

      actix_web::rt::spawn(async {
        let _ = permit;

        sleep.await;
      });
    }
  }
}

#[derive(Default)]
struct State {}

pub struct MangadexService {
  state: Arc<Mutex<State>>,
  client: awc::Client,
}

impl MangadexService {
  pub async fn acquire_permit<'a>() -> Result<MangadexPermit<'a>, MangadexServiceError> {
    Ok(MangadexPermit::new(SEMAPHORE.acquire().await?))
  }
}

fn build_client() -> awc::Client {
  awc::Client::builder()
    .connector(awc::Connector::new().rustls(Arc::new(RUSTLS_CONFIG.to_owned())))
    .finish()
}

#[derive(Clone)]
pub struct MangadexServiceBuilder {
  state: Arc<Mutex<State>>,
}

impl MangadexServiceBuilder {
  pub fn new() -> Self {
    Self {
      state: Default::default(),
    }
  }

  pub fn build(&self) -> MangadexService {
    MangadexService {
      state: self.state.to_owned(),
      client: build_client(),
    }
  }
}

impl Clone for MangadexService {
  fn clone(&self) -> Self {
    MangadexService {
      state: self.state.to_owned(),
      client: build_client(),
    }
  }
}

impl MangadexService {
  pub fn builder() -> MangadexServiceBuilder {
    MangadexServiceBuilder::new()
  }
}

#[derive(Debug, Error)]
pub enum MangadexServiceError {
  #[error("chapter download error")]
  ChapterDownloadError,
  #[error("internal error")]
  InternalError,
  #[error("io error")]
  IoError(#[from] std::io::Error),
  #[error("join error")]
  JoinError(#[from] JoinError),
  #[error("failed to retrieve payload")]
  PayloadError(#[from] PayloadError),
  #[error("failed to acquire semaphore")]
  AcquireError(#[from] AcquireError),
  #[error("failed to send request")]
  SendRequestError(#[from] SendRequestError),
  #[error("failed to deserialize")]
  SerdeError(#[from] serde_json::Error),
  #[error("failed to serialize error")]
  UrlEncodeError(#[from] serde_urlencoded::ser::Error),
}
