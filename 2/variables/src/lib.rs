pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    let distance = end - start;
    
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;
    
    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }
    
    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }
    
    #[test]
    fn case3() {
        assert_eq!(speed(10, 31, 10), 2);
    }
}