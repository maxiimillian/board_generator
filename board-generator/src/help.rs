pub mod help {
    use futures::{
        future::FutureExt, // for `.fuse()`
        pin_mut,
        select,
    };

    use crate::square::square::Square;

    use rand::{ Rng, thread_rng };
    use rand::seq::SliceRandom;
    use std::{fs::OpenOptions, fs::File, io::prelude::*};
    use std::io::{BufWriter, Write};
    use rand::prelude::*;
    use std::collections::HashMap;

    pub fn isRow(square: &Square, other_square: &Square) -> bool {
        return other_square.y == square.y && other_square.x != square.x;
    }

    pub fn isColumn(square: &Square, other_square: &Square) -> bool {
        return other_square.x == square.x && other_square.y != square.y;
    }

    pub fn isBox(board: &Vec<Square>, square: &Square, other_square: &Square) -> bool {
        let square_cordinates: (i32, i32) = find_box_x(board, square);
        let other_square_cordinates: (i32, i32) = find_box_x(board, other_square);

        return !(square.x == other_square.x && square.y == other_square.y) && other_square_cordinates == square_cordinates;
    }

    fn find_box_y(square: &Square, base_unit: i32) -> i32 {
        if square.y <= base_unit-1 {
            return 1;
        }
        else if square.y <= (base_unit * 2)-1 {
            return 2;
        }
        else if square.y <= (base_unit * 3)-1 {
            return 3;
        }
        return 0;
    }
    
    //For finding the x cordinate for the sub-grid of a given square in the board
    fn find_box_x(board: &Vec<Square>, square: &Square) -> (i32, i32) {
        let board_length = board.len() as f64;
        let square_side_length = board_length.sqrt() as i32;
        let square_side_length_base_unit: i32 = square_side_length / 3;
    
        if square.x <= (square_side_length_base_unit)-1 {
            return (1, find_box_y(square, square_side_length_base_unit));
        }
        else if square.x <= (square_side_length_base_unit * 2)-1 {
            return (2, find_box_y(square, square_side_length_base_unit));
        }
        else if square.x <= (square_side_length_base_unit * 3)-1 {
            return (3, find_box_y(square, square_side_length_base_unit));
        }
        
        return (0, 0);
    }
    
    fn isValidBox(board: &Vec<Square>, index: usize, option: i32) -> bool {
        let square = &board[index];

        for other_square in board.iter() {
            if  isBox(board, square, &other_square) && other_square.value == option && other_square.value != 0  {
                return false;
            }
        }
        return true;
    
    }
    
    fn isValidRow(board: &Vec<Square>, index: usize, option: i32) -> bool {
        let square = &board[index];
        for other_square in board.iter() {
            if isRow(square, &other_square)  && (other_square.value == option && option != 0) {
                return false;
            }
        }
        return true;
    }
    
    fn isValidColumn(board: &Vec<Square>, index: usize, option: i32) -> bool {
        let square = &board[index];
        for other_square in board.iter() {
            if isColumn(square, &other_square) && (other_square.value == option && option != 0) {
                return false;
            }
        }
        return true;
    } 

    pub async fn isValid(board: &Vec<Square>, index: usize, option: i32) -> bool {
        
        if isValidColumn(board, index, option) && isValidRow(board, index, option) && isValidBox(board, index, option) {
            return true;
        } else {
            return false;
        }

    }
    


    pub fn print_vec(board: &Vec<Square>) {
        println!("");
        for square in board.iter() {
    
            if (square.y) % 3 == 0 && square.y != 0{
                print!(" | ");
            }
    
            if (square.x) % 3 == 0 && square.x != 0 && (square.y+1) == 1 {
                print!("\n{}\n", "=".repeat(32));
            }
            else if (square.y) % 9 == 0 as i32{
                print!("\n\n");
            }
            print!(" {} ", square.value.to_string());
    
    
           
    
        }
        println!("");
    }
    
    pub fn string_to_squares(board_string: &str) -> Vec<Square> {
        let mut squares: Vec<Square> = Vec::new();
    
        let string_length = board_string.chars().count() as f64;
        let square_side_length = string_length.sqrt() as i32;
    
        let mut x: i32 = 0;
        let mut y: i32 = 0;
    
        for value in board_string.chars() {
            squares.push(Square {
                x,
                y,
                value: value as i32 - 0x30,
                candidates: Vec::new(),
            });
    
            x += 1;
            if x % square_side_length == 0 {
                y += 1;
                x = 0;
            }
            
        }
        return squares
    }
    
    pub fn squares_to_string(board: &Vec<Square>) -> String {
        let mut board_string: String = "".to_string(); 
    
        for square in board.iter() {
            board_string = board_string + &square.value.to_string();
        }
    
        return board_string;
    }
    
}


