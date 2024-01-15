use inquire::Select;
use wizard::wizard::game::Game;
use wizard::wizard::player::Player;

fn main() {
    println!("Wizard-rs");

    let mut game = Game::default();

    // select player count
    {
        let options = (3..=6).map(|e| e.to_string()).collect();
        let player_count = Select::new("How many players are you?", options)
            .prompt()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        // create players
        for i in 1..=player_count {
            let player = Player::new(i as u32);
            game.players.push(player);
        }
    }

    for round_number in 1..=game.round_limit() {
        println!("Round number {}", round_number);
    }
}