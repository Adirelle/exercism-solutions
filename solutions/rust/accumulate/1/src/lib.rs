pub fn map<T, U>(input: Vec<T>, mut function: impl FnMut(T) -> U) -> Vec<U> {
    let mut result = Vec::new();
    for item in input.into_iter() {
        result.push(function(item))
    }
    result
}
