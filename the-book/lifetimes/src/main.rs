fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

#[derive(Debug)]
struct Car<'a> {
    name: &'a str,
}

fn main() {
    let x = "hola";
    let y = "hola";
    let l = longest(x, y);

    let c = Car { name: "fiat" };
    println!("l: {l}");
    println!("c: {}", c.name);
}
