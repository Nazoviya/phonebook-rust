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


//let len = print.trim().len();
//let final_str = &print[len-1..];
//println!("{}", final_str);
//let mystring = print.chars().last().unwrap();

//let ch = print.chars().nth(0).unwrap();
//let mm = &print.trim();

//let end = print.trim().chars().last().unwrap();


/*


if contents.contains(&print) {
    for i in contents.lines() {
        if i.contains(&print.trim()) {
            println!("{}", &i);
        }
    }
} else {
    println!("Telephone number does not exist.");
}


*/



//println!("{}", print);

//let mut vec = Vec::new();
//vec.push(&contents);
//contents.lines();

//println!("{}",vec[0]);

/*
    if contents.contains(&print) {
        for i in contents.lines() {
            if contents.contains(&print) {
                println!("{}",i);
            }
        }

*/

        //format!("{}", &print, );

        //println!("{}", print);
/*
        let log_file = File::open("output.txt").unwrap();
        //let reader = BufReader::new(file);
        let rev_lines = RevLines::new(BufReader::new(log_file)).unwrap();

        //let text = "fe";
        let start = line_span::find_line_start(&contents, 0);
        let end = line_span::find_line_end(&contents, 0);
        let len = &print.len();
        let final_str = &print[len-1..];
        //let start_prev = line_span::find_prev_line_start(&print, 0).unwrap();

        //println!("{}", &print[..end]);
        println!("{}", &contents[start..]);
*/
        /*
        for span in contents.line_spans() {
            println!("{:?}", span.range());
        }
        */
/*
        let n_last_lines = 5;
        let mut lines: Vec<String> = Vec::new();
        let mut counter = 0;
        for i in rev_lines {
            if counter >= n_last_lines {
                break;
            }
            counter += 1;
            lines.push(i);
        }

        for j in lines.iter().rev() {
            println!("{}", j);
        }
*/



        /*
        for(index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            println!("{}-) {}", index + 1, line);
            if let ch = line.chars().nth(0)..unwrap(){
                println!("{}", line);
            }

        */


        //println!("{}", print);
