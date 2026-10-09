use crate::random::{Platform, sample};
use crate::{Board, Error, Mode, Profile, VERSION, profiles};
use serde::{Deserialize, Serialize};

pub const MAX_RECORD_BYTES: usize = 1_048_576;
const LABEL: &str = "Laboratory simulation — not a ticket, official result or prediction.";
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub schema_version: u8,
    pub software_version: String,
    pub profile: Profile,
    pub profile_digest: String,
    pub mode: Mode,
    pub entropy_backend: String,
    pub label: String,
    pub boards: Vec<Board>,
}
fn profile(id: &str) -> Result<Profile, Error> {
    profiles()?
        .into_iter()
        .find(|p| p.id == id)
        .ok_or(Error("UNKNOWN_PROFILE"))
}
pub fn generate(id: &str, mode: Mode, count: usize) -> Result<Record, Error> {
    let p = profile(id)?;
    p.enabled()?;
    if !(1..=100).contains(&count) {
        return Err(Error("INVALID_REQUEST: boards must be 1..100"));
    }
    let boards = (0..count)
        .map(|_| sample(&p, mode, &mut Platform))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Record {
        schema_version: 1,
        software_version: VERSION.into(),
        profile_digest: p.digest()?,
        profile: p,
        mode,
        entropy_backend: if cfg!(target_arch = "wasm32") {
            "web_crypto"
        } else {
            "operating_system"
        }
        .into(),
        label: LABEL.into(),
        boards,
    })
}
/// Structural and mathematical validation only, not authenticity or randomness proof.
pub fn verify(json: &str) -> Result<Record, Error> {
    if json.len() > MAX_RECORD_BYTES {
        return Err(Error("RECORD_TOO_LARGE"));
    }
    let r: Record = serde_json::from_str(json)
        .map_err(|_| Error("INVALID_RECORD: malformed or unsupported JSON"))?;
    let trusted = profile(&r.profile.id)?;
    trusted.enabled()?;
    if r.schema_version != 1
        || r.software_version != VERSION
        || r.profile != trusted
        || r.profile_digest != trusted.digest()?
        || r.label != LABEL
        || !["web_crypto", "operating_system"].contains(&r.entropy_backend.as_str())
        || !(1..=100).contains(&r.boards.len())
    {
        return Err(Error("INVALID_RECORD: identity, schema or bounds mismatch"));
    }
    for b in &r.boards {
        b.validate(&trusted, r.mode)?;
    }
    Ok(r)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generation_roundtrip_and_all_profile_modes() {
        for p in profiles().unwrap() {
            for mode in [Mode::Selection, Mode::Draw] {
                let r = generate(&p.id, mode, 100);
                if p.enabled().is_err() {
                    assert!(r.is_err());
                    continue;
                }
                let r = r.unwrap();
                verify(&serde_json::to_string(&r).unwrap()).unwrap();
                for b in &r.boards {
                    b.validate(&p, mode).unwrap();
                }
            }
        }
    }
    #[test]
    fn rejects_identity_mutations_and_invalid_bounds() {
        assert!(generate("lab-5of36", Mode::Selection, 0).is_err());
        assert!(generate("lab-5of36", Mode::Draw, 101).is_err());
        assert!(generate("missing", Mode::Draw, 1).is_err());
        let r = generate("lab-5of36", Mode::Draw, 1).unwrap();
        let original = serde_json::to_value(&r).unwrap();
        for field in [
            "schema_version",
            "software_version",
            "profile_digest",
            "entropy_backend",
            "label",
            "boards",
        ] {
            let mut altered = original.clone();
            altered[field] = serde_json::Value::Null;
            assert!(verify(&altered.to_string()).is_err());
        }
        let mut altered = original.clone();
        altered["profile"]["pool"] = 37.into();
        assert!(verify(&altered.to_string()).is_err());
        let mut altered = original;
        altered["boards"][0]["main"] = serde_json::json!([1, 1, 2, 3, 4]);
        assert!(verify(&altered.to_string()).is_err());
        assert!(verify(&" ".repeat(MAX_RECORD_BYTES + 1)).is_err());
        assert!(verify("{\"schema_version\":1,\"schema_version\":1}").is_err());
    }
}
