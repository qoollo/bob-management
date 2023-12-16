use crate::prelude::*;
use cli::{Config, LoggerConfig};
use file_rotate::{suffix::AppendTimestamp, ContentLimit, FileRotate};
use thiserror::Error;
use tower_http::cors::CorsLayer;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::{filter::LevelFilter, prelude::*, util::SubscriberInitExt};

#[allow(clippy::module_name_repetitions)]
pub trait ConfigExt {
    /// Return either very permissive [`CORS`](`CorsLayer`) configuration
    /// or empty one based on `cors_allow_all` field
    fn get_cors_configuration(&self) -> CorsLayer;
}

pub trait LoggerExt {
    /// Initialize logger.
    ///
    /// Returns [`WorkerGuard`]s for off-thread writers.
    /// Should not be dropped.
    ///
    /// # Errors
    ///
    /// Function returns error if `init_file_rotate` fails
    fn init_logger(&self) -> Result<Vec<WorkerGuard>, LoggerError>;

    /// Returns [`std:io::Write`] object that rotates files on write
    ///
    /// # Errors
    ///
    /// Function returns error if `log_file` is not specified
    fn init_file_rotate(&self) -> Result<FileRotate<AppendTimestamp>, LoggerError>;

    /// Returns non-blocking file writer
    ///
    /// Also returns [`WorkerGuard`] for off-thread writing.
    /// Should not be dropped.
    ///
    /// Returns `None` if logger is not enabled
    ///
    /// # Errors
    ///
    /// This function will return an error if the file logger configuration is empty, file logging
    /// is disabled or logs filename is not specified
    fn non_blocking_file_writer(&self) -> Result<Option<(NonBlocking, WorkerGuard)>, LoggerError>;

    /// Returns non-blocking stdout writer
    ///
    /// Also returns [`WorkerGuard`] for off-thread writing.
    /// Should not be dropped.
    ///
    /// Returns `None` if logger is not enabled
    ///
    /// # Errors
    ///
    /// This function will return an error if the stdout logger configuration is empty or stdout logging
    /// is disabled
    fn non_blocking_stdout_writer(&self)
        -> Result<Option<(NonBlocking, WorkerGuard)>, LoggerError>;
}

impl ConfigExt for Config {
    fn get_cors_configuration(&self) -> CorsLayer {
        self.cors_allow_all
            .then_some(CorsLayer::very_permissive())
            .unwrap_or_default()
    }
}

impl LoggerExt for LoggerConfig {
    fn init_logger(&self) -> Result<Vec<WorkerGuard>, LoggerError> {
        let mut guards = Vec::with_capacity(2);

        let mut layers_iter = [
            self.non_blocking_file_writer()?,
            self.non_blocking_stdout_writer()?,
        ]
        .into_iter()
        .flatten()
        .map(|(writer, guard)| {
            guards.push(guard);
            tracing_subscriber::fmt::layer()
                .with_writer(writer)
                .with_filter(LevelFilter::from_level(self.trace_level))
        });

        if let Some(first_layer) = layers_iter.next() {
            tracing_subscriber::registry()
                .with(layers_iter.fold(first_layer.boxed(), |layer, next_layer| {
                    layer.and_then(next_layer).boxed()
                }))
                .init();
        };

        Ok(guards)
    }

    fn init_file_rotate(&self) -> Result<FileRotate<AppendTimestamp>, LoggerError> {
        let config = self.file.as_ref().ok_or(LoggerError::EmptyConfig)?;
        let log_file = config.log_file.as_ref().ok_or(LoggerError::NoFileName)?;
        log_file
            .is_file()
            .then(|| {
                FileRotate::new(
                    log_file,
                    AppendTimestamp::default(file_rotate::suffix::FileLimit::MaxFiles(
                        config.log_amount,
                    )),
                    ContentLimit::BytesSurpassed(config.log_size),
                    file_rotate::compression::Compression::OnRotate(1),
                    None,
                )
            })
            .ok_or_else(|| LoggerError::IncorrectFileName.into())
    }

    fn non_blocking_file_writer(&self) -> Result<Option<(NonBlocking, WorkerGuard)>, LoggerError> {
        self.file.as_ref().map_or_else(
            || Err(LoggerError::EmptyConfig.into()),
            |config| {
                Ok(config
                    .enabled
                    .then_some(tracing_appender::non_blocking(self.init_file_rotate()?)))
            },
        )
    }

    fn non_blocking_stdout_writer(
        &self,
    ) -> Result<Option<(NonBlocking, WorkerGuard)>, LoggerError> {
        self.stdout.as_ref().map_or_else(
            || Err(LoggerError::EmptyConfig.into()),
            |config| {
                Ok(config
                    .enabled
                    .then(|| tracing_appender::non_blocking(std::io::stdout())))
            },
        )
    }
}

#[derive(Debug, Error)]
pub enum LoggerError {
    #[error("Empty logger configuration")]
    EmptyConfig,
    #[error("No filename specified")]
    NoFileName,
    #[error("Incorrect filename specified")]
    IncorrectFileName,
}
