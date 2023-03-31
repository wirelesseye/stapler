use std::fmt::Debug;

pub fn format_list<T>(list: &Vec<T>, sep: &str) -> String where T: Debug {
    let outputs: Vec<String> = list
        .iter()
        .map(|i| format!("{:?}", i))
        .collect();
    outputs.join(sep)
}