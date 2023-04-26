pub mod parser {
    use std::{os::unix::process::CommandExt};

    pub trait Node {
        fn new_from_string(s: String) -> Self;
        fn run(&self){}
    }

    #[derive(Debug)]
    pub struct Tree {
        main: Vec<Instruction>
    }

    impl Node for Tree {
        fn new_from_string(s: String) -> Tree {
            Tree {main: s.replace("\n", "").replace(";   ", ";").replace(";  ", ";").replace("; ", ";").split(';').map(|str| {Instruction::new_from_string(str.to_string())}).collect()}
        }
        fn run(&self) -> () {
            for instr in self.main.as_slice() {
                instr.run()
            }
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
        fn run(&self) {
            if self.command.len() != 0 {
                let mut path1 = String::from("/bin/");
                for c in self.command.split("") {
                    path1.push_str(c);
                }
                
                std::process::Command::new(&path1[..]).exec();
            }
        }
    }

    pub fn get_tree(s: String) -> Tree {
        Tree::new_from_string(s)
    }
}