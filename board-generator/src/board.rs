

pub mod board {

    use crate::square::square::Square;
    use crate::help::help;
    use crate::difficulty::difficulty::Difficulty;

    use rusqlite::{params, Connection, Result, OpenFlags};
    use futures::executor::block_on;
    use time::PreciseTime;
    use rand::Rng;
    use rand::thread_rng;
    use rand::seq::SliceRandom;
    use std::{fs::{ OpenOptions, File, canonicalize }, io::prelude::*, time::{SystemTime, UNIX_EPOCH}};
    use std::io::{BufWriter, Write};
    use rand::prelude::*;
    use std::collections::HashMap;
    use std::convert::TryInto;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug)]
    pub struct Board {
        pub squares: Vec<Square>,
        pub solution: Vec<Square>,
        pub difficulty: Difficulty,
        pub timestamp: SystemTime,
    }
    
    impl Default for Board {
        #[tokio::main]
        async fn default() -> Board {
            let mut squares: Vec<Square> = Vec::new();
            let mut solution: Vec<Square> = Vec::new();

            for x in 0..9 {
                for y in 0..9 {
                    squares.push(Square {
                        x,
                        y,
                        value: 0,
                        candidates: Vec::new(),
                    });
                }   
            }
            
            let mut board = Board { squares: squares, solution: solution, difficulty: Difficulty::MEDIUM(), timestamp: SystemTime::now()};

            loop {
                board.create_board(0);

                board.solution = board.squares.clone();
                
                let result: bool = board.remove_squares();
                if result {
                    break;
                }
            }

            return board;
        }
    }
    
    impl Board {
        pub fn custom(difficulty: Difficulty) -> Board {
            let mut squares: Vec<Square> = Vec::new();
            let mut solution: Vec<Square> = Vec::new();

            for x in 0..9 {
                for y in 0..9 {
                    squares.push(Square {
                        x,
                        y,
                        value: 0,
                        candidates: Vec::new(),
                    });
                }   
            }
            
            let mut board = Board { squares: squares, solution: solution, difficulty: difficulty, timestamp: SystemTime::now()};

            loop {
                board.create_board(0);

                board.solution = board.squares.clone();
                
                let result: bool = board.remove_squares();
                if result {
                    break;
                }
            }

            return board;  
        }

        pub fn print(&self) {
            help::print_vec(&self.squares);
        }

        pub fn export_json(&self) -> Result<()> {
            let mut f = OpenOptions::new()
            .write(true)
            .open("C:/Users/frogg/Desktop/code/Placeholdr/frontend/src/test.json")
            .expect("unable to open file");
            
            let j = serde_json::to_string(&self.squares).unwrap();
            write!(f, "{:?}", j);

            Ok(())
        }
    
        pub fn save_db(&self) -> Result<()> {
            let mut path = "".to_string();

            match self.timestamp.duration_since(UNIX_EPOCH) {
                Ok(n) => path = format!("./db.db"),
                Err(_) => panic!("Negative time"),
            }

            let conn = Connection::open(path).expect("Could not connect");
            conn.execute(
                "CREATE TABLE IF NOT EXISTS boards (
                    id INT AUTO INCREMENT PRIMARY KEY,
                    unsolved TEXT UNIQUE,
                    solved TEXT,
                    difficulty TEXT
                )", [],
            );

            conn.execute(
                "INSERT INTO boards (unsolved, solved, difficulty) VALUES (?1, ?2, ?3)",
                params![help::squares_to_string(&self.squares), help::squares_to_string(&self.solution), self.difficulty.name],
            );

            Ok(())
        }

        pub fn save_text(&self)     {
            let f = OpenOptions::new()
            .write(true)
            .append(true)
            .open("test.txt")
            .expect("unable to open file");
            
            let mut f = BufWriter::new(f);
            
            write!(f, "{}{},", help::squares_to_string(&self.squares), help::squares_to_string(&self.solution));
        }

        pub fn print_candidates(&self) {
            for square in self.squares.iter() {
                println!("\nINDEX:({},{})", square.x, square.y);
                for candidate in square.candidates.iter() {
                    println!(" {}", candidate);
                }
            }
        }

        pub fn refresh_candidates(&mut self) {
            for index in 0..self.squares.len() {
                let mut candidates: Vec<i32> = Vec::new();

                for option in 1..10 {
                    let future = help::isValid(&self.squares, index, option);
                    if block_on(future) {
                        candidates.push(option);
                    }
                    
                }
                self.squares[index].candidates = candidates;
            }

        }

        fn create_board(&mut self, index: usize) -> bool {
            let mut square_options: Vec<i32> = (1..10).collect();
            let mut rng = StdRng::from_entropy();
            square_options.shuffle(&mut rng);  
        
            if index == self.squares.len() {
                return true;
            }
            for option in square_options.iter() {
                let future = help::isValid(&self.squares, index, *option);
                if block_on(future) {
                    self.squares[index].value = *option;
                    if self.create_board(index+1) {
                        return true;
                    } else {
                        self.squares[index].value = 0;
                    }
                }
            }
            return false;
        }

        fn remove_squares(&mut self) -> bool {
            let random_index: Vec<usize> = find_random_filled_squares(&self.squares);
    
            let mut option: i32 = 0;
            let mut solutions: i32 = 0;
            
            if random_index.len() < self.difficulty.filled_squares_count.try_into().unwrap()     {
                return true;
            } 
    
            for index in random_index.iter() {
                option = self.squares[*index].value;
    
                self.squares[*index].value = 0;
    
                if !one_solution(&mut self.squares) {
                    self.squares[*index].value = option;
                    continue
                }
                
                if self.remove_squares() {
                    return true;
                } else {
                    self.squares[*index].value = option;
                }
    
            }
            return false;
        }
    }
        
    
    fn find_open_squares(board: &Vec<Square>) -> Option<Vec<usize>> {
        let mut open_squares: Vec<usize> = Vec::new();
    
        for (index, square) in board.iter().enumerate() {
            if square.value == 0 {
                open_squares.push(index);
            }
        }
    
        if open_squares.len() > 0 {
            return Some(open_squares);
        } else {
            return None; 
        }
        
    
    }
    

    fn find_random_filled_squares(board: &Vec<Square>) -> Vec<usize> {
        let mut random_squares: Vec<usize> = Vec::new();
        let mut rng = StdRng::from_entropy();
    
        for (index, square) in board.iter().enumerate() {
            if square.value != 0 {
                random_squares.push(index);
            }
        }
    
        random_squares.shuffle(&mut rng);
        return random_squares;
    }


        
    
    //Finds all combinations that the current unsolved board allows
    fn find_valid_values(board: &Vec<Square>) -> Option<HashMap<usize, Vec<i32>>> {
        let mut valid_values = HashMap::new();
        let result = find_open_squares(&board);
        let mut empty_squares: Vec<usize> = Vec::new();
    
    
        match result {
            Some(zero_index) => empty_squares = zero_index,
            None => {
                return None;
            }
        }
    
        for index in &empty_squares {
            let mut options = Vec::new();
            for option in 1..10 {
                //println!("find valid");
                let future = help::isValid(&board, *index, option);
                if block_on(future) {
                    options.push(option);
                }
                
            }
            
            valid_values.insert(*index, options);
            
        }
    
        return Some(valid_values);
    
    
    }
    
    fn one_solution_muscle(board: &mut Vec<Square>, solutions: &mut i32, valid_values: &HashMap<usize, Vec<i32>>) -> bool {
        let result = find_open_squares(&board);
        let mut empty_squares: Vec<usize> = Vec::new();
    
    
        match result {
            Some(zero_index) => empty_squares = zero_index,
            None => {
               
                *solutions = *solutions + 1;
                //println!("\n\n S:{} \n\n", solutions);
                return true;
            }
        }
        //println!("{:?}", &empty_squares);
        for index in &empty_squares {
            //println!("{}",  index);
            for option in valid_values[index].iter() {
                //println!("one sol");
                let future = help::isValid(&board, *index, *option);
                if block_on(future) {
                    //println!("its valid!");
                    board[*index].value = *option;
                    one_solution_muscle(board, solutions, valid_values);
                    board[*index].value = 0;
                }
                
            }
            return false;
            
        }
        return false;
    }
    
    pub fn one_solution(board: &mut Vec<Square>) -> bool {
        let mut solutions: i32 = 0;
        let valid_values = find_valid_values(&board);
        let mut board_to_solve: Vec<Square> = board.clone();

        match valid_values {
            Some(values) => one_solution_muscle(&mut board_to_solve, &mut solutions, &values),
            None => return false,
        };
        if solutions == 1 {
            //println!("only 1");
            return true;
        } else {
            //println!("more than 1");
            return false;
        }
    }
}

