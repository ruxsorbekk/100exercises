fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    if time_elapsed != 0 {
        (end - start) / time_elapsed
    } else {
        panic!("The journey took no time at all. That's impossible!");        
    }
}

#[cfg(test)]
mod tests {
    use crate::speed;
    
    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }
    
    #[test]
    #[should_panic(expected = "The journey took no time at all. That's impossible")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}