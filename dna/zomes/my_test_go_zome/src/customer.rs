use hdk::prelude::*;
use hdk::{map_extern::ExternResult, prelude::anchor};
use holo_hash::{EntryHash, EntryHashB64};
use crate::{utils::try_get_and_convert};

// Since we'll be using a hardcoded string value to access all customers,
// we'd better declare it as a constant to be re-used
// Note: we're using &str instead of String type here because size of this string
// is known at compile time, so there's no need to allocate memory dynamically
// by using String.
// More about &str and String difference here:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-string-type
pub const ALL_CUSTOMERS_ANCHOR: &str = "ALL_CUSTOMERS";
pub const ALL_CUSTOMERS_TAG: &str = "ALL_CUSTOMERS_TAG";

pub fn ensure_all_customers_anchor() -> ExternResult<EntryHash> {
    let anchor = anchor((ALL_CUSTOMERS_ANCHOR).into(), (ALL_CUSTOMERS_ANCHOR).into())?;
    // Note the lack of ; in the end of the next code line: this is the value we return here
    // More on that syntax here:
    // https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html#functions-with-return-values
    // Since the return value of our fn is an ExternResult, we're wrapping our
    // anchor (which is an entry hash) into the Ok() variant of ExternResult
    Ok(anchor)
}

pub fn get_all_customers_anchor() -> ExternResult<EntryHash> {
    ensure_all_customers_anchor()?;
    /* Since do not know the hash of the anchor, because only the game code is known,
    we have to calculate the hash.
    */
    let path: Path = (&Anchor {
        anchor_type: (ALL_CUSTOMERS_ANCHOR).into(),
        anchor_text: Some((ALL_CUSTOMERS_ANCHOR).into()),
    })
        .into();
    path.hash()
}


#[hdk_entry(id = "customer", visibility = "public")]
#[derive(Clone)]
pub struct Customer {
    pub first_name: String,
    pub last_name: String,
}

pub fn create_customer(first_name : String, last_name : String) -> ExternResult<EntryHashB64> {
    let customer = Customer {
        first_name,
        last_name
    };

    // commit data to DHT
    create_entry(&customer)?; // & -> no data copy
    // just calculate the hash (already stored in DHT, just for app logic)
    let customerhash = hash_entry(&customer)?;

    let all_customers_anchor = get_all_customers_anchor()?;

    create_link(all_customers_anchor.into(), customerhash.clone(), LinkTag::new(ALL_CUSTOMERS_TAG))?;

    Ok(EntryHashB64::from(customerhash))
}

pub fn get_customer(hash : EntryHash) -> ExternResult<Customer> {
    let customer : Customer = try_get_and_convert(hash, GetOptions::latest())?;
    Ok(customer)
}

pub fn get_all_customers() -> ExternResult<Vec<Customer>> {

    let all_customers_anchor = get_all_customers_anchor()?;
    let links: Links = get_links(all_customers_anchor, Some(LinkTag::new(String::from(ALL_CUSTOMERS_TAG))))?;

    let mut customers = vec![];

    for link in links.into_inner() {
        // Retrieve an element at the hash specified by link.target
        // No fancy retrieve options are applied, so we just go with GetOptions::default()
        let element: Element = get(link.target, GetOptions::default())?.ok_or(WasmError::Guest(String::from("Customer entry not found")))?;
        // Retrieve an Option with our entry inside. Since not all Elements can have
        // entry, their method `entry()` returns an Option which would be None in case
        // the corresponding Element is something different.
        let entry_option = element.entry().to_app_option()?;
        // Now try to unpack the option that we received and write an error to show
        // in case it turns out there's no entry
        let entry: Customer = entry_option.ok_or(WasmError::Guest(
            "The targeted entry is not a customer".into(),
        ))?;

        customers.push(entry);
    }

    Ok(customers)
}