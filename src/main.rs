

use core::time;
use std::{path::PathBuf, thread::sleep, io::BufReader, fs::read, ffi::OsString, os::unix::prelude::OsStringExt};

use crate::parser::parser::Node;

mod parser;
mod io;


//#[test]
fn teste(){
    let mut ch = std::process::Command::new("/home/rc/code/projet/hs24/a.out").spawn();
    print!("owo");
    sleep(time::Duration::from_secs(5));
    let res = ch.unwrap().wait().unwrap().code().unwrap_or(-1);
    print!("{:?}",res)
}

fn main() {
    // teste()
    init_shell()
}


/// Init steps todo:
fn init_shell(){
    let mut r = read("/etc/hostname").unwrap_or_else(|x|{Vec::from([145,162,162])});
    r.pop().unwrap();

    std::env::set_var("HOSTNAME",  OsString::from_vec(r));
    main_loop()
}

/// Main loop:
/// 1- ---Print PS1---
/// 2- ---Get user input---
/// 3- ---Parse---
/// 4- Execute
/// 5- Repeat
fn main_loop(){
    //TODO 
    let mut return_code: i32 = 0;

    loop {

        io::io::print::print_ps1(return_code);
        io::io::cursor::cursor_previous_line();
        io::io::cursor::erase_line();
        
        match io::io::input::get_user_input() {
            Ok(command) => {
                print!("-> {}", &command);
                let t = parser::parser::get_tree(command);
                dbg!(&t);
                return_code = t.run();
            }
            Err(x) => { if x == 2048 {println!("Error while reading line")} else {
                println!("Error before processing line (command too long)")}
            }
        };

        //print!("|-> {}", );
    }
}