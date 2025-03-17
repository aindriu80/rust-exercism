#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { rolls: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let (frames, rolls_in_current) = self.count_frames_and_rolls();
        if frames >= 10 && !self.needs_bonus_rolls() {
            return Err(Error::GameComplete);
        }

        if frames < 10 && rolls_in_current == 1 {
            let prev_roll = self.rolls[self.rolls.len() - 1];
            if prev_roll != 10 && prev_roll + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
        } else if frames >= 10 && rolls_in_current == 2 {
            // Second bonus roll
            let tenth_start = self.rolls.len() - 1;
            let first_bonus = self.rolls[tenth_start];
            let tenth_first = self.rolls[tenth_start - 1];
            if tenth_first != 10 && first_bonus != 10 && first_bonus + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
        }

        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_complete() || self.needs_bonus_rolls() {
            return None;
        }

        let mut total = 0;
        let mut roll_idx = 0;

        // Score first 9 frames with bonuses
        for _ in 0..9 {
            if roll_idx >= self.rolls.len() {
                return None;
            }
            if self.rolls[roll_idx] == 10 {
                if roll_idx + 2 >= self.rolls.len() {
                    return None;
                }
                total += 10 + self.rolls[roll_idx + 1] + self.rolls[roll_idx + 2];
                roll_idx += 1;
            } else if roll_idx + 1 < self.rolls.len() {
                let first = self.rolls[roll_idx];
                let second = self.rolls[roll_idx + 1];
                if first + second == 10 {
                    if roll_idx + 2 >= self.rolls.len() {
                        return None;
                    }
                    total += 10 + self.rolls[roll_idx + 2];
                    roll_idx += 2;
                } else {
                    total += first + second;
                    roll_idx += 2;
                }
            } else {
                return None;
            }
        }

        // Score 10th frame: sum all remaining rolls
        if roll_idx >= self.rolls.len() {
            return None;
        }
        if self.rolls[roll_idx] == 10 {
            if roll_idx + 2 >= self.rolls.len() {
                return None;
            }
            total += 10 + self.rolls[roll_idx + 1] + self.rolls[roll_idx + 2];
        } else if roll_idx + 1 < self.rolls.len() {
            let first = self.rolls[roll_idx];
            let second = self.rolls[roll_idx + 1];
            if first + second == 10 {
                if roll_idx + 2 >= self.rolls.len() {
                    return None;
                }
                total += 10 + self.rolls[roll_idx + 2];
            } else {
                total += first + second;
            }
        }

        Some(total)
    }

    fn count_frames_and_rolls(&self) -> (usize, usize) {
        let mut frames = 0;
        let mut roll_idx = 0;

        while roll_idx < self.rolls.len() && frames < 10 {
            if self.rolls[roll_idx] == 10 {
                roll_idx += 1;
            } else {
                if roll_idx + 1 < self.rolls.len() {
                    roll_idx += 2;
                } else {
                    return (frames, 1);
                }
            }
            frames += 1;
        }

        (frames, self.rolls.len() - roll_idx)
    }

    fn is_game_complete(&self) -> bool {
        let (frames, rolls_in_current) = self.count_frames_and_rolls();
        frames >= 10 && !self.needs_bonus_rolls()
    }

    fn needs_bonus_rolls(&self) -> bool {
        let (frames, rolls_in_current) = self.count_frames_and_rolls();
        if frames < 10 {
            return true; // Not complete yet
        }
        if rolls_in_current == 0 {
            return false; // No rolls in 10th yet
        }

        let tenth_start = self.rolls.len() - rolls_in_current;
        if self.rolls[tenth_start] == 10 {
            rolls_in_current < 3 // Strike needs 2 bonus rolls
        } else if rolls_in_current >= 2
            && self.rolls[tenth_start] + self.rolls[tenth_start + 1] == 10
        {
            rolls_in_current < 3 // Spare needs 1 bonus roll
        } else {
            rolls_in_current < 2 // Open frame needs 2 rolls
        }
    }
}
