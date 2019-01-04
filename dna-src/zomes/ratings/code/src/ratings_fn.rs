use utils::{get_links_and_load_type, GetLinksLoadElement};
use hdk::{
    holochain_core_types::{
        hash::HashString,
        entry::Entry,
    },
    error::ZomeApiResult,
    api::AGENT_ADDRESS,
};
use crate::entry::{
    Ratings
};


pub fn handle_creating_ratings(rate: String, review: String, reviewed_hash: HashString ) -> ZomeApiResult<HashString> {
    let ratings_entry = Entry::App(
        "ratings".into(),
        Ratings {
            rate: rate.to_string(),
            review: review.to_string(),
            author: AGENT_ADDRESS.to_string().into(),
            timestamp: "TODO:(Add Date)".to_string(),
        }.into()
    );

    let address = hdk::commit_entry(&ratings_entry)?;
    hdk::link_entries(&reviewed_hash, &address, "rating_tag")?;
    Ok(address)
}


pub fn handle_get_reviews_by_hash(reviewed_hash: HashString) -> ZomeApiResult<Vec<GetLinksLoadElement<Ratings>>> {
    get_links_and_load_type(&reviewed_hash, "rating_tag")
}
