use std::fs;
use crate::model::Student;


/// get all students from the file and store them in the vector 
pub fn load_students() -> Vec<Student> {
    fs::read_to_string("students.json")
        .map(|data| serde_json::from_str(&data).unwrap_or_else(|_| vec![]))
        .unwrap_or_else(|_| vec![])
}


/// save the vector of students to the file
pub fn save_students(students: &[Student]) {
    fs::write("students.json", serde_json::to_string_pretty(students).unwrap()).unwrap();
}