use diesel::prelude::*;
use server::models::Link;
use server::schema::links::dsl::links;
use server::service::Service;

fn main() {
    let mut service = Service::default();
    let results = links
        .limit(5)
        .load::<Link>(&mut service.conn)
        .expect("Error loading links");

    println!("Displaying {} links", results.len());
    for link in results {
        println!("original {}", link.original_link);
        println!("shared {}", link.shared_link);
        println!("create_at {}", link.created_at);
        println!("-----------\n");
    }
}
