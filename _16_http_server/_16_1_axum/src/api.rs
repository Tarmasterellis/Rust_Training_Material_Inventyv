use axum::{extract::{Path, State}, http::StatusCode, response::Json};
use uuid::Uuid;
use crate::{model::Student, handler::save_students, SharedState};



/// get all the students from the file - Command "curl -X GET http://127.0.0.1:4500/students"
pub async fn get_students(State(state): State<SharedState>) -> Json<Vec<Student>> {
    let students = state.lock().await;
    Json(students.clone())
}

/// get a student by id
/// curl -X GET http://127.0.0.1:4500/students/{id}
pub async fn get_student(Path(id) : Path<String>, State(state): State<SharedState>) -> Result<Json<Student>, StatusCode> {
    let students = state.lock().await;
    students.iter().find(|s| s.id == id).cloned().map(Json).ok_or(StatusCode::NOT_FOUND)
}

/// add a new student
/// curl -X POST http://127.0.0.1:4500/students -H "Content-Type: application/json" -d "{ \"name\": \"Aman\", \"email\": \"aman@example.com\", \"mobile\": \"9876543210\" }"
pub async fn add_student(State(state): State<SharedState>, Json(mut student): Json<Student>) -> StatusCode {
    student.id = Uuid::new_v4().to_string();
    let mut students = state.lock().await;
    students.push(student);
    save_students(&students);
    StatusCode::CREATED
}

/// update a student
/// curl -X PUT http://127.0.0.1:4500/students/{id} -H "Content-Type: application/json" -d "{ \"name\": \"Aman Verasia\", \"email\": \"aman@example.com\", \"mobile\": \"9876543210\" }"
pub async fn update_student(Path(id): Path<String>, State(state): State<SharedState>, Json(updated_student): Json<Student>) -> StatusCode {
    let mut students = state.lock().await;
    if let Some(student) = students.iter_mut().find(|s| s.id == id) {
        student.name = updated_student.name;
        student.email = updated_student.email;
        student.mobile = updated_student.mobile;
        save_students(&students);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}


/// delete a student
/// curl -X DELETE http://127.0.0.1:4500/students/{id}
pub async fn delete_student(Path(id): Path<String>, State(state): State<SharedState>) -> StatusCode {
    let mut students = state.lock().await;
    if students.iter().any(|s| s.id == id) {
        students.retain(|s| s.id != id);
        save_students(&students);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}