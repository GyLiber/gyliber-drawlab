//! One implementation for native and browser laboratory generation.
//! Official-game candidates fail closed until rule evidence is reviewed.
mod math;
mod model;
mod random;
mod record;

pub use math::{at_least_matches, combinations, match_distribution, match_moments, odds};
pub use model::{Board, Error, Extra, Mode, Profile, Status, profiles};
pub use record::{MAX_RECORD_BYTES, Record, generate, verify};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
