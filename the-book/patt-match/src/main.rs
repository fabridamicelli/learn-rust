fn main() {
    let n = 10;
    match n {
        n if n > 5 => {
            println!("first branch")
        }
        n if n > 3 => {
            println!("first branch")
        }
        other => {
            println!("nothing here {other}")
        }
    }
}
