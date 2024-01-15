use inquire::{Select, Text};

use crate::wizard::card::Card;

#[derive(PartialEq, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub cards: Vec<Card>,
    pub guess_stitches: u32,
    pub actual_stitches: u32,
}

impl Player {
    /// This functions will return a default `Player` with no cards and a name that was choosen by the user.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let my_player: Player = Player::new(1); // user is asked to input his name
    /// ```
    pub fn new(index: u32) -> Self {
        // get input username
        loop {
            let input = Text::new(format!("Player {}: What's your name?", index).as_str()).prompt();

            match input {
                Ok(name) => {
                    if name.len() <= 1 {
                        println!("Your name is to short.")
                    } else {
                        return Player {
                            name,
                            cards: Vec::new(),
                            guess_stitches: 0,
                            actual_stiches: 0,
                        };
                    }
                }
                Err(e) => println!("Error reading input. ({})", e),
            }
        }
    }

    /// This function is called on a `Player` and will take in a vector of `u32`s, which are the possible options for the player. The player will then be asked to select how many stitches he want's to get.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use wizard::wizard::player::Player;
    ///
    /// let mut player: Player = Player::new(1);
    ///
    /// let player.guess_stitches = player.guess_stitches(&self, vec![1, 2, 3, 5]); // player will be asked to select his stitches
    /// ```
    pub fn guess_stitches(&self, options: Vec<u32>) -> u32 {
        loop {
            let input = Select::new(
                format!("{}: Select how many stitches you want to get.", self.name).as_str(),
                options.iter().map(|x| x.to_string()).collect(),
            )
            .prompt();
            match input {
                Ok(guess) => {
                    return guess.trim().parse().unwrap();
                }
                Err(e) => {
                    println!("Error reading input. ({})", e)
                }
            }
        }
    }
}
