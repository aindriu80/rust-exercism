// #[derive(Debug, PartialEq, Eq)]
// pub enum Error {
//     NotEnoughPinsLeft,
//     GameComplete,
// }
//
// // Define FramesInfo outside of the impl block
// struct FramesInfo {
//     frame_start_indices: Vec<usize>,
//     current_frame: Option<usize>,
//     current_roll: usize,
//     rolls_in_frame: usize,
//     is_complete: bool,
// }
//
// pub struct BowlingGame {
//     rolls: Vec<u16>,
// }
//
// impl BowlingGame {
//     pub fn new() -> Self {
//         BowlingGame { rolls: Vec::new() }
//     }
//
//     pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
//         if pins > 10 {
//             return Err(Error::NotEnoughPinsLeft);
//         }
//
//         let frames_info = self.frames_info();
//
//         // Game is complete, no more rolls allowed
//         if frames_info.is_complete {
//             return Err(Error::GameComplete);
//         }
//
//         // Check for frame-specific pin constraints
//         if let Some(frame) = frames_info.current_frame {
//             // Regular frame (not 10th)
//             if frame < 9 && frames_info.current_roll == 1 {
//                 let prev_roll = self.rolls[self.rolls.len() - 1];
//                 if prev_roll != 10 && prev_roll + pins > 10 {
//                     return Err(Error::NotEnoughPinsLeft);
//                 }
//             }
//             // Special handling for 10th frame
//             else if frame == 9 {
//                 if frames_info.current_roll == 1 {
//                     // Second roll in 10th frame - check if first wasn't a strike
//                     let first_roll = self.rolls[self.rolls.len() - 1];
//                     if first_roll != 10 && first_roll + pins > 10 {
//                         return Err(Error::NotEnoughPinsLeft);
//                     }
//                 } else if frames_info.current_roll == 2 {
//                     // Third roll (bonus) in 10th frame
//                     let tenth_frame_start = frames_info.frame_start_indices[9];
//                     let first_roll = self.rolls[tenth_frame_start];
//
//                     if frames_info.rolls_in_frame == 2 {
//                         // If first roll was a strike, second roll was not
//                         if first_roll == 10 {
//                             let second_roll = self.rolls[tenth_frame_start + 1];
//                             if second_roll != 10 && second_roll + pins > 10 {
//                                 return Err(Error::NotEnoughPinsLeft);
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//
//         self.rolls.push(pins);
//         Ok(())
//     }
//
//     pub fn score(&self) -> Option<u16> {
//         let frames_info = self.frames_info();
//
//         // Game must be complete to calculate score
//         if !frames_info.is_complete {
//             return None;
//         }
//
//         let mut total = 0;
//         let mut roll_idx = 0;
//
//         // Score first 9 frames
//         for frame in 0..9 {
//             match self.score_frame(roll_idx) {
//                 Some((score, next_idx)) => {
//                     total += score;
//                     roll_idx = next_idx;
//                 }
//                 None => return None,
//             }
//         }
//
//         // Score 10th frame
//         match self.score_tenth_frame(roll_idx) {
//             Some(score) => Some(total + score),
//             None => None,
//         }
//     }
//
//     // Helper methods
//     fn score_frame(&self, roll_idx: usize) -> Option<(u16, usize)> {
//         if roll_idx >= self.rolls.len() {
//             return None;
//         }
//
//         // Strike
//         if self.rolls[roll_idx] == 10 {
//             if roll_idx + 2 >= self.rolls.len() {
//                 return None;
//             }
//             let score = 10 + self.rolls[roll_idx + 1] + self.rolls[roll_idx + 2];
//             return Some((score, roll_idx + 1));
//         }
//
//         // Open frame or spare
//         if roll_idx + 1 >= self.rolls.len() {
//             return None;
//         }
//
//         let first_roll = self.rolls[roll_idx];
//         let second_roll = self.rolls[roll_idx + 1];
//
//         // Spare
//         if first_roll + second_roll == 10 {
//             if roll_idx + 2 >= self.rolls.len() {
//                 return None;
//             }
//             let score = 10 + self.rolls[roll_idx + 2];
//             return Some((score, roll_idx + 2));
//         }
//
//         // Open frame
//         let score = first_roll + second_roll;
//         Some((score, roll_idx + 2))
//     }
//
//     fn score_tenth_frame(&self, roll_idx: usize) -> Option<u16> {
//         if roll_idx >= self.rolls.len() {
//             return None;
//         }
//
//         let first_roll = self.rolls[roll_idx];
//
//         // Strike in 10th frame
//         if first_roll == 10 {
//             if roll_idx + 2 >= self.rolls.len() {
//                 return None;
//             }
//             return Some(10 + self.rolls[roll_idx + 1] + self.rolls[roll_idx + 2]);
//         }
//
//         // Need at least two rolls for 10th frame
//         if roll_idx + 1 >= self.rolls.len() {
//             return None;
//         }
//
//         let second_roll = self.rolls[roll_idx + 1];
//
//         // Spare in 10th frame
//         if first_roll + second_roll == 10 {
//             if roll_idx + 2 >= self.rolls.len() {
//                 return None;
//             }
//             return Some(10 + self.rolls[roll_idx + 2]);
//         }
//
//         // Open frame in 10th
//         Some(first_roll + second_roll)
//     }
//
//     // Comprehensive frame information to help with both scoring and validation
//     fn frames_info(&self) -> FramesInfo {
//         let mut frame_indices = Vec::with_capacity(10);
//         let mut roll_idx = 0;
//         let mut frames = 0;
//
//         // Track starting index of each frame
//         while roll_idx < self.rolls.len() && frames < 10 {
//             frame_indices.push(roll_idx);
//
//             if self.rolls[roll_idx] == 10 {
//                 // Strike
//                 roll_idx += 1;
//             } else if roll_idx + 1 < self.rolls.len() {
//                 // Open frame or spare
//                 roll_idx += 2;
//             } else {
//                 // Incomplete frame (only one roll)
//                 roll_idx += 1;
//                 frames += 1;
//                 break;
//             }
//
//             frames += 1;
//         }
//
//         // Fill in any missing frames
//         while frames < 10 {
//             frame_indices.push(roll_idx);
//             frames += 1;
//         }
//
//         let mut current_frame = None;
//         let mut current_roll = 0;
//         let mut rolls_in_frame = 0;
//         let mut is_complete = false;
//
//         // If we haven't completed 10 frames
//         if frames < 10 {
//             current_frame = Some(frames - 1);
//             current_roll = if roll_idx == self.rolls.len() { 1 } else { 0 };
//             rolls_in_frame = current_roll;
//         }
//         // If we're in or past the 10th frame
//         else {
//             // We're in the 10th frame
//             let tenth_frame_idx = frame_indices[9];
//
//             if roll_idx < self.rolls.len() {
//                 current_frame = Some(9);
//                 rolls_in_frame = self.rolls.len() - tenth_frame_idx;
//                 current_roll = rolls_in_frame;
//
//                 // Determine if the game is complete based on 10th frame state
//                 if tenth_frame_idx < self.rolls.len() {
//                     let first_roll = self.rolls[tenth_frame_idx];
//
//                     if first_roll == 10 {
//                         // Strike: need two more rolls
//                         is_complete = rolls_in_frame >= 3;
//                     } else if tenth_frame_idx + 1 < self.rolls.len() {
//                         let second_roll = self.rolls[tenth_frame_idx + 1];
//                         if first_roll + second_roll == 10 {
//                             // Spare: need one more roll
//                             is_complete = rolls_in_frame >= 3;
//                         } else {
//                             // Open frame: just need two rolls
//                             is_complete = rolls_in_frame >= 2;
//                         }
//                     }
//                 }
//             } else {
//                 // No rolls in 10th frame yet
//                 current_frame = Some(9);
//                 current_roll = 0;
//                 is_complete = false;
//             }
//         }
//
//         FramesInfo {
//             frame_start_indices: frame_indices,
//             current_frame,
//             current_roll,
//             rolls_in_frame,
//             is_complete,
//         }
//     }
// }

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
