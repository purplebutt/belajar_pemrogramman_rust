pub fn exp(base: u64, pwr: u64) -> u64 {
    let mut result = 1;
    let mut pwr = pwr;
    while pwr > 0 {
        result *= base;
        pwr -= 1;
    }
    result
}