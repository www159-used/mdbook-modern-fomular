//! CLI entry point for the mdBook preprocessor.

use std::io;
use std::process;

use clap::{crate_version, Arg, ArgMatches, Command};
use mdbook_modern_fomular::{init_logger, FomularPreprocessor};
use mdbook_preprocessor::errors::{Error, Result};
use mdbook_preprocessor::{parse_input, Preprocessor};

use mdbook_modern_fomular::constants::PREPROCESSOR_NAME;

const BIN_NAME: &str = "mdbook-modern-fomular";

fn main() {
    init_logger();

    let matches = build_cli().get_matches();
    let processor = FomularPreprocessor;

    let result = if let Some(args) = matches.subcommand_matches("supports") {
        handle_supports(&processor, args)
    } else {
        handle_preprocess(&processor)
    };

    if let Err(error) = result {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn build_cli() -> Command {
    Command::new(BIN_NAME)
        .version(crate_version!())
        .about("mdBook preprocessor that renders LaTeX math with KaTeX.")
        .subcommand(
            Command::new("supports")
                .about("Report whether a renderer is supported")
                .arg(Arg::new("renderer").required(true)),
        )
}

fn handle_supports(processor: &dyn Preprocessor, args: &ArgMatches) -> Result<()> {
    let renderer = args
        .get_one::<String>("renderer")
        .expect("renderer argument is required");

    if processor.supports_renderer(renderer)? {
        Ok(())
    } else {
        Err(Error::msg(format!(
            "renderer `{renderer}` is not supported by `{PREPROCESSOR_NAME}`"
        )))
    }
}

fn handle_preprocess(processor: &dyn Preprocessor) -> Result<()> {
    let (ctx, book) = parse_input(io::stdin())?;
    let processed = processor.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_name_matches_preprocessor() {
        assert!(build_cli().get_name().ends_with(PREPROCESSOR_NAME));
    }
}
