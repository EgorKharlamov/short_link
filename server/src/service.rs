pub use crate::config::Config;
use crate::models::{Link, NewLink};
use crate::repository::{create_link, get_link_by_original, get_link_by_short, remove_old_links};

use chrono::Utc;
use diesel::{Connection, PgConnection};
use nanoid::nanoid;

pub struct Service {
    pub conn: PgConnection,
}

impl Service {
    pub fn default() -> Self {
        let config = Config::default();
        let conn = PgConnection::establish(config.database_url.as_str())
            .unwrap_or_else(|_| panic!("Error connecting to {}", config.database_url));
        Service { conn }
    }

    pub fn create_link(original: &str) -> Link {
        match get_link_by_original(original) {
            Ok(val) => val,
            Err(_) => {
                let new_link = NewLink {
                    shared_link: nanoid!(4),
                    original_link: original,
                    created_at: &Utc::now().naive_utc(),
                };
                create_link(new_link)
            }
        }
    }

    pub fn get_link_by_short(short_link: &str) -> Result<Link, &str> {
        match get_link_by_short(&short_link) {
            Ok(val) => Ok(val),
            Err(_) => Err("No such links!"),
        }
    }

    pub fn remove_old_links() {
        match remove_old_links(1) {
            Ok(val) => Ok(val),
            Err(_) => Err("Something wrong db..."),
        }
        .expect("Something wrong db...");
    }
}
