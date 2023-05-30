pub mod parser {
    use std::{os::unix::process::CommandExt};

    pub trait Node {
        fn new_from_string(s: String) -> Self;
        fn run(&self) -> i32;
    }

    #[derive(Debug)]
    pub struct Tree {
        main: Vec<Instruction>
    }

    impl Node for Tree {
        fn new_from_string(s: String) -> Tree {
            Tree {main: s.replace("\n", "").replace(";   ", ";").replace(";  ", ";").replace("; ", ";").split(';').map(|str| {Instruction::new_from_string(str.to_string())}).collect()}
        }
        fn run(&self) -> i32 {
            let mut res:i32 = 0; 
            for instr in self.main.as_slice() {
                res = instr.run()
            }
            return res;
        }
    }

    #[derive(Debug)]
    pub struct Instruction {
        command: String,
        args: String
    }

    impl Node for Instruction {
        fn new_from_string(s: String) -> Instruction {
            let r = match s.split_once(' ') {
                Some((a,b)) => (a,b),
                _ => (&s[..], "")
            };
            Instruction { command: r.0.to_string(), args: r.1.to_string() }
        }
        fn run(&self) -> i32{
            if self.command.len() != 0 {
                let mut bin_path = String::from("/bin/");
                for c in self.command.split("") {
                    bin_path.push_str(c);
                }
                if std::path::Path::new(&bin_path).exists() {
                    // return match std::process::Command::new(&bin_path[..]).spawn() {
                        // Ok(mut x) => {match x.wait(){
                            // Ok(y) => y.code().unwrap_or(-1),
                            // _ => -1
                        // }},
                        // _ => -1
                    // }

                    return match std::process::Command::new(&bin_path[..]).args(self.args.split_whitespace()).spawn() {
                        Ok(mut x) => {match x.wait(){
                            Ok(y) => y.code().unwrap_or(-1),
                            _ => -1
                        }},
                        _ => -1
                    }


                }

                // le cursed-o-meter s'affole
                let mut rel_path = std::env::current_dir().unwrap_or_else(|_x| std::path::PathBuf::new()).as_os_str().to_str().unwrap().to_string();
                for c in self.command.replacen("./", "/", 1) .split("") {
                    rel_path.push_str(c);
                }
                //print!("{}",rel_path);
                if std::path::Path::new(&rel_path).exists() {
                    return match std::process::Command::new(&rel_path[..]).args(self.args.split_whitespace()).spawn() {
                        Ok(mut x) => {match x.wait(){
                            Ok(y) => y.code().unwrap_or(-1),
                            _ => -1
                        }},
                        _ => -1
                    }
                }
                -1
            }
            else {
                -1
            }
        }
    }

    pub fn get_tree(s: String) -> Tree {
        Tree::new_from_string(s)
    }
}