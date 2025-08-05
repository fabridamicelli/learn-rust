use adder::addone;
#[test]
fn add_one() {
    let got = addone(20);
    assert_eq!(got, 21)
}
