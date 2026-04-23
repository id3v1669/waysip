mod cli;
#[cfg(feature = "logger")]
mod logger;
mod settings;
mod utils;

use clap::Parser;
use cli::Cli;
use libwaysip::SelectionType;
use settings::{SelectionDispatch, read_boxes_from_stdin, resolve_output_format, run_selection};
use utils::apply_format;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Cli::parse();

    #[cfg(feature = "completions")]
    if let Some(shell) = args.completions {
        utils::print_completions(shell);
        return Ok(());
    }

    #[cfg(feature = "logger")]
    logger::setup(&args);

    let fmt = resolve_output_format(&mut args);

    if args.boxes {
        let boxes = read_boxes_from_stdin();
        let info = run_selection(&mut args, SelectionType::PredefinedBoxes, Some(boxes));
        #[cfg(feature = "benchmark")]
        if args.bench {
            print_bench_results(&info.frames_per_second);
        }
        print!("{}", apply_format(&info, &fmt, false));
    } else if let Some(mode) = SelectionDispatch::from_cli(&args) {
        let info = run_selection(&mut args, mode.selection_type(), None);
        #[cfg(feature = "benchmark")]
        if args.bench {
            print_bench_results(&info.frames_per_second);
        }
        let use_screen_format = match mode {
            SelectionDispatch::DimensionsOrOutput => {
                matches!(info.effective_selection_type, Some(SelectionType::Screen))
            }
            SelectionDispatch::Screen => true,
            SelectionDispatch::Point | SelectionDispatch::Area => false,
        };
        print!("{}", apply_format(&info, &fmt, use_screen_format));
    }

    Ok(())
}

#[cfg(feature = "benchmark")]
fn print_bench_results(fps: &[u32]) {
    if fps.len() < 5 {
        eprintln!("benchmark: not enough data (selection lasted less than 5 seconds)");
        return;
    }
    let trimmed = &fps[1..fps.len() - 1];
    let total: u64 = trimmed.iter().map(|&f| f as u64).sum();
    let avg = total as f64 / trimmed.len() as f64;
    let min = *trimmed.iter().min().unwrap();
    eprintln!("benchmark results (first and last second excluded):");
    eprintln!("  total frames : {total}");
    eprintln!("  avg fps      : {avg:.1}");
    eprintln!("  min fps      : {min}");
}
