

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (y, row) in input.iter().enumerate() {
        let Some(highest) = row.iter().max().copied() else { return result };
        for (x, &v) in row.iter().enumerate().filter(|(_, &v)| v == highest) {
            let lowest = input.iter().filter_map(|row| row.get(x)).min().copied().unwrap();
            if v == lowest {
                result.push((y, x));
            }
        }
    }
    result
}
