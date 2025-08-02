use std::fmt::Write;

pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();
    if !list.is_empty() {
        for i in 0..list.len()-1 {
            writeln!(result, "For want of a {} the {} was lost.", list[i], list[i+1]).unwrap();
        }
        write!(result, "And all for the want of a {}.", list[0]).unwrap();
    }
    result
}
