use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element, HtmlElement};

// Expose function to JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub struct Student {
    name: String,
    age: i32,
    is_active: bool,
    grades: Vec<i32>,
}

#[wasm_bindgen]
impl Student {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, age: i32, is_active: bool, grades: Vec<i32>) -> Student {
        Student { name, age, is_active, grades }
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        let document: Document = window().unwrap().document().unwrap();

        // Explicitly cast to HtmlElement to get `body()`
        let body: HtmlElement = document.body().expect("document should have a body");

        let student_info = format!(
            "<h2>Student Details</h2>
             <p><strong>Name:</strong> {}</p>
             <p><strong>Age:</strong> {}</p>
             <p><strong>Active:</strong> {}</p>
             <p><strong>Grades:</strong> {:?}</p>",
            self.name, self.age, self.is_active, self.grades
        );

        let div: Element = document.create_element("div").unwrap();
        div.set_inner_html(&student_info);
        body.append_child(&div).unwrap();
    }
}