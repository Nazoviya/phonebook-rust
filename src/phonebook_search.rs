use std::io;
use std::io::prelude::*;
use std::fs::File;


pub fn search(){

    // takes input from user and pass value to string named search.
    let mut search = String::new();
    println!("\nEnter name or surname to see the content: ");
    match io::stdin().read_line(&mut search){
        Ok(_) => {
            //println!("success you said: {}", input);
        },
        Err(e) => println!("error {}", e)
    }
    println!("");

    //open file named output.txt.
    //pass arguments into string named contents.
    let mut file = File::open("output.txt")
        .expect("unable to open file.");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("unable to read file");

    //println!("{:?}", contents.find(&print));
    //check if length of taken value less than 100 characters.
    if search.trim().len() < 100 {
        if contents.contains(&search.trim()) {
            //create vector for count number of return.
            let mut count = Vec::new();
            for i in contents.lines() {
                //push values to vector v, by splitting values with char ';'
                let v: Vec<&str> = i.split(";").collect();
                //check if name or surname matches with taken value from user.
                if i.contains(&search.trim()) {
                    println!("name: {}; surname: {}; phone number: {}; \
                    ", v[0], v[1], v[2]);
                    count.push(1);
                }
            }
            //print number of returned values in search.
            if count.len() == 1 {
                println!("\n{} result found.", count.len());
            } else {
                println!("\n{} results found.", count.len());
            }
        } else {
            println!("Name or surname does not exist or invalid.\n");
        }
    } else {
        println!("Search string must be less than 100 characters.\n");
    }
}


/*

if contents.contains(&search.trim()) {
    let mut count = 0;
    for j in contents.lines() {
        if j.contains(&search.trim()) {
            count = count + 1;
            let a = count.to_string();
            //let mut vec2 = Vec::new();
            //vec2.push(a);
        }
    }
    println!("{}", a);
}


*/


//if i.contains(&search.trim()) && (search.trim() == v[0]) || search.trim() == v[1] {


//let person: Person = Person { name: &print, surname: &print, number: &print};

// println!("-{}-{}-{}", person.name, person.surname, person.number);
