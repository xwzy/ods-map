use crate::types::location;

#[derive(Debug)]
pub enum OrderType {
    Taxi,
    FlashCar,
}

#[derive(Debug)]
pub enum OrderStatus {
    New,
    Accepted,
    Cancelled,
    Completed,
}

#[derive(Debug)]
pub struct Order {
    id: u64,
}


#[test]
fn test_a() {
    let p = location::Point { x: 1, y: 2 };
}
