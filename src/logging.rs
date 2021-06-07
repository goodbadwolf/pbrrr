use chrono::prelude::*;
use std::path::PathBuf;

pub fn setup_logger(program_name: &str) -> Result<(), fern::InitError> {
    let log_file_path = PathBuf::from("/tmp").join(format!(
        "{}.{}.{}.log",
        program_name,
        Local::now().format("%Y%m%d"),
        Local::now().format("%H%M%S")
    ));

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(log_file_path)?)
        .chain(std::io::stderr())
        .apply()?;
    Ok(())
}
