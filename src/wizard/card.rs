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
    /// let card_3: String = Card::Number(9, CardColor::YELLOW).name(); // "Yellow 9"
    /// let card_3: String = Card::Number(1, CardColor::BLUE).name(); // "Blue 1"
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

    /// Determines the *winning* `Player` in a round of standard Wizard **with the *main color* considered.**
    ///
    /// # Panics
    ///
    /// This function panics if the given vector is empty as the winner cannot be determined.
    pub fn evaluate_winner(cards: Vec<(Card, Player)>, main_color: Option<CardColor>) -> Player {
        if cards.is_empty() {
            panic!("Input for this function was empty.");
        }

        let mut winner: (Card, Player) = cards[0].clone();
        macro_rules! transfer_winner {
            ($card:expr; $player:expr) => {
                winner.0 = $card;
                winner.1 = $player;
            };
        }
        let prio_color: Option<CardColor>;

        {
            let mut cards_only: Vec<Card> = Vec::new();
            for card in cards.clone() {
                cards_only.push(card.0);
            }
            prio_color = Card::get_prio_color(cards_only);
        }

        match (main_color, prio_color) {
            (Some(main_color), Some(prio_color)) => {
                for (new_card, new_player) in cards.clone() {
                    match (new_card, winner.0) {
                        (Card::Wizard, _) => return new_player,
                        (Card::Number(_, _), Card::Fool) => {
                            winner.0 = new_card;
                            winner.1 = new_player;
                        }
                        (
                            Card::Number(new_value, new_color),
                            Card::Number(old_value, old_color),
                        ) => {
                            if new_color == main_color
                                && old_color == main_color
                                && new_value > old_value
                            {
                                // same color
                                transfer_winner!(new_card; new_player);
                            } else if new_color == main_color && old_color != main_color {
                                // higher value color
                                transfer_winner!(new_card; new_player);
                            } else if new_color == prio_color
                                && old_color != prio_color
                                && old_color != main_color
                            {
                                // higher value color
                                transfer_winner!(new_card; new_player);
                            } else if new_color == prio_color
                                && old_color == prio_color
                                && new_value > old_value
                            {
                                // same color
                                transfer_winner!(new_card; new_player);
                            } else if new_color != main_color
                                && new_color != prio_color
                                && old_color != main_color
                                && old_color != prio_color
                                && new_value > old_value
                            {
                                // same value color
                                transfer_winner!(new_card; new_player);
                            }
                        }
                        (_, _) => {}
                    }
                }
            }
            (None, Some(prio_color)) => {
                for (new_card, new_player) in cards.clone() {
                    match (new_card, winner.0) {
                        (Card::Wizard, _) => return new_player,
                        (Card::Number(_, _), Card::Fool) => {
                            transfer_winner!(new_card; new_player);
                        }
                        (
                            Card::Number(new_value, new_color),
                            Card::Number(old_value, old_color),
                        ) => {
                            if new_color == prio_color && old_color != prio_color {
                                transfer_winner!(new_card; new_player);
                            } else if new_color == prio_color
                                && old_color == prio_color
                                && new_value > old_value
                            {
                                transfer_winner!(new_card; new_player);
                            } else if new_color != prio_color
                                && old_color != prio_color
                                && new_value > old_value
                            {
                                transfer_winner!(new_card; new_player);
                            }
                        }
                        (_, _) => {}
                    }
                }
            }
            (Some(main_color), None) => {
                for (new_card, new_player) in cards.clone() {
                    match (new_card, winner.0) {
                        (Card::Wizard, _) => return new_player,
                        (Card::Number(_, _), Card::Fool) => {
                            transfer_winner!(new_card; new_player);
                        }
                        (
                            Card::Number(new_value, new_color),
                            Card::Number(old_value, old_color),
                        ) => {
                            if new_color == main_color && old_color != main_color {
                                transfer_winner!(new_card; new_player);
                            } else if new_color == main_color
                                && old_color == main_color
                                && new_value > old_value
                            {
                                transfer_winner!(new_card; new_player);
                            } else if new_color != main_color
                                && old_color != main_color
                                && new_value > old_value
                            {
                                transfer_winner!(new_card; new_player);
                            }
                        }
                        (_, _) => {}
                    }
                }
            }
            (None, None) => {
                for (new_card, new_player) in cards.clone() {
                    match (new_card, winner.0) {
                        (Card::Wizard, _) => return new_player,
                        (Card::Number(_, _), Card::Fool) => {
                            transfer_winner!(new_card; new_player);
                        }
                        (Card::Number(new_value, _), Card::Number(old_value, _)) => {
                            if new_value > old_value {
                                transfer_winner!(new_card; new_player);
                            }
                        }
                        (_, _) => {}
                    }
                }
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
    /// let main_color = Card::get_prio_color(cards); // CardColor::YELLOW
    /// ```
    pub fn get_prio_color(cards: Vec<Card>) -> Option<CardColor> {
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

    /// Returns a `Vec<Card>` with all possible cards in wizard. (sorted)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use wizard::wizard::card::Card;
    /// # use wizard::wizard::card::CardColor;
    ///
    /// let all_cards: Vec<Card> = Card::all_cards();
    /// ```
    pub fn all_cards() -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();
        let colors: [CardColor; 4] = [CardColor::BLUE, CardColor::GREEN, CardColor::RED, CardColor::YELLOW];
        for color in colors {
            for value in 1..=13 {
                cards.push(Card::Number(value, color));
            }
            cards.push(Card::Fool);
            cards.push(Card::Wizard);
        }
        cards
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CardColor {
    BLUE,
    GREEN,
    RED,
    YELLOW,
}