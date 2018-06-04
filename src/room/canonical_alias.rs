//! Types for the *m.room.canonical_alias* event.

use ruma_identifiers::RoomAliasId;
use serde;
use std::convert::TryFrom;

state_event! {
    /// Informs the room as to which alias is the canonical one.
    pub struct CanonicalAliasEvent(CanonicalAliasEventContent) {}
}

/// The payload of a `CanonicalAliasEvent`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CanonicalAliasEventContent {
    /// The canonical alias.
    #[serde(default)]
    #[serde(deserialize_with = "empty_string_as_none")]
    pub alias: Option<RoomAliasId>,
}

// TODO: this is a mere hack and duplicates stuff from the deserialization visitor,
// and is not generic. There are probably other fields in need of this as well.
fn empty_string_as_none<'de, D>(de: D) -> Result<Option<RoomAliasId>, D::Error> where D: serde::Deserializer<'de> {
    let opt: Option<String> = serde::Deserialize::deserialize(de)?;
    match opt {
        None => Ok(None),
        Some(ref s) if s.is_empty() => Ok(None),
        Some(s) => match RoomAliasId::try_from(s.as_str()) {
            Ok(room_alias_id) => Ok(Some(room_alias_id)),
            Err(_) => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(s.as_str()), &"a valid RoomAliasId")),
        }
    }
}
