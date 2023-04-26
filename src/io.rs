

pub mod io {
    pub mod print{

        //pub fn refresh_hostname(){
        //    todo!();
        //}
        pub fn print_ps1(child_return_code: i32){
            print!("[{}] {}@{} > ",
                                    child_return_code.to_string(),
                                    std::env::var("USER").unwrap_or("err".to_string()),
                                    std::env::var("HOSTNAME").unwrap_or("err".to_string()),
                                    );
            std::io::Write::flush(&mut std::io::stdout()).unwrap_or(());
        }

    }
    pub mod input{

        pub fn get_user_input() -> Result<String, usize>{
            
            let mut buf: String = String::new();
            let r = std::io::stdin().read_line(&mut buf);
            match r {
                Err(_) => {return Err(2048);}
                Ok(_) => {if r.unwrap() < 2048 {Ok(buf)} else {Err(2049)}}
            }
        }   
    }
    pub mod cursor{
        pub fn cursor_previous_line(){
            print!("\x1b[1F");
        }
        pub fn erase_line(){
            print!("\x1b[0K");
        }
    }
}