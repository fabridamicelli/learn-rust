fn main() {
    let mut s = String::from("hello");

    s.push_str(&s.clone());

    println!("{s}");

    let s = "hi".to_string();
    let ss = "hi".to_string();
    let sss = s + &ss;
    println!("{sss}");

    let a1 = "hi".to_string();
    let a2 = "hi".to_string();
    let a3 = "hi".to_string();

    let a = format!("{a1}-{a2}-{a3}");

    println!("{a}");
    println!("{a1}");

    let h = "नमस्ते".to_string();
    println!("{h}");

    for c in h.chars() {
        println!("{c}")
    }
}
