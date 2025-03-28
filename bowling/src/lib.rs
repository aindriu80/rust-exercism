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
        // Basic validation
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        // Check if the game is complete
        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }

        // Get current frame information
        let frame_info = self.frame_info();

        // Check pin constraints
        match frame_info.current_frame {
            // Normal frames (1-9)
            Some(frame) if frame < 9 => {
                if frame_info.roll_in_frame == 1 {
                    // Second roll in the frame - check pin constraint
                    let first_roll = self.rolls[self.rolls.len() - 1];
                    if first_roll + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                }
            }
            // 10th frame
            Some(9) => {
                if frame_info.roll_in_frame == 1 {
                    // Second roll in 10th frame
                    let first_roll = self.rolls[self.rolls.len() - 1];
                    // If first roll was not a strike, check pins constraint
                    if first_roll != 10 && first_roll + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                } else if frame_info.roll_in_frame == 2 {
                    // Third roll in 10th frame
                    let tenth_start = frame_info.frame_start_indices[9];
                    let first_roll = self.rolls[tenth_start];
                    let second_roll = self.rolls[tenth_start + 1];

                    // If first was a strike, second wasn't a strike, check pin constraint
                    if first_roll == 10 && second_roll != 10 && second_roll + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                    // If first wasn't a strike, second makes spare (bonus roll), no constraint
                    // If first wasn't a strike, second doesn't make spare (no bonus roll allowed)
                    if first_roll != 10 && first_roll + second_roll != 10 {
                        return Err(Error::GameComplete);
                    }
                }
            }
            // Game already complete
            _ => return Err(Error::GameComplete),
        }

        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        // If the game isn't complete, we can't calculate the score
        if !self.is_game_complete() {
            return None;
        }

        let mut score = 0;
        let mut frame_index = 0;

        // Score each of the 10 frames
        for _ in 0..10 {
            if frame_index >= self.rolls.len() {
                return None;
            }

            // Strike
            if self.rolls[frame_index] == 10 {
                // Need two more rolls for the strike bonus
                if frame_index + 2 >= self.rolls.len() {
                    return None;
                }

                score += 10 + self.rolls[frame_index + 1] + self.rolls[frame_index + 2];
                frame_index += 1;
            }
            // Spare or Open Frame
            else {
                // Need at least one more roll
                if frame_index + 1 >= self.rolls.len() {
                    return None;
                }

                let two_roll_score = self.rolls[frame_index] + self.rolls[frame_index + 1];

                // Spare
                if two_roll_score == 10 {
                    // Need one more roll for the spare bonus
                    if frame_index + 2 >= self.rolls.len() {
                        return None;
                    }

                    score += 10 + self.rolls[frame_index + 2];
                }
                // Open frame
                else {
                    score += two_roll_score;
                }

                frame_index += 2;
            }
        }

        // All frames scored successfully
        Some(score)
    }

    // Helper to determine if the game is complete
    fn is_game_complete(&self) -> bool {
        let frame_info = self.frame_info();

        // If we've reached beyond the 10th frame
        if frame_info.current_frame.is_none() {
            return true;
        }

        // If we're not in the 10th frame, game is not complete
        if frame_info.current_frame != Some(9) {
            return false;
        }

        // 10th frame logic
        if let Some(tenth_start) = frame_info.frame_start_indices.get(9) {
            let rolls_in_tenth = self.rolls.len() - tenth_start;

            // Not enough rolls
            if rolls_in_tenth < 2 {
                return false;
            }

            let first_roll = self.rolls[*tenth_start];
            let second_roll = self.rolls[*tenth_start + 1];

            // Strike in first roll of 10th frame - need 2 more rolls
            if first_roll == 10 {
                return rolls_in_tenth >= 3;
            }

            // Spare in 10th frame - need 1 more roll
            if first_roll + second_roll == 10 {
                return rolls_in_tenth >= 3;
            }

            // Open frame in 10th - just need 2 rolls
            return rolls_in_tenth >= 2;
        }

        false
    }

    // Comprehensive frame information
    fn frame_info(&self) -> FrameInfo {
        let mut frame_indices = Vec::new();
        let mut frame = 0;
        let mut roll_index = 0;

        // Find the starting index for each frame
        while roll_index < self.rolls.len() && frame < 10 {
            frame_indices.push(roll_index);

            // Strike
            if self.rolls[roll_index] == 10 {
                // For frames 1-9, a strike completes the frame
                if frame < 9 {
                    frame += 1;
                    roll_index += 1;
                }
                // For 10th frame, a strike means we need 2 more rolls
                else {
                    // If we have all 3 rolls for 10th frame strike
                    if roll_index + 3 <= self.rolls.len() {
                        roll_index += 3;
                        frame += 1;
                    } else {
                        // Haven't finished 10th frame yet
                        break;
                    }
                }
            }
            // Spare or Open Frame
            else if roll_index + 1 < self.rolls.len() {
                let first_roll = self.rolls[roll_index];
                let second_roll = self.rolls[roll_index + 1];

                // For frames 1-9, a spare or open frame completes the frame
                if frame < 9 {
                    frame += 1;
                    roll_index += 2;
                }
                // For 10th frame
                else {
                    // Spare in 10th frame - need 1 more roll
                    if first_roll + second_roll == 10 {
                        if roll_index + 3 <= self.rolls.len() {
                            roll_index += 3;
                            frame += 1;
                        } else {
                            // Haven't finished 10th frame yet
                            break;
                        }
                    }
                    // Open frame in 10th - just need 2 rolls
                    else {
                        roll_index += 2;
                        frame += 1;
                    }
                }
            }
            // Incomplete frame
            else {
                // Only have the first roll
                break;
            }
        }

        // Determine current frame and roll
        let current_frame = if frame < 10 { Some(frame) } else { None };

        // Calculate roll in current frame
        let roll_in_frame = if let Some(f) = current_frame {
            if f < frame_indices.len() {
                self.rolls.len() - frame_indices[f]
            } else {
                0
            }
        } else {
            0
        };

        FrameInfo {
            frame_start_indices: frame_indices,
            current_frame,
            roll_in_frame,
        }
    }
}

struct FrameInfo {
    frame_start_indices: Vec<usize>,
    current_frame: Option<usize>,
    roll_in_frame: usize,
}
