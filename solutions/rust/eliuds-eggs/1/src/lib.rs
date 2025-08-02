pub fn egg_count(mut display_value: u32) -> usize {
    let mut count = 0;
    while display_value > 0 {
        count += (display_value & 1) as usize;
        display_value >>= 1;
    }  
    count
}
