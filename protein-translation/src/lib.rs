pub fn translate(rna: &str) -> Option<Vec<&str>> {
    match rna {
        Some(rna_str) => {
            // Your existing translation logic here
            // Return a Vec<String> instead of Option<Vec<&str>>
        }
        None => vec![String::new()],
    }
}
