pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }

    let mut output: u32 = 0;
    let len = 1 + num.ilog10();
    let mut input = num;

    while input > 0 {
        let digit = input % 10;
        output += digit.pow(len);
        input /= 10;
    }

    output == num
}
