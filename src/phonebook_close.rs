//use std::process;
use std::thread;
use std::time::Duration;

pub fn close() {

    let handle = thread ::spawn(|| {

        println!("\nProgram is closed.");
        println!("Use: \"exit\" or \"Ctrl+D\" to close terminal.\n");

        //This function will never return and will never immediately
        //terminate the current process. So, wait until every code executed.

        thread::sleep(Duration::from_millis(1));
    });

    //process::exit(0x0100);
    handle.join().unwrap();
}
