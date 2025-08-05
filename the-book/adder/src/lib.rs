pub fn addone(x: i64) -> i64 {
    x + 1
}

fn addone2(x: i64) -> Result<i64, String> {
    if x < 0 {
        return Err("Invalid input".to_string());
    }
    Ok(x + 1)
}

#[cfg(test)]

mod tests {
    use super::{addone, addone2};

    #[test]
    fn case1() {
        let x = 1;
        assert_eq!(addone(x), 2);
    }
    #[test]
    #[should_panic]
    fn case2() {
        let f = { |_: i64| panic!("panicoo") };
        f(1);
    }

    #[test]
    fn case3() {
        let got = addone2(0);
        assert_eq!(got.unwrap(), 1);

        let got = addone2(-1);
        assert!(got.is_err())
    }
}
