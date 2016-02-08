// This module implements various utilities.

pub fn join_string(list: Vec<String>, seq: &str) -> String {
    list.iter().fold(String::new(), |a, b| if a.len() > 0 { a + seq } else { a } + &b)
}
