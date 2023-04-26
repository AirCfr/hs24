

use crate::parser::parser::Node;

mod parser;
mod io;


//#[test]
// fn teste(){
    // print!("\x1b[1F");
// }

fn main() {
    // teste()
    init_shell()
}


/// Init steps todo:
fn init_shell(){
    //TODO
    std::env::set_var("HOSTNAME", "archlinux");
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
    let return_code: i32 = 0;

    loop {

        io::io::print::print_ps1(return_code);
        io::io::cursor::cursor_previous_line();
        io::io::cursor::erase_line();
        
        match io::io::input::get_user_input() {
            Ok(command) => {
                print!("-> {}", &command);
                let t = parser::parser::get_tree(command);
                dbg!(&t);
                t.run()
            }
            Err(x) => { if x == 2048 {println!("Error while reading line")} else {
                println!("Error before processing line (command too long)")}
            }
        };

        //print!("|-> {}", );
    }
}