import init, { Student } from "./pkg/_18_1_wasm_package.js";

async function run() {
    await init();

    let student = new Student("Alice", 22, true, [90, 85, 88]);
    student.render(); // This updates the HTML page
}

run();