use std::{cmp::Ordering, collections::HashMap};
fn main() {
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    //
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
fn median(v: Vec<i64>) -> f64 {
    let mut vv = v.clone();
    vv.sort();
    println!("{vv:?}");
    let middle = (v.len() / 2) - 1;
    if vv.len() % 2 == 0 {
        return (vv[middle] + vv[middle + 1]) as f64 / 2.0;
    } else {
        return vv[middle + 1] as f64;
    }
}

fn mode(v: Vec<i64>) -> i64 {
    let mut count: HashMap<i64, i64> = HashMap::new();
    for key in &v {
        let val = count.entry(*key).or_insert(0);
        *val += 1;
    }
    let mut count: Vec<(&i64, &i64)> = count.iter().collect();
    count.sort_by(|a, b| {
        let cmp = a.1.cmp(&b.1);
        if cmp == Ordering::Equal {
            a.0.cmp(b.0)
        } else {
            cmp
        }
    });

    *count.last().unwrap().0
    //     let mut out = v[0];
    //     let max_count = 0;
    //     for (num, val) in count.iter() {
    //         if *val > max_count {
    //             out = *num;
    //         }
    //     }
    //     out
}

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
fn starts_with_consonant(s: &str) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    if vowels.contains(&s.chars().next().unwrap()) {
        return false;
    }
    return true;
}
// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

fn add_person_to_department(person: &str, dep: &str, register: &mut HashMap<String, Vec<String>>) {
    let names = register.entry(dep.to_string()).or_insert_with(Vec::new);

    if !names.contains(&person.to_string()) {
        names.push(person.to_string());
    }
}

fn to_platin(s: &str) -> String {
    let (first, rest) = s.split_at(1); // s.chars().take(1).last().unwrap();
    match starts_with_consonant(s) {
        // out = out + "-" + &String::from(first) + "ay";
        // for (i, c) in s.char_indices() {
        //     if i > 0 {
        //         out.push(c);
        //     }
        // }
        // out = out + "-" + &String::from(first) + "ay";
        // }
        // false => {
        //     for c in s.chars() {
        //         out.push(c);
        //     }
        //     out = out + "-hay";
        // }
        true => return format!("{}-{}ay", rest, first),
        false => return format!("{}-hay", s),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{add_person_to_department, median, mode, to_platin};

    #[test]
    fn case1() {
        assert_eq!(median(vec![1, 10, 3]), 3.);
        assert_eq!(median(vec![1, 2, 3]), 2.);
        assert_eq!(median(vec![1, 2, 3, 4]), 2.5);
    }

    #[test]
    fn case2() {
        assert_eq!(mode(vec![1, 10, 3]), 10);
        assert_eq!(mode(vec![22, 33, 44]), 44);
        assert_eq!(mode(vec![1, 2, 2, 3]), 2);
        assert_eq!(mode(vec![1, 2, 3, 3, 3, 3, 4]), 3);
    }
    #[test]
    fn case3() {
        assert_eq!(to_platin("first"), "irst-fay");
        assert_eq!(to_platin("apple"), "apple-hay");
        assert_eq!(to_platin("hello"), "ello-hay");
        assert_eq!(to_platin("jello"), "ello-jay");
        assert_eq!(to_platin("ahoi"), "ahoi-hay");
    }

    #[test]
    fn case4() {
        let mut reg: HashMap<String, Vec<String>> = HashMap::new();
        add_person_to_department("sally", "eng", &mut reg);
        assert_eq!(reg.get("eng").unwrap().contains(&"sally".to_string()), true);
        add_person_to_department("bob", "eng", &mut reg);
        assert_eq!(reg.get("eng").unwrap().contains(&"bob".to_string()), true);
        add_person_to_department("bob", "eng", &mut reg);
        assert_eq!(reg.len(), 1); // only 1 dep added
        assert_eq!(reg.get("eng").unwrap().len(), 2); // no repeated person in a dep
    }
}
