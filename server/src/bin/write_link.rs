use server::service::Service;
use std::io::stdin;

fn main() {
    let mut original_link = String::new();

    println!("Enter the link you want to short:");
    stdin().read_line(&mut original_link).unwrap();
    let original_link = original_link.trim_end(); // Remove the trailing newline

    let new_link = Service::create_link(original_link);
    println!(
        "New link is: {:?}, for your : {:?}",
        new_link.shared_link, new_link.original_link
    );
}
