// This module implements various utilities.

pub fn join_string(list: Vec<String>, seq: &str) -> String {
    list.iter().fold(String::new(), |a, b| if a.len() > 0 { a + seq } else { a } + &b)
}

macro_rules! try_return(
    ($e:expr) => {{
        match $e {
            Ok(v) => v,
            Err(e) => { error!("Error: {}", e); return; }
        }
    }}
);
