pub struct School {
    students: std::collections::HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students: std::collections::HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        // First check if this student exists in ANY grade
        let student_exists = self
            .students
            .values()
            .any(|students| students.contains(&student.to_string()));

        // If student already exists somewhere, don't add them again
        if !student_exists {
            // Get or create the vector for this grade
            let students_in_grade = self.students.entry(grade).or_insert_with(Vec::new);

            // Add the student
            students_in_grade.push(student.to_string());

            // Sort the students
            students_in_grade.sort();
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        // Get all grade numbers that have at least one student
        let mut grades: Vec<u32> = self.students.keys().cloned().collect();

        // Sort the grades
        grades.sort();

        // Return the sorted vector
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut result = self.students.get(&grade).cloned().unwrap_or_else(Vec::new);
        result.sort();
        result
    }
}
