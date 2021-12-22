use std::collections::HashMap;
use crate::types::location;
use crate::types::order;

struct OrderFeature {
    id: u64,
    passenger_id: u64,
    from: location::Point,
    to: location::Point,
    price: u64,
    order_type: order::OrderType,
    order_status: order::OrderStatus,
}

impl OrderFeature {
    fn new(id: u64, passenger_id: u64, from: location::Point, to: location::Point, price: u64, order_type: order::OrderType, order_status: order::OrderStatus) -> OrderFeature {
        OrderFeature {
            id: id,
            passenger_id: passenger_id,
            from: from,
            to: to,
            price: price,
            order_type: order_type,
            order_status: order_status,
        }
    }
}

struct OrderFeatureMap {
    features: HashMap<u64, OrderFeature>,
}

impl OrderFeatureMap {
    fn new() -> OrderFeatureMap {
        OrderFeatureMap {
            features: HashMap::new(),
        }
    }

    fn add(&mut self, id: u64, passenger_id: u64, from: location::Point, to: location::Point, price: u64, order_type: order::OrderType, order_status: order::OrderStatus) {
        self.features.insert(id, OrderFeature::new(id, passenger_id, from, to, price, order_type, order_status));
    }

    fn get(&self, id: u64) -> Option<&OrderFeature> {
        self.features.get(&id)
    }

    fn get_mut(&mut self, id: u64) -> Option<&mut OrderFeature> {
        self.features.get_mut(&id)
    }

    fn remove(&mut self, id: u64) {
        self.features.remove(&id);
    }
}


#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_new_order_feature() {
        let of = OrderFeature::new(1, 2, location::Point::new(1.0, 2.0), location::Point::new(3.0, 4.0), 5,  order::OrderType::Taxi, order::OrderStatus::New);
        assert_eq!(of.price, 5);
    }

}
