pub mod strategies {
    use crate::square::square::Square;
    use crate::help::help;
    use crate::board::board;
    use std::collections::HashMap;
    use itertools::Itertools;

    pub fn naked_doubles(board: &Vec<Square>) {
        
    }

    pub fn check_adjacent_candidates(board: &Vec<Square> , index: usize, candidate: i32) -> bool {
        let square = &board[index];

        for other_square in board.iter() {
            
            if help::isRow(square, &other_square) || help::isColumn(square, &other_square) || help::isBox(board, square, &other_square) {

                for other_candidate in other_square.candidates.iter() {

                    if *other_candidate == candidate {
                        return false;

                    }
                }

            }

        }
        //println!("NO CANDIDATE IS THE SAME");
        return true;
    }

    pub fn naked_hidden_singles(board: &Vec<Square>) -> HashMap<usize, i32>{
        let mut naked_singles: HashMap<usize, i32> = HashMap::new();

        for index in 0..board.len() {
            let square = &board[index];
            let candidates = &square.candidates;

            if square.value != 0 {
                continue;
            }

            if candidates.len() == 1 {
                println!("C: ({},{}) - {}", square.x, square.y, candidates[0]);
                naked_singles.insert(index as usize, candidates[0]);

            } else {
                for candidate in candidates.iter() {
                    if check_adjacent_candidates(board, index, *candidate) {
                        println!("C: ({},{}) - {}", square.x, square.y, *candidate);
                        naked_singles.insert(index as usize, *candidate);
                    } 
                }
            }
        }
        
        return naked_singles;   
    }


    
}