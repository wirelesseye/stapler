use crate::asts::ASTTrait;

pub fn format_list<T>(list: &Vec<T>) -> String where T: ASTTrait {
    let outputs: Vec<String> = list
        .iter()
        .map(|i| format!("{:?}", i))
        .collect();
    outputs.join(", ")
}