
pub mod brainfuck {

    #[derive(Debug)]
    enum BFOps {
        Add, 
        Subtract,
        MoveLeft, 
        MoveRight, 
        Write, 
        Read
    }

    #[derive(Debug)]
    enum BFContainer {
        Operation(BFOps),
        Loop(Vec<BFContainer>)
    }

    use std::fs::File;
    use std::io::{stdin, stdout, Write, Read};

    fn construct_from_string(mut operations: &mut str) -> Result<BFContainer, String> {
        let mut op_list = Vec::new();
        while operations.len() >= 1 {
            let c = match operations.chars().nth(0) {
                Some(c) => c,
                None => return Err(String::from("oh no")) // if this happens we're fucked
            };
            match c {
                '+' => op_list.push(BFContainer::Operation(BFOps::Add)),
                '-' => op_list.push(BFContainer::Operation(BFOps::Subtract)),
                '<' => op_list.push(BFContainer::Operation(BFOps::MoveLeft)),
                '>' => op_list.push(BFContainer::Operation(BFOps::MoveRight)),
                '.' => op_list.push(BFContainer::Operation(BFOps::Write)),
                ',' => op_list.push(BFContainer::Operation(BFOps::Read)),
                '[' => {
                        operations = &mut operations[1..];
                        match construct_from_string(&mut operations) {
                        Ok(container) => op_list.push(container),
                        Err(err) => return Err(err.to_string())
                    }
                },
                ']' => {
                    operations = &mut operations[1..];
                    return Ok(BFContainer::Loop(op_list))
                },
                _ => ()
            };
            operations = &mut operations[1..];
        }
        Ok(BFContainer::Loop(op_list))
    }

    pub fn process_input() -> Result<(), std::io::Error> {
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

    pub fn process_input_file(filename: String) -> Result<(), std::io::Error> {
        let mut file = match File::open(filename) {
            Ok(val) => val,
            Err(err) => return Err(err)
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(err) => return Err(err)
        };
        let res = match construct_from_string(&mut contents) {
            Ok(val) => val,
            Err(err) => panic!("{}", err.to_string())
        };
        println!("{:?}", res);
        Ok(())
    }
}