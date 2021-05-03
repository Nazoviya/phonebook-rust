use std::io;
use std::fs::File;
use std::io::Write;
use std::fs;
use std::fs::OpenOptions;
use std::io::ErrorKind;

pub fn add(){
    //takes name as string from user and assign to value called input.
    let mut input = String::new();
    println!("Enter your name: ");
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            //println!("success you said: {}", input);
        },
        Err(e) => println!("error {}", e)
    }
    //takes surname as string from user and assign to value called input2.
    let mut input2 = String::new();
    println!("Enter your Surname: ");
    match io::stdin().read_line(&mut input2){
        Ok(_) => {
        },
        Err(e) => println!("error {}", e)
    }
    //takes phone number as string from user and assign to value called input3.
    let mut input3 = String::new();
    println!("Enter your Phone number: ");
    match io::stdin().read_line(&mut input3){
        Ok(_) => {
        },
        Err(e) => println!("error {}", e)
    }

    //open file. if it does not exists, create.
    File::open("output.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("output.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //read value of file as string and assign value called contents.
    let mut contents = fs::read_to_string("output.txt")
        .expect("something went wrong");

    contents.push_str(" ");
    //put value -------PHONEBOOK------- into first line of file.
    if contents.trim().len() == 0 {
        let mut file = OpenOptions::new().append(true).open("output.txt")
            .expect("cannot open file");
        file.write_all("-------PHONEBOOK-------\n".as_bytes())
            .expect("fail");
    }

    //check if character sizes are ok.
    if input.len() < 100 && input2.len()< 100 && input3.len() < 30 {
        print!("\n-{}-{}-{}", input, input2, input3);

        //check if the given phone number is given already.
        //if !(same.contains(&input3)) {
        if !(contents.contains(&input3)) {
            let mut file = OpenOptions::new().append(true).open("output.txt")
                .expect("cannot open file");

            //push values to string called last, which are provided
            //from user as name, surname, and phone number.
            let last = String::from("");
            let last = last + &input.trim() + ";" + &input2.trim() + ";\
            " + &input3.trim() + "\n";

            //write content of values which are passed from if line.
            file.write_all(last.as_bytes())
                .expect("fail");

            println!("Succesfully added!");

        } else {
            print!("\nERROR!\nThis phone number is already exists, or invalid.\n");
        }
    } else {
        println!("\nERROR! name should be < 100, surname should be < 100,
and telephone number should < 30 characters.");
    }
}
