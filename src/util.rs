pub type DateTime = time::OffsetDateTime;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Get non-2xx response from upstream")]
  UpstreamNon2xx(http::Response<axum::body::Body>),

  #[error("IO error")]
  Io(#[from] std::io::Error),

  #[error("HTTP error")]
  Http(#[from] http::Error),

  #[error("Hyper client error")]
  HyperClient(#[from] hyper_util::client::legacy::Error),

  #[error("Axum error")]
  Axum(#[from] axum::Error),

  #[error("YAML parse error")]
  Yaml(#[from] serde_yaml::Error),

  #[error("Bad time format")]
  TimeFormat(#[from] time::error::Format),

  #[error("Feed error")]
  Rss(#[from] rss::Error),

  #[error("Feed parsing error")]
  FeedParse(&'static str),

  #[error("{0}")]
  Message(String),

  #[error("Generic anyhow error")]
  Generic(#[from] anyhow::Error),
}
