extern crate rev_lines;
use std::io;
use std::fs;

pub fn printout(){

    // takes input from user and pass value to string named print.
    let mut print = String::new();
    println!("Enter phone number to see content: ");
    match io::stdin().read_line(&mut print){
        Ok(_) => {
            //println!("success you said: {}", input);
        },
        Err(e) => println!("error {}", e)
    }

    println!("");

    //read value of file as string and assign value called contents.
    let contents = fs::read_to_string("output.txt")
        .expect("something went wrong");

    //check if file contains taken value from user.
    if contents.contains(&print) {
        for i in contents.lines() {
            //push values to vector v, by splitting values with char ';'
            let v: Vec<&str> = i.split(";").collect();
            //check if phone number matches with taken value from user.
            if i.contains(&print.trim()) && print.trim() == v[2] {
                println!("name: {}; surname: {}; phone number: {}; \
                ", v[0], v[1], v[2]);
                break;
            }
        }
    } else {
        println!("Telephone number does not exist.");
    }
    println!("");
}
