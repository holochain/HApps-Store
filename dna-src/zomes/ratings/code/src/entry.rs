/// This file holds everything that represents the "post" entry type.

use hdk::holochain_core_types::{
    // cas::content::Address,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
};
// use boolinator::*;
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

/// We declare the structure of our entry type with this Rust struct.
/// It will be checked automatically by the macro below, similar
/// to how this happens with functions parameters and zome_functions!.
///
/// So this is our normative schema definition:
#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Ratings {
    pub rate: String,
    pub review: String,
    pub author: String,
    pub timestamp: String,
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "ratings",
        description: "Ratings for a given hash",
        sharing: Sharing::Public,
        native_type: Ratings,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_ratings: Ratings, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}
