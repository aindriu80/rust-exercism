// use rand::Rng;
//
// pub struct Robot {
//     pub name: String,
// }
//
// impl Robot {
//     pub fn new() -> Self {
//         Robot {
//             name: Self::generate_name(),
//         }
//     }
//
//     pub fn name(&self) -> &str {
//         &self.name
//     }
//
//     pub fn reset_name(&mut self) {
//         self.name = Self::generate_name();
//     }
//
//     fn generate_name() -> String {
//         let mut rng = rand::thread_rng();
//
//         // Generate 2 random uppercase ASCII letters
//         let letter1 = (rng.gen_range(0..26) as u8 + b'A') as char;
//         let letter2 = (rng.gen_range(0..26) as u8 + b'A') as char;
//
//         // Generate 3 random digits
//         let digit1 = rng.gen_range(0..10);
//         let digit2 = rng.gen_range(0..10);
//         let digit3 = rng.gen_range(0..10);
//
//         format!("{}{}{}{}{}", letter1, letter2, digit1, digit2, digit3)
//     }
// }

use once_cell::sync::Lazy;
use rand::Rng;
use std::collections::HashSet;
use std::sync::Mutex;

// Global state to track used robot names
static USED_NAMES: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub struct Robot {
    pub name: String,
}

impl Robot {
    pub fn new() -> Self {
        let name = Self::generate_unique_name();
        Robot { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        // Remove the old name from used names
        {
            let mut used_names = USED_NAMES.lock().unwrap();
            used_names.remove(&self.name);
        }

        // Generate a new unique name
        self.name = Self::generate_unique_name();
    }

    fn generate_name() -> String {
        let mut rng = rand::thread_rng();

        // Generate 2 random uppercase ASCII letters
        let letter1 = (rng.gen_range(0..26) as u8 + b'A') as char;
        let letter2 = (rng.gen_range(0..26) as u8 + b'A') as char;

        // Generate 3 random digits
        let digit1 = rng.gen_range(0..10);
        let digit2 = rng.gen_range(0..10);
        let digit3 = rng.gen_range(0..10);

        format!("{}{}{}{}{}", letter1, letter2, digit1, digit2, digit3)
    }

    fn generate_unique_name() -> String {
        loop {
            let new_name = Self::generate_name();

            // Check if the name is already used
            let mut used_names = USED_NAMES.lock().unwrap();
            if !used_names.contains(&new_name) {
                // If not used, add it to the set and return it
                used_names.insert(new_name.clone());
                return new_name;
            }

            // If the name is already used, loop and try again
        }
    }
}
