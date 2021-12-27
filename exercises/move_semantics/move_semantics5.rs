// move_semantics5.rs
// Make me compile only be reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

fn main() {
    let mut x = 100; // The benefit of having this restriction
    let y = &mut x;  // is that Rust can prevent data races at compile time.
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
