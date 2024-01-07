#[cfg(test)]
mod tests {
    use crate::wizard::{
        card::{Card, CardColor},
        player::Player,
    };

    #[test]
    fn card_value() {
        assert_eq!(Card::Fool.value(), 0);
        assert_eq!(Card::Wizard.value(), 14);
        assert_eq!(Card::Number(4, CardColor::BLUE).value(), 4);
    }

    #[test]
    fn card_color_filter() {
        let mut cards = Vec::new();
        cards.push(Card::Fool);
        cards.push(Card::Wizard);
        cards.push(Card::Number(5, CardColor::RED));
        cards.push(Card::Number(4, CardColor::GREEN));

        let mut correct_filtered = Vec::new();
        correct_filtered.push(Card::Fool);
        correct_filtered.push(Card::Wizard);
        correct_filtered.push(Card::Number(5, CardColor::RED));

        let filtered = Card::filter(cards, CardColor::RED);

        assert_eq!(filtered, correct_filtered);
    }

    #[test]
    fn card_name() {
        assert_eq!(
            "Blue 5".to_string(),
            Card::Number(5, CardColor::BLUE).name()
        );
        assert_eq!("Fool".to_string(), Card::Fool.name());
        assert_eq!("Wizard".to_string(), Card::Wizard.name());
    }

    fn new_player(name: String) -> Player {
        Player {
            name: name,
            cards: Vec::new(),
            guess_stitches: 0,
            actual_stiches: 0,
        }
    }

