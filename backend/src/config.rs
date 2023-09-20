use cli::{Config, LoggerConfig};
use log4rs::{
    append::rolling_file::{
        policy::compound::{
            roll::{delete::DeleteRoller, fixed_window::FixedWindowRoller},
            trigger::size::SizeTrigger,
            CompoundPolicy,
        },
        RollingFileAppender, RollingFileAppenderBuilder,
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Handle,
};
use tower_http::cors::CorsLayer;

pub trait ConfigExt {
    /// Return either very permissive [`CORS`](`CorsLayer`) configuration
    /// or empty one based on `cors_allow_all` field
    fn get_cors_configuration(&self) -> CorsLayer;
}

pub trait LoggerExt {
    /// Return [`tracing_appender`] instance based on [`LoggerConfig`]'s `rotation_frequency` field
    fn get_tracing_appender(&self) -> Result<Handle, LoggerError>;
}

impl ConfigExt for Config {
    fn get_cors_configuration(&self) -> CorsLayer {
        self.cors_allow_all
            .then_some(CorsLayer::very_permissive())
            .unwrap_or_default()
    }
}
impl LoggerExt for LoggerConfig {
    fn get_tracing_appender(&self) -> Result<Handle, LoggerError> {
        // let logfile = RollingFileAppender::builder().build(self.log_file, CompoundPolicy::new(SizeTrigger::new(self.log_size), DeleteRoller::new()));
        // let config = log4rs::Config::builder().appender(Appender::builder().)
        let window_size = 3; // log0, log1, log2
        let fixed_window_roller = FixedWindowRoller::builder()
            .build("log{}", window_size)
            .unwrap();
        let size_limit = 5 * 1024; // 5KB as max log file size to roll
        let size_trigger = SizeTrigger::new(size_limit);
        let compound_policy =
            CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));
        let config = log4rs::Config::builder()
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Debug)))
                    .build(
                        "logfile",
                        Box::new(
                            RollingFileAppender::builder()
                                .encoder(Box::new(PatternEncoder::new("{d} {l}::{m}{n}")))
                                .build("logfile", Box::new(compound_policy))
                                .unwrap(),
                        ),
                    ),
            )
            .build(
                Root::builder()
                    .appender("logfile")
                    .build(log::LevelFilter::Debug),
            )
            .unwrap();

        Ok(log4rs::init_config(config).unwrap())
    }
}

#[derive(Debug)]
pub enum LoggerError {
    InitError,
}
