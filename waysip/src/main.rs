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
        {
            if args.bench_fn {
                print_bench_results("--bench-fn", &info.timestamps_fn);
            }
            if args.bench_total {
                print_bench_results("--bench-total", &info.timestamps_total);
            }
        }
        print!("{}", apply_format(&info, &fmt, false));
    } else if let Some(mode) = SelectionDispatch::from_cli(&args) {
        let info = run_selection(&mut args, mode.selection_type(), None);
        #[cfg(feature = "benchmark")]
        {
            if args.bench_fn {
                print_bench_results("--bench-fn", &info.timestamps_fn);
            }
            if args.bench_total {
                print_bench_results("--bench-total", &info.timestamps_total);
            }
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
fn print_bench_results(bench_type: &str, timestamps: &[u32]) {
    if timestamps.len() < 2 {
        eprintln!("{bench_type}: no data");
        return;
    }
    let frametimes: Vec<u32> = timestamps
        .windows(2)
        .map(|w| w[1].wrapping_sub(w[0]))
        .collect();
    let total = frametimes.len();
    let sum: u64 = frametimes.iter().map(|&f| f as u64).sum();
    let avg_ft = sum as f64 / total as f64;
    let min_ft = *frametimes.iter().min().unwrap();
    let max_ft = *frametimes.iter().max().unwrap();
    let avg_fps = 1000.0 / avg_ft;
    let min_fps = 1000.0 / max_ft as f64;
    let max_fps = 1000.0 / min_ft.max(1) as f64;
    eprintln!("{bench_type} results:");
    eprintln!("  total frames  : {total}");
    eprintln!("  avg fps       : {avg_fps:.1}");
    eprintln!("  min fps       : {min_fps:.0}");
    eprintln!("  max fps       : {max_fps:.0}");
    eprintln!("  avg frametime : {avg_ft:.2}ms");
    eprintln!("  min frametime : {min_ft}ms");
    eprintln!("  max frametime : {max_ft}ms");
}
