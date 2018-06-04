//! Types for the *m.room.name* event.

use serde::{Deserialize, Serialize};

state_event! {
    /// A human-friendly room name designed to be displayed to the end-user.
    pub struct NameEvent(NameEventContent) {}
}

/// The payload of a `NameEvent`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NameEventContent {
    /// The name of the room. This MUST NOT exceed 255 bytes.
    /// Rooms with `name: None` should be treated the same as a room with no name.
    // The spec says “A room with an m.room.name event with an absent, null, or empty name field
    // should be treated the same as a room with no m.room.name event.”.
    // Serde maps null fields to None by default, serde(default) maps an absent field to None,
    // and empty_string_as_none completes the handling.
    #[serde(default)]
    #[serde(deserialize_with = "empty_string_as_none")]
    pub name: Option<String>,
}

// TODO: this is a mere hack and duplicates stuff from the deserialization visitor,
// and is not generic. There are probably other fields in need of this as well, at least there is
// the CanonicalAliasEvent.
fn empty_string_as_none<'de, D>(de: D) -> Result<Option<String>, D::Error> where D: serde::Deserializer<'de> {
    let opt: Option<String> = serde::Deserialize::deserialize(de)?;
    match opt {
        None => Ok(None),
        Some(ref s) if s.is_empty() => Ok(None),
        Some(s) => Ok(Some(s)),
    }
}
