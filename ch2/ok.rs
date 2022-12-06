fn main() {
    println!("OK");

    let x: f32 = 1.0 / 0.0;
    println!("{}", x);
    assert!(x.is_finite());
}
