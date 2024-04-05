use std::{
  env::var,
  fs::{create_dir_all, File},
  io::Result,
  path::PathBuf,
};

use directories::ProjectDirs;
use lazy_static::lazy_static;
use tracing_error::ErrorLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Layer};

lazy_static! {
  pub static ref PKG_NAME: String = env!("CARGO_PKG_NAME").to_string();
  pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();
  pub static ref DATA_FOLDER: Option<PathBuf> = var(format!("{}_DATA", PROJECT_NAME.as_str())).ok().map(PathBuf::from);
  pub static ref CONFIG_FOLDER: Option<PathBuf> =
    var(format!("{}_CONFIG", PROJECT_NAME.as_str())).ok().map(PathBuf::from);
  pub static ref GIT_COMMIT_HASH: String =
    var(format!("{}_GIT_INFO", PROJECT_NAME.as_str())).unwrap_or_else(|_| String::from("UNKNOWN"));
  pub static ref LOG_FILE: String = format!("{}.log", PKG_NAME.as_str());
}

fn project_directory() -> Option<ProjectDirs> {
  ProjectDirs::from("be", "endevops", &PKG_NAME)
}

pub fn get_data_dir() -> PathBuf {
  return if let Some(s) = DATA_FOLDER.clone() {
    s
  } else if let Some(proj_dirs) = project_directory() {
    proj_dirs.data_local_dir().to_path_buf()
  } else {
    PathBuf::from(".").join(".data")
  };
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn initialize_logging() -> Result<()> {
  let directory = get_data_dir();
  create_dir_all(&directory)?;
  let log_path = directory.join(LOG_FILE.clone());
  let log_file = File::create(log_path)?;

  #[cfg(debug_assertions)]
  std::env::set_var("RUST_LOG", var("RUST_LOG").unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME"))));

  let file_subscriber = tracing_subscriber::fmt::layer()
    .with_file(true)
    .with_line_number(true)
    .with_writer(log_file)
    .with_target(false)
    .with_ansi(false)
    .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());
  tracing_subscriber::registry().with(file_subscriber).with(ErrorLayer::default()).init();

  Ok(())
}

/// Similar to the `std::dbg!` macro, but generates `tracing` events rather
/// than printing to stdout.
///
/// By default, the verbosity level for the generated events is `DEBUG`, but
/// this can be customized.
#[macro_export]
macro_rules! trace_dbg {
    (target: $target:expr, level: $level:expr, $ex:expr) => {{
        match $ex {
            value => {
                tracing::event!(target: $target, $level, ?value, stringify!($ex));
                value
            }
        }
    }};
    (level: $level:expr, $ex:expr) => {
        trace_dbg!(target: module_path!(), level: $level, $ex)
    };
    (target: $target:expr, $ex:expr) => {
        trace_dbg!(target: $target, level: tracing::Level::DEBUG, $ex)
    };
    ($ex:expr) => {
        trace_dbg!(level: tracing::Level::DEBUG, $ex)
    };
}
