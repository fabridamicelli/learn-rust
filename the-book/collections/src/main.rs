fn main() {
    let mut v = vec![1, 2, 3];
    println!("{v:?}");
    v.push(4);

    for i in &v {
        println!("{i}")
    }
    println!("{v:?}");
}
