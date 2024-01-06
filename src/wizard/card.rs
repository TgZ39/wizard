use super::player::Player;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Card {
    Number(u32 /* value */, CardColor /* color */),
    /// value = 14
    Wizard,
    /// value = 0
    Fool,
}

impl Card {
    /// Returns the value of the given `Card`. <br>
    /// Return values of this function:
    /// - Fool -> 0
    /// - Wizard -> 14
    /// - Number -> it's value
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use wizard::wizard::card::Card;
    /// # use wizard::wizard::card::CardColor;
    /// let card: Card = Card::Number(5, CardColor::RED);
    /// let value: u32 = card.value();
    /// assert_eq!(value, 5);
    /// ```
    pub fn value(&self) -> u32 {
        match self {
            Card::Fool => 0,
            Card::Wizard => 14,
            Card::Number(value, _) => value.to_owned(),
        }
    }

    /// This function consumes a `Vec<Card>`, filters them with a `CardColor` and returns a new `Vec<Card>`. <br>
    /// Only cards with the same color as the one given are keep. Wizards and Fools are keep too.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use wizard::wizard::card::Card;
    /// # use wizard::wizard::card::CardColor;
    /// let mut cards: Vec<Card> = Vec::new();
    /// cards.push(Card::Fool);
    /// cards.push(Card::Wizard);
    /// cards.push(Card::Number(3, CardColor::RED));
    /// cards.push(Card::Number(5, CardColor::GREEN));
    /// let filtered: Vec<Card> = Card::filter(cards, CardColor::RED);
    /// ```
    pub fn filter(cards: Vec<Card>, color: CardColor) -> Vec<Card> {
        let mut out = Vec::new();

        for card in cards {
            match card {
                Card::Number(_, card_color) => {
                    if card_color == color {
                        out.push(card)
                    }
                }
                _ => out.push(card),
            }
        }
        out
    }

    /// Returns the *name* of the given card as a String. <br>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use wizard::wizard::card::Card;
    /// # use wizard::wizard::card::CardColor;
    /// let card_1: String = Card::Fool.name(); // "Fool"
    /// let card_2: String = Card::Wizard.name(); // "Wizard"
    /// let card_3: String = Card::Number(9, CardColor::YELLOW); // "Yellow 9"
    /// let card_3: String = Card::Number(1, CardColor::BLUE); // "Blue 1"
    /// ```
    pub fn name(&self) -> String {
        match self {
            Card::Fool => "Fool".to_string(),
            Card::Wizard => "Wizard".to_string(),
            Card::Number(value, color) => match color {
                CardColor::BLUE => format!("Blue {}", value),
                CardColor::GREEN => format!("Green {}", value),
                CardColor::RED => format!("Red {}", value),
                CardColor::YELLOW => format!("Yellow {}", value),
            },
        }
    }

    /// Determines the *winning* `Player` in a round of standart Wizard **with the *main color* considered.**
    ///
    /// # Panics
    ///
    /// This function panics if the given vector is empty as the winner cannot be determined.
    pub fn evaluate_winner(
        cards: Vec<(Card, Player)>,
        option_main_color: Option<CardColor>,
    ) -> Player {
        if cards.is_empty() {
            panic!("Input for this function was empty.");
        }

        let mut winner = cards[0].clone();

        // determine winner without main color
        if option_main_color.is_none() {
            for (card, player) in cards {
                if card.value() > winner.0.value() {
                    winner.0 = card;
                    winner.1 = player;
                }
            }

            return winner.1;
        }

        let main_color = option_main_color.unwrap();

        // determine winner with main color
        for (card, player) in cards {
            match (card, winner.0) {
                (Card::Number(value, color), Card::Number(winner_value, winner_color)) => {
                    if color == main_color && winner_color != main_color {
                        winner.0 = card;
                        winner.1 = player;
                    } else if color == main_color
                        && winner_color == main_color
                        && value > winner_value
                    {
                        winner.0 = card;
                        winner.1 = player;
                    } else if color != main_color
                        && winner_color != main_color
                        && value > winner_value
                    {
                        winner.0 = card;
                        winner.1 = player;
                    }
                }
                (Card::Wizard, Card::Fool) => {
                    winner.0 = card;
                    winner.1 = player;
                }
                (Card::Wizard, Card::Number(_, _)) => {
                    winner.0 = card;
                    winner.1 = player;
                }
                (Card::Number(_, _), Card::Fool) => {
                    winner.0 = card;
                    winner.1 = player;
                }
                (_, _) => {}
            }
        }
        winner.1
    }

    /// This function takes a `Vec<Card>` as arguments and returns an optional `CardColor` if the color that other players are forced to play can be determined.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use wizard::wizard::card::Card;
    /// # use wizard::wizard::card::CardColor;
    /// let mut cards: Vec<Card> = Vec::new();
    /// cards.push(Card::Fool);
    /// cards.push(Card::Number(5, CardColor::YELLOW));
    /// cards.push(Card::Number(8, CardColor::GREEN));
    /// let main_color = Card::get_forced_color(cards); // CardColor::YELLOW
    /// ```
    pub fn get_forced_color(cards: Vec<Card>) -> Option<CardColor> {
        for card in cards {
            match card {
                Card::Fool => {}
                Card::Wizard => {
                    return None;
                }
                Card::Number(_, color) => return Some(color),
            }
        }

        None
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CardColor {
    BLUE,
    GREEN,
    RED,
    YELLOW,
}
