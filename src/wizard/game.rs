use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::wizard::card::Card;
use crate::wizard::player::Player;

pub struct Game {
    pub players: Vec<Player>,
}

impl Game {
    pub fn shift(&mut self) {
        self.players.rotate_right(1);
    }

    pub fn shift_till(&mut self, player: Player) {
        while self.players[0] != player {
            self.shift();
        }
    }

    pub fn round_limit(&self) -> u32 {
        60 / self.players.len() as u32
    }

    pub fn assign_cards(&mut self, amount: u32) {
        if amount <= 0 || amount >= 21 {
            panic!("Invalid amount of cards to assign? Allowed is anything between 1 and 60/player_count = {}. Provided was {}.", self.round_limit(), amount)
        }

        // generate cards
        let mut cards = Card::all_cards();
        // randomize cards
        cards.shuffle(&mut thread_rng());

        // empty player cards
        for player in &mut self.players {
            player.cards.clear()
        }

        // assign new cards
        for player in &mut self.players {
            for _ in 1..=amount {
                player.cards.push(cards[0])
            }
        }
    }

    pub fn stitch_options(max: u32, current_count: u32, is_last: bool) -> Vec<u32> {

        let mut out: Vec<u32> = (1..=max).collect();

        return out.iter().filter(|x| x != max - current_count).collect();
    }
}

impl Default for Game {
    fn default() -> Self {
        Game { players: vec![] }
    }
}
