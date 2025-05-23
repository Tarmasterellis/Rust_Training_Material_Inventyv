pub mod model;
pub mod api;
pub mod handler;

use axum::{routing::get, Router};
use std::{ net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use model::Student;
use api::{get_students, get_student, add_student, update_student, delete_student};

type SharedState = Arc<Mutex<Vec<Student>>>;


#[tokio::main]
async fn main() {
    // Load student data from a file and initialize the shared state
    let students = handler::load_students();

    // Wrap the students vector in a Mutex and Arc
    let state = Arc::new(Mutex::new(students));

    /// Calling Api From Following Curl Command
        // curl -X GET http://127.0.0.1:4500/students
        // curl -X POST http://127.0.0.1:4500/students -H "Content-Type: application/json" -d "{ \"name\": \"Aman\", \"email\": \"aman@example.com\", \"mobile\": \"9876543210\" }"
        // curl -X PUT http://127.0.0.1:4500/students/5666cc48-2f9d-4db9-8725-5f7b5bb50231 -H "Content-Type: application/json" -d "{ \"name\": \"Aman Verasia\", \"email\": \"aman@example.com\", \"mobile\": \"9876543210\" }"
        // curl -X DELETE http://127.0.0.1:4500/students/5666cc48-2f9d-4db9-8725-5f7b5bb50231

    let app = Router::new()
        .route("/students", get(get_students).post(add_student))
        .route("/students/{id}", get(get_student).put(update_student).delete(delete_student))
        .with_state(state);

        let addr = SocketAddr::from(([127, 0, 0, 1], 4500));
        println!("Server running at http://{}", addr);
    
        // Create a TCP listener
        let listener = TcpListener::bind(addr).await.unwrap();
    
        // Run the server
        axum::serve(listener, app)
            .await
            .unwrap();
}