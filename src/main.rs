use std::env;
mod phonebook_add;
mod phonebook_delete;
mod phonebook_printout;
mod phonebook_search;
mod phonebook_close;
use std::thread;
use std::time::Duration;

fn main() {

    let handle = thread ::spawn(|| {

        //get input from terminal to control the operation.
        let args: Vec<String> = env::args().collect();

        //control args[1], and perform operation.
        if args[1] == "add" {
            phonebook_add::add();
        } else if args[1] == "delete" {
            phonebook_delete::delete();
        } else if args[1] == "printout" {
            phonebook_printout::printout();
        } else if args[1] == "search" {
            phonebook_search::search();
        } else if args[1] == "close" {
            phonebook_close::close();
        } else if args[1] == "help" {
            println!("\nUsage: cargo run [command]\nUsable commands are:
add, delete, printout, search, and close.\n");
        } else {
            println!("\nProvide: cargo run help \nTo understand how to use program.\n");
        }

        thread::sleep(Duration::from_millis(1));
    });

    handle.join().unwrap();
}
