use clap::{Parser, Subcommand, ValueEnum};
use drawlab_core::{MAX_RECORD_BYTES, Mode, generate, match_distribution, odds, profiles, verify};
use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
    process::ExitCode,
};

#[derive(Parser)]
#[command(
    name = "drawlab",
    version,
    about = "GyLiber mathematical lottery laboratory. Not a ticket or prediction."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    /// Inspect available mathematical models and disabled official candidates.
    Games {
        #[command(subcommand)]
        command: Games,
    },
    /// Generate independently sampled selections. No remaining-pool bonus is selected.
    Generate {
        #[arg(long)]
        profile: String,
        #[arg(long, default_value_t = 1)]
        boards: usize,
        #[arg(long,value_enum,default_value_t=Format::Json)]
        format: Format,
    },
    /// Simulate mathematical draws, including applicable remaining-pool bonuses.
    SimulateDraw {
        #[arg(long)]
        profile: String,
        #[arg(long, default_value_t = 1)]
        boards: usize,
        #[arg(long,value_enum,default_value_t=Format::Json)]
        format: Format,
    },
    /// Exact combination counts and main-match probabilities; not prize payouts.
    Odds {
        #[arg(long)]
        profile: String,
    },
    /// Check structure and mathematics, not authenticity, timestamps or randomness.
    Verify { file: PathBuf },
}
#[derive(Subcommand)]
enum Games {
    List,
    Show { profile: String },
}
#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Json,
    Text,
}
fn run(cli: Cli) -> Result<String, Box<dyn std::error::Error>> {
    match cli.command {
        Command::Games {
            command: Games::List,
        } => Ok(serde_json::to_string_pretty(&profiles()?)?),
        Command::Games {
            command: Games::Show { profile },
        } => {
            let p = profiles()?
                .into_iter()
                .find(|p| p.id == profile)
                .ok_or("UNKNOWN_PROFILE")?;
            Ok(serde_json::to_string_pretty(&p)?)
        }
        Command::Generate {
            profile,
            boards,
            format,
        } => render(&profile, Mode::Selection, boards, format),
        Command::SimulateDraw {
            profile,
            boards,
            format,
        } => render(&profile, Mode::Draw, boards, format),
        Command::Odds { profile } => {
            let p = profiles()?
                .into_iter()
                .find(|p| p.id == profile)
                .ok_or("UNKNOWN_PROFILE")?;
            Ok(serde_json::to_string_pretty(
                &serde_json::json!({"profile":p.id,"exact_match":odds(&p)?,"main_matches":match_distribution(&p)?}),
            )?)
        }
        Command::Verify { file } => {
            let mut bytes = Vec::new();
            File::open(file)?
                .take((MAX_RECORD_BYTES + 1) as u64)
                .read_to_end(&mut bytes)?;
            if bytes.len() > MAX_RECORD_BYTES {
                return Err("RECORD_TOO_LARGE".into());
            }
            let record = verify(std::str::from_utf8(&bytes)?)?;
            Ok(format!(
                "Valid mathematical record: {} board(s), {}. Authenticity and randomness are not established.",
                record.boards.len(),
                record.profile.id
            ))
        }
    }
}
fn render(
    id: &str,
    mode: Mode,
    count: usize,
    format: Format,
) -> Result<String, Box<dyn std::error::Error>> {
    let r = generate(id, mode, count)?;
    match format {
        Format::Json => Ok(serde_json::to_string_pretty(&r)?),
        Format::Text => {
            let mut text = format!(
                "{}\nProfile: {} / {}\nMode: {:?}\n",
                r.label, r.profile.id, r.profile.version, r.mode
            );
            for (i, b) in r.boards.iter().enumerate() {
                text.push_str(&format!(
                    "{}: {}",
                    i + 1,
                    b.main
                        .iter()
                        .map(u8::to_string)
                        .collect::<Vec<_>>()
                        .join(" ")
                ));
                if let Some(extra) = b.extra {
                    text.push_str(&format!(" | extra: {extra}"));
                }
                text.push('\n');
            }
            Ok(text)
        }
    }
}
fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(result) => match writeln!(io::stdout().lock(), "{result}") {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) if e.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("OUTPUT_FAILED: {e}");
                ExitCode::FAILURE
            }
        },
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
