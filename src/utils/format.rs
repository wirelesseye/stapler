use std::fmt::Debug;

pub fn join_list<T>(list: &[T], separator: &str) -> String
where
    T: Debug,
{
    let mut buff = String::new();
    for (i, item) in list.iter().enumerate() {
        if i > 0 {
            buff.push_str(separator);
        }
        buff.push_str(&format!("{:?}", item))
    }
    return buff;
}
