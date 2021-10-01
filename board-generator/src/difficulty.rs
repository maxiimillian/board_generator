pub mod difficulty {

    #[derive(Clone, Debug)]
    pub struct Difficulty {
        pub filled_squares_count: i32,
        pub name: String,
    }

    impl Difficulty {
        pub fn EASY() -> Difficulty {
            Difficulty {
                filled_squares_count: 50,
                name: "easy".to_string(),
            }
        }

        pub fn MEDIUM() -> Difficulty {
            Difficulty {
                filled_squares_count: 40,
                name: "medium".to_string(),
            }
        }

        pub fn HARD() -> Difficulty {
            Difficulty {
                filled_squares_count: 30,
                name: "hard".to_string(),
            }
        }

        pub fn EXTREME() -> Difficulty {
            Difficulty {
                filled_squares_count: 26,
                name: "extreme".to_string(),
            }
        }
    }
}

