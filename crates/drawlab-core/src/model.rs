use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(pub &'static str);
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for Error {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Selection,
    Draw,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Laboratory,
    Unverified,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Extra {
    None,
    Remaining,
    Independent { pool: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub id: String,
    pub version: String,
    pub name: String,
    pub pool: u8,
    pub pick: u8,
    pub extra: Extra,
    pub status: Status,
    pub source: String,
}
impl Profile {
    pub fn validate(&self) -> Result<(), Error> {
        if self.pool == 0 || self.pool > 64 || self.pick == 0 || self.pick > self.pool {
            return Err(Error("INVALID_PROFILE: require 1 <= pick <= pool <= 64"));
        }
        match self.extra {
            Extra::Remaining if self.pick == self.pool => {
                return Err(Error("INVALID_PROFILE: no remaining bonus"));
            }
            Extra::Independent { pool } if pool == 0 || pool > 64 => {
                return Err(Error("INVALID_PROFILE: invalid independent pool"));
            }
            _ => {}
        }
        Ok(())
    }
    pub fn enabled(&self) -> Result<(), Error> {
        self.validate()?;
        if self.status != Status::Laboratory {
            return Err(Error("RULES_UNVERIFIED: official profile is disabled"));
        }
        Ok(())
    }
    /// SHA-256 of compact serde JSON in this struct's declared field order.
    pub fn digest(&self) -> Result<String, Error> {
        use sha2::{Digest, Sha256};
        let bytes = serde_json::to_vec(self).map_err(|_| Error("SERIALIZATION_FAILED"))?;
        Ok(format!("{:x}", Sha256::digest(bytes)))
    }
}

pub fn profiles() -> Result<Vec<Profile>, Error> {
    let items: Vec<Profile> = serde_json::from_str(include_str!("../../../rules/profiles.json"))
        .map_err(|_| Error("INVALID_REGISTRY"))?;
    let mut ids = std::collections::BTreeSet::new();
    for p in &items {
        p.validate()?;
        if !ids.insert(&p.id) {
            return Err(Error("INVALID_REGISTRY: duplicate id"));
        }
    }
    Ok(items)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Board {
    /// Sorted for display. Sampling order is not claimed to be retained.
    pub main: Vec<u8>,
    pub extra: Option<u8>,
}
impl Board {
    pub fn validate(&self, p: &Profile, mode: Mode) -> Result<(), Error> {
        p.validate()?;
        if self.main.len() != usize::from(p.pick)
            || self.main.iter().any(|&v| v == 0 || v > p.pool)
            || self.main.windows(2).any(|w| w[0] >= w[1])
        {
            return Err(Error(
                "INVALID_RECORD: wrong main count, range, order or uniqueness",
            ));
        }
        let valid = match (p.extra, mode, self.extra) {
            (Extra::None, _, None) | (Extra::Remaining, Mode::Selection, None) => true,
            (Extra::Remaining, Mode::Draw, Some(v)) => {
                v > 0 && v <= p.pool && !self.main.contains(&v)
            }
            (Extra::Independent { pool }, _, Some(v)) => v > 0 && v <= pool,
            _ => false,
        };
        if !valid {
            return Err(Error("INVALID_RECORD: incorrect extra ball"));
        }
        Ok(())
    }
}
