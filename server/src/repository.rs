use crate::models::{Link, NewLink};
use crate::schema::links::dsl::links;
use crate::schema::links::{created_at, original_link, shared_link};
use crate::service::Service;
use diesel::dsl::{now, IntervalDsl};
use diesel::prelude::*;
use std::error::Error;

pub fn create_link(new_link: NewLink) -> Link {
    use crate::schema::links;
    let mut service = Service::default();
    let new_link = diesel::insert_into(links::table)
        .values(new_link)
        .get_result(&mut service.conn)
        .expect("Error saving new link");
    new_link
}

pub fn get_link_by_short(short_link: &str) -> Result<Link, Box<dyn Error>> {
    let mut service = Service::default();
    let link = links
        .filter(shared_link.eq(short_link))
        .get_result::<Link>(&mut service.conn)?;
    Ok(link)
}

pub fn get_link_by_original(original: &str) -> Result<Link, Box<dyn Error>> {
    let mut service = Service::default();
    let link = links
        .filter(original_link.eq(original))
        .get_result::<Link>(&mut service.conn)?;
    Ok(link)
}

pub fn remove_old_links(life: i32) -> Result<Vec<Link>, Box<dyn Error>> {
    let mut service = Service::default();
    let _ = diesel::delete(links.filter(created_at.lt(now - life.days())))
        .execute(&mut service.conn)
        .expect("Something wrong db...");
    let res = links
        .filter(created_at.ge(now - life.days()))
        .get_results::<Link>(&mut service.conn)?;
    Ok(res)
}
