fn f() -> u32 {
    288999
}
fn main() {
    let mut x = 7;
    println!("val: {x}");
    x = 8;
    println!("val: {x}");

    let _gou: u32 = "34".parse().expect("donow");

    let u: u8 = match f().try_into() {
        Ok(num) => num,
        Err(err) => panic!("{err}"),
    };
    println!("{u}");
}
