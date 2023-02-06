use server::service::Service;
use std::io::stdin;

fn main() {
    let mut short_link = String::new();

    println!("Enter your short link:");
    stdin().read_line(&mut short_link).unwrap();
    let short_link = short_link.trim_end();

    let res = Service::get_link_by_short(short_link);
    match res {
        Ok(val) => println!(
            "New link is: {:?}, for your: {:?}, date: {:?}",
            val.shared_link, val.original_link, val.created_at
        ),
        Err(_) => println!("saaad..."),
    }
}