    #[test]
    fn evaluate_winner() {
        fn check_winner_with_color(
            cards: &Vec<(Card, Player)>,
            main_color: &CardColor,
            winner: &Player,
        ) {
            let eval_winner = Card::evaluate_winner(cards.clone(), Some(main_color.clone()));
            assert_eq!(
                winner.clone(),
                eval_winner.clone(),
                "Input ({:?}): {:#?}. Correct winner: {}. Eval Winner: {}",
                main_color,
                cards,
                winner.name,
                eval_winner.name
            )
        }

        fn check_winner_without_color(cards: &Vec<(Card, Player)>, winner: &Player) {
            let eval_winner = Card::evaluate_winner(cards.clone(), None);
            assert_eq!(
                winner.clone(),
                eval_winner.clone(),
                "Input (None): {:#?}. Correct winner: {}. Eval Winner: {}",
                cards,
                winner.name,
                eval_winner.name
            )
        }

        let p1 = new_player("Max".to_string());
        let p2 = new_player("David".to_string());
        let p3 = new_player("Karl".to_string());

        let mut case_1: Vec<(Card, Player)> = Vec::new();

        case_1.push((Card::Fool, p1.clone()));
        case_1.push((Card::Number(7, CardColor::BLUE), p2.clone()));
        case_1.push((Card::Number(5, CardColor::BLUE), p3.clone()));

        check_winner_with_color(&case_1, &CardColor::BLUE, &p2);
        check_winner_with_color(&case_1, &CardColor::GREEN, &p2);

        let mut case_2: Vec<(Card, Player)> = Vec::new();

        case_2.push((Card::Fool, p1.clone()));
        case_2.push((Card::Wizard, p2.clone()));
        case_2.push((Card::Fool, p3.clone()));

        check_winner_with_color(&case_2, &CardColor::BLUE, &p2);

        let mut case_3: Vec<(Card, Player)> = Vec::new();

        case_3.push((Card::Fool, p1.clone()));
        case_3.push((Card::Number(7, CardColor::RED), p2.clone()));
        case_3.push((Card::Number(5, CardColor::GREEN), p3.clone()));

        check_winner_with_color(&case_3, &CardColor::RED, &p2);
        check_winner_with_color(&case_3, &CardColor::GREEN, &p3);
        check_winner_with_color(&case_3, &CardColor::BLUE, &p2);

        let mut case_4: Vec<(Card, Player)> = Vec::new();

        case_4.push((Card::Fool, p1.clone()));
        case_4.push((Card::Fool, p2.clone()));
        case_4.push((Card::Fool, p3.clone()));

        check_winner_with_color(&case_4, &CardColor::BLUE, &p1);

        let mut case_5: Vec<(Card, Player)> = Vec::new();

        case_5.push((Card::Wizard, p1.clone()));
        case_5.push((Card::Wizard, p2.clone()));
        case_5.push((Card::Wizard, p3.clone()));

        check_winner_with_color(&case_5, &CardColor::GREEN, &p1);

        let mut case_6: Vec<(Card, Player)> = Vec::new();

        case_6.push((Card::Number(9, CardColor::YELLOW), p1.clone()));
        case_6.push((Card::Number(4, CardColor::YELLOW), p2.clone()));
        case_6.push((Card::Number(6, CardColor::RED), p3.clone()));
        case_6.push((Card::Number(13, CardColor::BLUE), p1.clone()));
        case_6.push((Card::Number(10, CardColor::YELLOW), p2.clone()));
        case_6.push((Card::Fool, p3.clone()));

        check_winner_with_color(&case_6, &CardColor::YELLOW, &p2);
        check_winner_with_color(&case_6, &CardColor::BLUE, &p1);
        check_winner_with_color(&case_6, &CardColor::RED, &p3);
        check_winner_with_color(&case_6, &CardColor::GREEN, &p2);

        let mut case_7: Vec<(Card, Player)> = Vec::new();

        case_7.push((Card::Number(9, CardColor::YELLOW), p1.clone()));
        case_7.push((Card::Number(4, CardColor::YELLOW), p2.clone()));
        case_7.push((Card::Number(6, CardColor::RED), p3.clone()));
        case_7.push((Card::Number(13, CardColor::BLUE), p1.clone()));
        case_7.push((Card::Number(10, CardColor::YELLOW), p2.clone()));
        case_7.push((Card::Fool, p3.clone()));

        check_winner_without_color(&case_6, &p2);

        let mut case_8: Vec<(Card, Player)> = Vec::new();

        case_8.push((Card::Number(7, CardColor::BLUE), p1.clone()));
        case_8.push((Card::Number(9, CardColor::RED), p2.clone()));

        check_winner_with_color(&case_8, &CardColor::BLUE, &p1);
        check_winner_with_color(&case_8, &CardColor::RED, &p2);
        check_winner_without_color(&case_8, &p1);

        // write more tests for this function
        let mut case_9: Vec<(Card, Player)> = Vec::new();

        case_9.push((Card::Number(4, CardColor::GREEN), p1.clone()));
        case_9.push((Card::Number(12, CardColor::BLUE), p2.clone()));
        case_9.push((Card::Number(3, CardColor::GREEN), p3.clone()));
        case_9.push((Card::Number(6, CardColor::BLUE), p1.clone()));
        case_9.push((Card::Number(7, CardColor::GREEN), p2.clone()));
        case_9.push((Card::Number(13, CardColor::RED), p3.clone()));

        check_winner_with_color(&case_9, &CardColor::BLUE, &p2);
        check_winner_with_color(&case_9, &CardColor::GREEN, &p2);
        check_winner_with_color(&case_9, &CardColor::RED, &p3);
        check_winner_with_color(&case_9, &CardColor::YELLOW, &p2);
        check_winner_without_color(&case_9, &p2);
    }

    #[test]
    fn get_forced_color() {
        fn check(cards: &Vec<Card>, correct_color: Option<CardColor>) {
            let eval_color = Card::get_prio_color(cards.clone());
            assert_eq!(
                eval_color,
                correct_color,
                "Error with input: {:#?}. Correct is {:?}. Output was {:?}",
                cards.clone(),
                correct_color.clone(),
                eval_color.clone()
            );
        }

        let mut case_1: Vec<Card> = Vec::new();
        case_1.push(Card::Fool);
        case_1.push(Card::Fool);
        case_1.push(Card::Wizard);

        check(&case_1, None);

        let mut case_2: Vec<Card> = Vec::new();
        case_2.push(Card::Number(5, CardColor::BLUE));
        case_2.push(Card::Wizard);
        case_2.push(Card::Fool);

        check(&case_2, Some(CardColor::BLUE));

        let mut case_3: Vec<Card> = Vec::new();
        case_3.push(Card::Wizard);
        case_3.push(Card::Number(4, CardColor::GREEN));
        case_3.push(Card::Number(5, CardColor::RED));

        check(&case_3, None);

        let mut case_4: Vec<Card> = Vec::new();
        case_4.push(Card::Fool);
        case_4.push(Card::Number(4, CardColor::GREEN));
        case_4.push(Card::Number(5, CardColor::RED));

        check(&case_4, Some(CardColor::GREEN));
    }
}
