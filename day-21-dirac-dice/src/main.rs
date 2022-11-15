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

    pub fn move_by_in_turn(&mut self, roll_1_score: u32, roll_2_score: u32, roll_3_score: u32) {
        self.move_by(roll_1_score + roll_2_score + roll_3_score);
        self.turn += 1;
    }

    pub fn won(&self) -> bool {
        self.score >= 1000
    }

    fn move_by(&mut self, roll_score: u32) {
        self.position = ((self.position as u32 + roll_score - 1) % 10 + 1) as u8;
        self.score += self.position as u32;
    }
}

pub fn play(mut player1: Player, mut player2: Player) -> (Player, Player) {
    (0..)
        .take_while(|turn| {
            player1.move_by_in_turn(
                (turn * 6 + 1) - 1 % 100 + 1,
                (turn * 6 + 2) - 1 % 100 + 1,
                (turn * 6 + 3) - 1 % 100 + 1,
            );
            if !player1.won() {
                player2.move_by_in_turn(
                    (turn * 6 + 4) - 1 % 100 + 1,
                    (turn * 6 + 5) - 1 % 100 + 1,
                    (turn * 6 + 6) - 1 % 100 + 1,
                );
                return !player2.won();
            }
            return false;
        })
        .count();

    (player1, player2)
}

fn part_1_result(player1: &Player, player2: &Player) {
    let (player1, player2) = play(*player1, *player2);
    let result = player1.score.min(player2.score) * ((player1.turn + player2.turn) * 3);

    println!("Part 1. Result: {}", result);
}

fn main() {
    let player1 = Player::new(1);
    let player2 = Player::new(5);

    part_1_result(&player1, &player2)
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

        player.move_by_in_turn(2, 2, 1);
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

        let (player1, player2) = play(player1, player2);
        println!("{:?} {:?}", player1, player2);

        assert_eq!(
            player1.score.min(player2.score) * ((player1.turn + player2.turn) * 3),
            739785
        )
    }
}
