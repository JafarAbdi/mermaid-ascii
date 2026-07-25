use std::ffi::OsString;
use std::fs::File;
use std::io::{self, IsTerminal, Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use mermaid_ascii::render;
use terminal_size::{Width, terminal_size};

const INPUT_BYTES_MAX: u64 = 1024 * 1024;
const WIDTH_MIN: usize = 10;
const WIDTH_MAX: usize = 1000;
const TERMINAL_WIDTH_MARGIN: usize = 2;

fn main() -> ExitCode {
    match run(std::env::args_os().skip(1)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("mermaid-ascii: {message}");
            ExitCode::from(2)
        }
    }
}

fn run(args: impl Iterator<Item = OsString>) -> Result<(), String> {
    let options = parse_args(args)?;
    if options.help {
        return write_stdout(usage());
    }
    if options.version {
        return write_stdout(&format!("mermaid-ascii {}\n", env!("CARGO_PKG_VERSION")));
    }

    let source = read_source(options.input)?;
    let width = resolve_width(options.width, stdout_width());
    let lines = render(&source, width).ok_or_else(|| "input is empty".to_string())?;
    write_stdout(&format_output(&lines))
}

fn stdout_width() -> Option<usize> {
    if !io::stdout().is_terminal() {
        return None;
    }
    terminal_size().map(|(Width(columns), _)| usize::from(columns))
}

fn resolve_width(explicit: Option<usize>, terminal_columns: Option<usize>) -> Option<usize> {
    explicit.or_else(|| {
        terminal_columns.map(|columns| {
            columns
                .saturating_sub(TERMINAL_WIDTH_MARGIN)
                .clamp(WIDTH_MIN, WIDTH_MAX)
        })
    })
}

/// Join rendered lines for stdout, dropping blank leading and trailing lines
/// (some layouts, e.g. left-to-right flowcharts, reserve an empty edge row).
fn format_output(lines: &[String]) -> String {
    let start = lines.iter().position(|l| !l.trim().is_empty()).unwrap_or(0);
    let end = lines
        .iter()
        .rposition(|l| !l.trim().is_empty())
        .map_or(start, |i| i + 1);
    let mut output = lines[start..end].join("\n");
    output.push('\n');
    output
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Options {
    help: bool,
    version: bool,
    width: Option<usize>,
    input: Option<PathBuf>,
}

fn parse_args(mut args: impl Iterator<Item = OsString>) -> Result<Options, String> {
    let mut options = Options::default();
    let mut parse_options = true;
    while let Some(argument) = args.next() {
        match argument.to_str() {
            Some("--") if parse_options => parse_options = false,
            Some("--help" | "-h") if parse_options => options.help = true,
            Some("--version" | "-V") if parse_options => options.version = true,
            Some("--width" | "-w") if parse_options => {
                let value = args.next().ok_or("--width requires a value")?;
                options.width = Some(parse_width(&value)?);
            }
            Some(value) if parse_options && value.starts_with('-') && value != "-" => {
                return Err(format!("unknown option: {value:?}"));
            }
            _ => set_input(&mut options, PathBuf::from(argument))?,
        }
    }
    Ok(options)
}

fn parse_width(value: &OsString) -> Result<usize, String> {
    let value = value
        .to_str()
        .ok_or_else(|| "width is not valid UTF-8".to_string())?;
    let width = value
        .parse::<usize>()
        .map_err(|_| format!("invalid width: {value:?}"))?;
    if (WIDTH_MIN..=WIDTH_MAX).contains(&width) {
        Ok(width)
    } else {
        Err(format!("width must be between {WIDTH_MIN} and {WIDTH_MAX}"))
    }
}

fn set_input(options: &mut Options, path: PathBuf) -> Result<(), String> {
    if options.input.is_some() {
        return Err("expected at most one input file".to_string());
    }
    options.input = Some(path);
    Ok(())
}

fn read_source(path: Option<PathBuf>) -> Result<String, String> {
    match path.as_deref() {
        None => read_bounded(io::stdin().lock()),
        Some(path) if path.as_os_str() == "-" => read_bounded(io::stdin().lock()),
        Some(path) => {
            let file = File::open(path).map_err(|error| format!("{path:?}: {error}"))?;
            read_bounded(file)
        }
    }
}

fn write_stdout(contents: &str) -> Result<(), String> {
    match io::stdout().lock().write_all(contents.as_bytes()) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(error) => Err(format!("could not write output: {error}")),
    }
}

