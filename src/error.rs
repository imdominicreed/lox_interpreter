pub fn error(line: usize, message: String) {
    report(line, String::from(""), message);
}

pub fn report(line: usize, code: String, message: String) {
    println!("[line {}] Error{}: {}",line, code, message)
}
