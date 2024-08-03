use std::{fmt::Display, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use plotters::prelude::*;

use ieee754::{DoubleFloat, QuadrupleFloat, SingleFloat, SmallFloat, Value};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    #[command(alias = "eval")]
    Evaluate {
        /// The float number as a sequence of 0s and 1s
        binary: String,

        /// The float format
        #[arg(short, long, default_value = "single")]
        format: FloatFormat,

        #[arg(short, long)]
        raw: bool,
    },

    /// Plot the distribution of all SmallFloat values
    Plot {
        /// Output file where to write the plotted image
        output_file: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FloatFormat {
    Small,
    Single,
    Double,
    Quadruple,
}

#[derive(Debug)]
enum FloatWrapper {
    Small(SmallFloat),
    Single(SingleFloat),
    Double(DoubleFloat),
    Quadruple(QuadrupleFloat),
}

impl FloatWrapper {
    fn parse(binary: &str, format: FloatFormat) -> Result<FloatWrapper, String> {
        Ok(match format {
            FloatFormat::Small => FloatWrapper::Small(SmallFloat::parse(binary)?),
            FloatFormat::Single => FloatWrapper::Single(SingleFloat::parse(binary)?),
            FloatFormat::Double => FloatWrapper::Double(DoubleFloat::parse(binary)?),
            FloatFormat::Quadruple => FloatWrapper::Quadruple(QuadrupleFloat::parse(binary)?),
        })
    }

    fn raw_value(&self) -> f32 {
        match self {
            FloatWrapper::Small(x) => x.raw_value(),
            FloatWrapper::Single(x) => x.raw_value(),
            FloatWrapper::Double(x) => x.raw_value(),
            FloatWrapper::Quadruple(x) => x.raw_value(),
        }
    }

    fn evaluate(&self) -> Value {
        match self {
            FloatWrapper::Small(x) => x.evaluate(),
            FloatWrapper::Single(x) => x.evaluate(),
            FloatWrapper::Double(x) => x.evaluate(),
            FloatWrapper::Quadruple(x) => x.evaluate(),
        }
    }
}

impl Display for FloatWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatWrapper::Small(x) => x.fmt(f),
            FloatWrapper::Single(x) => x.fmt(f),
            FloatWrapper::Double(x) => x.fmt(f),
            FloatWrapper::Quadruple(x) => x.fmt(f),
        }
    }
}

fn evaluate(binary: String, format: FloatFormat, raw: bool) {
    let f = match FloatWrapper::parse(&binary, format) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1)
        }
    };

    let out = if raw {
        Value::Number(f.raw_value(), false)
    } else {
        f.evaluate()
    };

    println!("{f} => {out}");
}

fn plot(path: PathBuf) {
    let floats = SmallFloat::generate_all()
        .into_iter()
        .filter(|f| !f.evaluate().is_nan());

    let root_area = BitMapBackend::new(&path, (1500, 150)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Bottom, 20)
        .build_cartesian_2d(-17.0_f32..17.0_f32, -2..2)
        .unwrap();

    ctx.draw_series(floats.map(|f| match f.evaluate() {
        Value::NaN => unreachable!(),
        Value::NegativeZero => Circle::new((0.0, -1), 3, GREEN.filled()),
        Value::PositiveZero => Circle::new((0.0, 1), 3, GREEN.filled()),
        Value::Number(num, denorm) => match denorm {
            false => Circle::new((num, 0), 2, BLUE.filled()),
            true => Circle::new((num, 0), 1, RED.filled()),
        },
        Value::NegativeInfinity => Circle::new((f.raw_value(), 0), 5, MAGENTA),
        Value::PositiveInfinity => Circle::new((f.raw_value(), 0), 5, MAGENTA),
    }))
    .unwrap();

    ctx.configure_mesh()
        .disable_y_axis()
        .label_style(("sans-serif", 20))
        .draw()
        .unwrap();
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Plot { output_file } => plot(output_file),
        Cmd::Evaluate {
            binary,
            raw,
            format,
        } => evaluate(binary, format, raw),
    }
}
