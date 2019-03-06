
pub mod brainfuck {

    use std::fs::File;
    use std::io::stdin;

    pub fn process_input() {
        let mut line = String::new();
        let _incomplete_loop = false;
        print!("> ");
        loop {
            let res = stdin().read_line(&mut line);
            match res {
                Ok(0) => break,
                Ok(_) => (),
                Err(e) => panic!("Failed to read from stdin: {}", e)
            }
            println!("{}", line);
            line.clear();
            print!("> ");
        }
    }
}