use crate::{deserialize_from_file, generate_combinations, Hanzi, generate_binary_numbers};
use std::collections::HashSet;

#[test]
fn are_all_strings_unique() {
    let hanzis = deserialize_from_file("src/files/input.json");
    let result: Vec<Hanzi> = generate_combinations(hanzis);

    for hanzi in result {
        if let Some(res) = hanzi.result {
            let set: HashSet<&String> = res.iter().collect();
            assert_eq!(set.len(), res.len());
        }
    }
}

#[test]
fn combinations_length() {
    let hanzis = deserialize_from_file("src/files/input.json");
    let result: Vec<Hanzi> = generate_combinations(hanzis);

    let length = |hanzi: &str| {
        result.iter().find(|h| h.id == hanzi.to_string()).unwrap().result.as_ref().unwrap().len()
    };

    assert_eq!(length("休"), 2*5);
    assert_eq!(length("体"), 2*6);
    assert_eq!(length("木"), 2*2);
    assert_eq!(length("林"), 5*5);
    assert_eq!(length("森"), 5*5*5);
    
}

#[test]
fn binary_number_eq() {
    assert_eq!(generate_binary_numbers(1), ["0", "1"]);
    assert_eq!(generate_binary_numbers(2), ["00", "01", "10", "11"]);
    assert_eq!(generate_binary_numbers(3), [
                                            "000",
                                            "001",
                                            "010",
                                            "011",
                                            "100",
                                            "101",
                                            "110",
                                            "111",
                                                  ]);

}