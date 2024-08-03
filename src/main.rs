use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

use ieee754::{DoubleFloat, Ieee754, QuadrupleFloat, SingleFloat, SmallFloat, Value};

mod plot;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,

    /// The float format
    #[arg(short, long, default_value = "small")]
    format: FloatFormat,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Evaluate a single binary float number
    #[command(alias = "eval")]
    Evaluate {
        /// The float number as a sequence of 0s and 1s
        binary: String,

        /// Don't check for special cases like infinity or NaN
        #[arg(short, long)]
        raw: bool,
    },

    /// Show the distribution of all float values (only -f small supported)
    All {
        /// Plot the distribution to an image
        #[arg(long)]
        plot: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FloatFormat {
    Small,
    Single,
    Double,
    Quadruple,
}

fn output_float<const R: usize, const P: usize>(f: Ieee754<R, P>, raw: bool) {
    let val = if raw {
        Value::Number(f.raw_value(), false)
    } else {
        f.evaluate()
    };

    println!("{f} => {val}");
}

fn evaluate(binary: String, raw: bool, format: FloatFormat) -> Result<(), String> {
    match format {
        FloatFormat::Small => {
            let f = SmallFloat::parse(&binary)?;
            output_float(f, raw);
        }
        FloatFormat::Single => {
            let f = SingleFloat::parse(&binary)?;
            output_float(f, raw);
        }
        FloatFormat::Double => {
            let f = DoubleFloat::parse(&binary)?;
            output_float(f, raw);
        }
        FloatFormat::Quadruple => {
            let f = QuadrupleFloat::parse(&binary)?;
            output_float(f, raw);
        }
    }

    Ok(())
}

fn all(plot: Option<PathBuf>) {
    if let Some(output_path) = plot {
        plot::plot(output_path);
    } else {
        for f in SmallFloat::generate_all() {
            output_float(f, false);
        }
    };
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Evaluate { binary, raw } => {
            if let Err(err) = evaluate(binary, raw, cli.format) {
                eprintln!("Error: {err}");
                std::process::exit(1);
            }
        }
        Cmd::All { plot } => {
            if cli.format == FloatFormat::Small {
                all(plot);
            } else {
                eprintln!("Format {:?} not supported for subcommand `all`", cli.format);
                std::process::exit(1);
            }
        }
    }
}
