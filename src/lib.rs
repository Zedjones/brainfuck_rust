
pub mod brainfuck {

    use std::fs::File;
    use std::io::{stdin, stdout, Write, Read};
    use std::iter::repeat;

    struct Cells {
        cells: Vec<u8>,
        curr_ind: usize
    }

    impl Cells {
        fn new() -> Cells {
            let mut ret = Cells{cells: Vec::new(), curr_ind: 0};
            ret.extend();
            ret
        }
        fn extend (&mut self) {
            self.cells.extend(repeat(0).take(30000));
        }
    }

    #[derive(Debug)]
    enum BFOps {
        Add, 
        Subtract,
        MoveLeft, 
        MoveRight, 
        Write, 
        Read
    }

    impl BFOps {
        fn operation(&self, mut cell_list: &mut Cells) {
            match self {
                BFOps::Add => cell_list.cells[cell_list.curr_ind] += 1,
                BFOps::Subtract => cell_list.cells[cell_list.curr_ind] -= 1,
                BFOps::MoveLeft => cell_list.curr_ind -= 1,
                BFOps::MoveRight => cell_list.curr_ind += 1,
                BFOps::Write => print!("{}", cell_list.cells[cell_list.curr_ind]),
                BFOps::Read => {
                    let input: Option<u8> = std::io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok())
                        .map(|byte| byte as u8);
                    let val = match input {
                        Some(input) => input,
                        None => 0
                    };
                    cell_list.cells[cell_list.curr_ind] = val;
                }
            }
        }
    }

    #[derive(Debug)]
    enum BFContainer {
        Operation(BFOps),
        Loop(Vec<BFContainer>)
    }

    impl BFContainer {
        fn get_loop(&self) -> Option<&Vec<BFContainer>>{
            match &self {
                BFContainer::Operation(_) => None,
                BFContainer::Loop(res) => Some(res.clone())
            }
        }

        fn is_loop(&self) -> bool {
            match self {
                BFContainer::Operation(_) => false,
                BFContainer::Loop(_) => true
            }
        }

    }

    fn process_inner_loop(op_list: &Vec<BFContainer>, mut cell_list: &mut Cells) {
        
    }

    fn process_main_loop(op_list: &Vec<BFContainer>, mut cell_list: &mut Cells) {
        for val in op_list.iter() {
            match val {
                BFContainer::Operation(op) => op.operation(&mut cell_list),
                BFContainer::Loop(bf_loop) => process_inner_loop(&bf_loop, &mut cell_list)
            }
        }
    }

    fn construct_from_string(mut operations: &mut String) -> Result<Vec<BFContainer>, String> {
        let mut op_list = Vec::new();
        while operations.len() >= 1 {
            let c = match operations.chars().nth(0) {
                Some(c) => c,
                None => { 
                    return Err(String::from("oh no")) // if this happens we're fucked
                }
            };
            match c {
                '+' => op_list.push(BFContainer::Operation(BFOps::Add)),
                '-' => op_list.push(BFContainer::Operation(BFOps::Subtract)),
                '<' => op_list.push(BFContainer::Operation(BFOps::MoveLeft)),
                '>' => op_list.push(BFContainer::Operation(BFOps::MoveRight)),
                '.' => op_list.push(BFContainer::Operation(BFOps::Write)),
                ',' => op_list.push(BFContainer::Operation(BFOps::Read)),
                '[' => {
                    operations.remove(0);
                    match construct_from_string(&mut operations) {
                        Ok(container) => {
                            op_list.push(BFContainer::Loop(container));
                        },
                        Err(err) => return Err(err.to_string())
                    }
                },
                ']' => {
                    operations.remove(0);
                    return Ok(op_list)
                },
                _ => ()
            };
            if c != '[' && c != ']' {
                operations.remove(0);
            }
        }
        Ok(op_list)
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

    pub fn process_input_file(filename: String) -> Result<(), String> {
        let mut file = match File::open(filename) {
            Ok(val) => val,
            Err(err) => return Err(err.to_string())
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        };
        contents.retain(|c| "[]<>,.+-".contains(c));
        let res = match construct_from_string(&mut contents) {
            Ok(val) => val,
            Err(err) => panic!("{}", err.to_string())
        };
        println!("{:?}", res);
        let cells = Cells::new();
        Ok(())
    }
}