fn read_bounded(reader: impl Read) -> Result<String, String> {
    let mut bytes = Vec::new();
    reader
        .take(INPUT_BYTES_MAX + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("could not read input: {error}"))?;
    if bytes.len() as u64 > INPUT_BYTES_MAX {
        return Err(format!("input exceeds {INPUT_BYTES_MAX} bytes"));
    }
    String::from_utf8(bytes).map_err(|_| "input is not valid UTF-8".to_string())
}

fn usage() -> &'static str {
    "Usage: mermaid-ascii [OPTIONS] [FILE]\n\
Render Mermaid source from FILE, or stdin when FILE is omitted or '-'.\n\n\
Options:\n\
  -w, --width COLUMNS  Limit output width (10..1000)\n\
  -h, --help           Print help\n\
  -V, --version        Print version\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args<'a>(values: &'a [&'a str]) -> impl Iterator<Item = OsString> + 'a {
        values.iter().map(OsString::from)
    }

    #[test]
    fn parses_file_and_width() {
        assert_eq!(
            parse_args(args(&["--width", "80", "diagram.mmd"])),
            Ok(Options {
                width: Some(80),
                input: Some(PathBuf::from("diagram.mmd")),
                ..Options::default()
            })
        );
    }

    #[test]
    fn rejects_invalid_arguments() {
        assert!(parse_args(args(&["--width", "9"])).is_err());
        assert!(parse_args(args(&["--unknown"])).is_err());
        assert!(parse_args(args(&["a", "b"])).is_err());
        assert_eq!(
            parse_args(args(&["--", "--diagram.mmd"])).unwrap().input,
            Some(PathBuf::from("--diagram.mmd"))
        );
    }

    #[test]
    fn escapes_controls_in_invalid_width() {
        let error = parse_args(args(&["--width", "\u{1b}[31m"])).unwrap_err();
        assert!(!error.contains('\u{1b}'));
        assert!(error.contains("\\u{1b}"));
    }

    #[test]
    fn explicit_width_wins_over_terminal_width() {
        assert_eq!(resolve_width(Some(80), Some(120)), Some(80));
    }

    #[test]
    fn terminal_width_has_margin_and_bounds() {
        assert_eq!(resolve_width(None, Some(80)), Some(78));
        assert_eq!(resolve_width(None, Some(10)), Some(10));
        assert_eq!(resolve_width(None, Some(2)), Some(WIDTH_MIN));
        assert_eq!(resolve_width(None, Some(2000)), Some(WIDTH_MAX));
    }

    #[test]
    fn redirected_output_is_unrestricted() {
        assert_eq!(resolve_width(None, None), None);
    }

    #[test]
    fn trims_blank_edges() {
        let lines = vec![
            String::new(),
            "  ".to_string(),
            "┌─┐".to_string(),
            "└─┘".to_string(),
            String::new(),
        ];
        assert_eq!(format_output(&lines), "┌─┐\n└─┘\n");
    }

    #[test]
    fn bounds_input() {
        let oversized = vec![b'x'; INPUT_BYTES_MAX as usize + 1];
        assert!(read_bounded(oversized.as_slice()).is_err());
        assert_eq!(
            read_bounded("flowchart LR\nA-->B".as_bytes()).unwrap(),
            "flowchart LR\nA-->B"
        );
    }
}
