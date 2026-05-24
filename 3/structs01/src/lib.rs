pub struct Order {
    price: u32,
    quantity: u32
}

impl Order {
    pub fn is_available(&self) -> bool {
        if self.quantity > 0 {
            true
        } else {
            false
        }   
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }
    
    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}