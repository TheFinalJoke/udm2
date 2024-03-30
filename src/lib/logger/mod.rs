use clap_verbosity_flag::Verbosity;
use log::SetLoggerError;
use pretty_env_logger::env_logger::Env;
use pretty_env_logger::env_logger::WriteStyle;
pub struct MyLogger {}
// writes to a file
impl MyLogger {
    pub fn init(verbose: Verbosity, _log_file_path: Option<&str>) -> Result<(), SetLoggerError> {
        let mut build = pretty_env_logger::formatted_builder();
        build.write_style(WriteStyle::Always);
        if verbose.is_present() {
            build.filter(None, verbose.log_level_filter());
        } else {
            let env = Env::default().default_filter_or("off");
            build.parse_env(env);
        }
        build.init();
        Ok(())
    }
}