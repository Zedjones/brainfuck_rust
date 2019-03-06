
pub mod brainfuck {

    use std::fs::File;
    use std::io::{stdin, stdout, Write};

    pub fn process_input() -> Result<(), std::io::Error>{
        let mut line = String::new();
        let _incomplete_loop = false;
        print!("> ");
        match stdout().flush() {
            Ok(_) => (),
            Err(err) => return Err(err)
        };
        loop {
            let res = stdin().read_line(&mut line);
            match res {
                Ok(0) => break,
                Ok(_) => (),
                Err(e) => return Err(e)
            }
            line = line.trim().to_string();
            println!("{}", line);
            line.clear();
            print!("> ");
            match stdout().flush() {
                Ok(_) => (),
                Err(err) => return Err(err)
            };
        }
        Ok(())
    }
}