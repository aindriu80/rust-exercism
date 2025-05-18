pub struct RailFence {
    rails: u32,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence { rails }
    }

    pub fn encode(&self, text: &str) -> String {
        // handle the edge cases
        if self.rails <= 1 || text.is_empty() {
            return text.to_string();
        }

        // Create a vector of strings, one for each rail
        let mut rails_content: Vec<String> = vec![String::new(); self.rails as usize];

        // Variables to track current rail and direction
        let mut current_rail = 0;
        let mut going_down = true;

        // Place each character on the appropriate rail
        for ch in text.chars() {
            // Add character to current rail
            rails_content[current_rail].push(ch);

            // Move to next rail
            if going_down {
                current_rail += 1;
                // If we hit the bottom rail, change direction
                if current_rail == self.rails as usize - 1 {
                    going_down = false;
                }
            } else {
                current_rail -= 1;
                // If we hit the top rail, change direction
                if current_rail == 0 {
                    going_down = true;
                }
            }
        }

        // Concatenate all rails to create the final encoded string
        rails_content.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        // Handle edge cases
        if self.rails <= 1 || cipher.is_empty() {
            return cipher.to_string();
        }

        // Step 1: Figure out how many characters go on each rail
        let mut rail_lengths = vec![0; self.rails as usize];
        let mut current_rail = 0;
        let mut going_down = true;

        // Simulate the zigzag to count characters per rail
        for _ in 0..cipher.len() {
            rail_lengths[current_rail] += 1;

            // Move to next rail (same logic as encode)
            if going_down {
                current_rail += 1;
                if current_rail == self.rails as usize - 1 {
                    going_down = false;
                }
            } else {
                current_rail -= 1;
                if current_rail == 0 {
                    going_down = true;
                }
            }
        }
        // Step 2: Split the cipher text into rails
        let mut rails: Vec<Vec<char>> = Vec::new();
        let mut start_index = 0;

        for &length in &rail_lengths {
            let end_index = start_index + length;
            let rail_chars: Vec<char> = cipher[start_index..end_index].chars().collect();
            rails.push(rail_chars);
            start_index = end_index;
        }

        // Step 3: Read characters back in zigzag order
        let mut result = String::new();
        let mut rail_indices = vec![0; self.rails as usize];
        current_rail = 0;
        going_down = true;

        for _ in 0..cipher.len() {
            // Take next character from current rail
            result.push(rails[current_rail][rail_indices[current_rail]]);
            rail_indices[current_rail] += 1;

            // Move to next rail (same zigzag logic)
            if going_down {
                current_rail += 1;
                if current_rail == self.rails as usize - 1 {
                    going_down = false;
                }
            } else {
                current_rail -= 1;
                if current_rail == 0 {
                    going_down = true;
                }
            }
        }

        result
    }
}
