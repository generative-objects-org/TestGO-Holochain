use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

mod customer;
mod utils;


entry_defs![
    // Our implementation of game_code uses `anchor` helper method,
    // which requires us to add the Anchor and Path entry definitions
    Anchor::entry_def(),
    Path::entry_def(),
    customer::Customer::entry_def()
];

#[hdk_extern]
pub fn create_customer(customer : customer::Customer) -> ExternResult<EntryHashB64> {
    customer::create_customer(customer.first_name, customer.last_name)
}

#[hdk_extern]
pub fn get_customer(hash : EntryHash) -> ExternResult<customer::Customer> {
    customer::get_customer(hash)
}

#[hdk_extern]
pub fn get_all_customers(_: ()) -> ExternResult<Vec<customer::Customer>> {
    customer::get_all_customers()
}

