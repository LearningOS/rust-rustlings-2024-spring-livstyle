// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


// 不能在同一个作用域中同时存在可变引用和不可变引用或多个可变引用

fn main() {
    let mut x = 100;
    {
        let y = &mut x;
        *y += 100;
    }
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
