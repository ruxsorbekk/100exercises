use outro12::Order;

#[test]
fn test_order() {
    let mut order = Order::new("product_name".into(), 10, 100);
    
    assert_eq!(order.product_name(), "product_name");
    assert_eq!(order.quantity(), &10);
    assert_eq!(order.unit_price(), &100);
    assert_eq!(order.total(), 1000);
    
    order.set_product_name("Rust book".into());
    order.set_quantity(5000);
    order.set_unit_price(25);
    
    assert_eq!(order.product_name(), "Rust book");
    assert_eq!(order.quantity(), &5000);
    assert_eq!(order.unit_price(), &25);
    assert_eq!(order.total(), 125000);
}

#[test]
#[should_panic(expected="Product name cannot be empty!")]
fn test_empty_name() {
    Order::new("".into(), 10, 100);
}

#[test]
#[should_panic(expected="Product name cannot be longer than 300 bytes!")]
fn test_long_name() {
    Order::new("hello".repeat(500), 10, 100);
}

#[test]
#[should_panic]
fn test_zero_quantity() {
    Order::new("product_name".into(), 0, 100);
}

#[test]
#[should_panic]
fn test_zero_price() {
    Order::new("product_name".into(), 100, 0);
}
