use std::io;
use std::fs;
use std::io::{Write};
use std::fs::OpenOptions;

pub fn delete(){

    //read every value from file and pass into string named contents.
    let contents = fs::read_to_string("output.txt")
        .expect("something went wrong");

    //println!("{}",contents);

    //takes input from user to delete given phone number.
    let mut delete_content = String::new();
    println!("Enter phone number to delete: ");
    match io::stdin().read_line(&mut delete_content){
        Ok(_) => {
            //println!("success you said: {}", input);
        },
        Err(e) => println!("error {}", e)
    }



    //check if value that provided from user contained in file.
    if contents.contains(&delete_content) {
        for i in contents.lines() {
            //push values to vector v, by splitting values with char ';'
            let v: Vec<&str> = i.split(";").collect();
            if !(i.contains(&delete_content.trim()) && delete_content.trim() == v[2]) {
                //println!("{}", i.trim().to_owned());
                let last = i.to_owned() + "\n";

                //println!("{}", i);
                //create another file named file2, to store new values.
                let mut file2 = OpenOptions::new().append(true).create(true).open("output2.txt")
                    .expect("fail");

                //write values into file named file2.
                file2.write_all(last.as_bytes())
                    .expect("fail");
            }
        }
        //rename correct file with output.txt, so we can
        //use every value in program correctly.
        fs::rename("output2.txt", "output.txt")
            .expect("fail");

        println!("\n-Content deleted successfully.\n");

    } else {
        println!("\n-Telephone number couldn't found.\n");
    }
}
