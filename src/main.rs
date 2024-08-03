use std::path::PathBuf;

use clap::{Parser, Subcommand};
use plotters::prelude::*;

use ieee754::{Ieee754, SmallFloat, Value};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    Plot {
        output_file: PathBuf,
    },

    #[command(alias = "eval")]
    Evaluate {
        binary: String,

        #[arg(short, long)]
        raw: bool,
    },

    Test {
        binary: String,
    },
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
        Cmd::Evaluate { binary, raw } => {
            if binary.to_lowercase() == "all" {
                let all = SmallFloat::generate_all();
                for f in all {
                    let out = if raw {
                        Value::Number(f.raw_value(), false)
                    } else {
                        f.evaluate()
                    };

                    println!("{f} => {out}");
                }
            } else {
                let f = SmallFloat::parse(&binary).expect("No valid SmallFloat");
                let out = if raw {
                    Value::Number(f.raw_value(), false)
                } else {
                    f.evaluate()
                };

                println!("{f} => {out}");
            }
        }
        Cmd::Test { binary } => {
            let f = Ieee754::<5, 10>::parse(binary).expect("No valid Ieee754<5,10>");
            println!("{f} => {}", f.evaluate());
        }
    }
}
