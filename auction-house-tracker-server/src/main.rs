#![recursion_limit = "128"]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

mod auction;
mod config;
mod db;
mod item;

use config::Config;
use std::thread;
use std::time::Duration;

fn main() {
    let c = config::load_config();
    loop {
        update_auctions(&c);
        thread::sleep(Duration::from_secs(3600));
    }
}

fn update_auctions(c: &Config) {
    let conn = db::establish_connection();
    let (url, time) = auction::get_auction_data_url(&c.bnet_key, &c.realm);
    let auctions = auction::get_auction_data(&url, time);

    let missing_auctions = auction::get_missing_auctions(&conn, &auctions);
    let ids = item::get_item_ids_from_auctions(&missing_auctions);
    let missing_ids = item::get_missing_ids(&conn, &ids);

    let items = item::get_items_threaded(&c.bnet_key, missing_ids);
    item::insert_items(&conn, &items);
    auction::insert_auctions(&conn, &missing_auctions);
    println!("Auctions updated");
}
