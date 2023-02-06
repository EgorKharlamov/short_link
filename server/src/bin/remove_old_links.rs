use server::service::Service;
use std::io::stdin;

fn main() {
    Service::remove_old_links();
}
