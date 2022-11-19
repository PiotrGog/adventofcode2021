#[derive(Debug, Clone, Copy)]
pub struct Player {
    position: u8,
    score: u32,
    turn: u32,
}

impl Player {
    pub fn new(position: u8) -> Self {
        Self {
            position,
            score: 0,
            turn: 0,
        }
    }

    pub fn move_by_in_turn(&mut self, roll_sum: u32) {
        self.move_by(roll_sum);
        self.turn += 1;
    }

    pub fn won(&self, winning_score: u32) -> bool {
        self.score >= winning_score
    }

    fn move_by(&mut self, roll_score: u32) {
        self.position = ((self.position as u32 + roll_score - 1) % 10 + 1) as u8;
        self.score += self.position as u32;
    }
}

pub trait DiceTrait {
    type Result;

    fn new() -> Self;
    fn roll_in_turn(&mut self) -> Self::Result;
}

#[derive(Debug, Clone, Copy)]
pub struct DeterministicDice {
    roll: u32,
}

impl DiceTrait for DeterministicDice {
    type Result = u32;

    fn new() -> Self {
        Self { roll: 0 }
    }

    fn roll_in_turn(&mut self) -> Self::Result {
        let sum = self.roll % 100 + (self.roll + 1) % 100 + (self.roll + 2) % 100 + 3;
        self.roll += 3;
        sum
    }
}

#[derive(Debug, Clone, Copy)]
pub struct QuastumDice {}

impl DiceTrait for QuastumDice {
    type Result = Vec<u32>;

    fn new() -> Self {
        Self {}
    }

    fn roll_in_turn(&mut self) -> Self::Result {
        vec![3, 4, 5, 6, 7, 8, 9]
    }
}

pub fn play_with_deterministic_dice(
    mut player1: Player,
    mut player2: Player,
    mut dice: DeterministicDice,
    winning_score: u32,
) -> (Player, Player) {
    (0..)
        .take_while(|_| {
            player1.move_by_in_turn(dice.roll_in_turn());
            if !player1.won(winning_score) {
                player2.move_by_in_turn(dice.roll_in_turn());
                return !player2.won(winning_score);
            }
            return false;
        })
        .count();

    (player1, player2)
}

fn universes_with_scores(w: u32) -> u64 {
    match w {
        3 => 1,
        4 => 3,
        5 => 6,
        6 => 7,
        7 => 6,
        8 => 3,
        9 => 1,
        _ => panic!(),
    }
}

pub fn play_with_quastum_dice(
    player1: Player,
    player2: Player,
    mut dice: QuastumDice,
    winning_score: u32,
) -> (u64, u64) {
    let mut player1_wins = 0;
    let mut player2_wins = 0;

    let binding = dice.roll_in_turn();
    let mut first_player_roll = binding.iter();
    while let Some(roll_result1) = first_player_roll.next() {
        let mut player1_clone = player1;
        player1_clone.move_by_in_turn(*roll_result1);
        if player1_clone.won(winning_score) {
            player1_wins += universes_with_scores(*roll_result1);
            continue;
        }
        let mut second_player_roll = binding.iter();
        while let Some(roll_result2) = second_player_roll.next() {
            let mut player2_clone = player2;
            player2_clone.move_by_in_turn(*roll_result2);
            if player2_clone.won(winning_score) {
                player2_wins += universes_with_scores(*roll_result2);
                continue;
            }

            let (player1_result, player2_result) =
                play_with_quastum_dice(player1_clone, player2_clone, dice, winning_score);
            let multiplier =
                universes_with_scores(*roll_result1) * universes_with_scores(*roll_result2);
            player1_wins += player1_result * multiplier;
            player2_wins += player2_result * multiplier;
        }
    }

    (player1_wins, player2_wins)
}

fn part_1_result(player1: &Player, player2: &Player) {
    let (player1, player2) =
        play_with_deterministic_dice(*player1, *player2, DeterministicDice::new(), 1000);
    let result = player1.score.min(player2.score) * ((player1.turn + player2.turn) * 3);

    println!("Part 1. Result: {}", result);
}

fn part_2_result(player1: &Player, player2: &Player) {
    let (player1_wins, player2_wins) =
        play_with_quastum_dice(*player1, *player2, QuastumDice::new(), 21);
    let result = player1_wins.max(player2_wins);

    println!("Part 2. Result: {}", result);
}

fn main() {
    let player1 = Player::new(1);
    let player2 = Player::new(5);

    part_1_result(&player1, &player2);
    part_2_result(&player1, &player2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_by() {
        let mut player = Player::new(7);

        player.move_by(2);
        let Player {
            position,
            score: _,
            turn: _,
        } = player;
        assert_eq!(position, 9);

        player.move_by(2);
        let Player {
            position,
            score: _,
            turn: _,
        } = player;
        assert_eq!(position, 1);

        player.move_by(1);
        let Player {
            position,
            score: _,
            turn: _,
        } = player;
        assert_eq!(position, 2);
    }

    #[test]
    fn test_move_by_in_turn() {
        let mut player = Player::new(7);

        player.move_by_in_turn(2 + 2 + 1);
        let Player {
            position,
            score,
            turn: _,
        } = player;
        assert_eq!(position, 2);
        assert_eq!(score, 2);
    }

    #[test]
    fn test_part_1() {
        let player1 = Player::new(4);
        let player2 = Player::new(8);

        let (player1, player2) =
            play_with_deterministic_dice(player1, player2, DeterministicDice::new(), 1000);
        assert_eq!(
            player1.score.min(player2.score) * ((player1.turn + player2.turn) * 3),
            739785
        )
    }

    #[test]
    fn test_part_2() {
        let player1 = Player::new(4);
        let player2 = Player::new(8);

        let (player1_wins, player2_wins) =
            play_with_quastum_dice(player1, player2, QuastumDice::new(), 21);
        assert_eq!(player1_wins.max(player2_wins), 444356092776315)
    }
}
