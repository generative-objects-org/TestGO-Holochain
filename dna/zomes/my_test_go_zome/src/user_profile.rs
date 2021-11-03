use hdk::prelude::*;

#[hdk_entry(id = "user_profile", visibility = "public")]
pub struct UserProfile {
    pub user_id: AgentPubKey,
    pub nickname: String,
}

pub fn create_and_hash_entry_player_profile(nickname : String) -> ExternResult<EntryHash> {
    let agent = agent_info()?;
    let user_profile = UserProfile {
        user_id : agent.agent_initial_pubkey, // hack, not best decision , key can change
        nickname
    };

    // commit data to DHT
    create_entry(&user_profile)?; // & -> no data copy
    // just calculate the hash (already stored in DHT, just for app logic)
    hash_entry(&user_profile)
}