// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(&mut data); // &mut 可变引用 会改变所有权 & 不可变引用 不会改变所有权
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &mut String) {
    *data = (*data.to_uppercase()).to_string();

    println!("{}", data);
}
