fn main() {
    let order = Order {
        id: 123,
        from: Point { x: 1, y: 2 },
        to: Point { x: 3, y: 4 },
    };
    println!("{:?}", order);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
enum OrderType {
    Taxi,
    FlashCar,
}

#[derive(Debug)]
struct Order {
    id: u64,
    from: Point,
    to: Point,
    price: u64,
    order_type: OrderType
}

#[derive(Debug)]
struct Rider {
    id: u64,
    name: String,
    position: Point,
}


#[derive(Debug)]
struct World {
    id: u64,
    height: u32,
    weight: u32,

    orders: Vec<Order>,
    riders: Vec<Rider>,
}
struct Action {

}

#[derive(Debug)]
struct WorldSnapshot {
    timestamp: u64,
    world: World
}

#[derive(Debug)]
struct WorldSnapshotStore {
    snapshots: Vec<WorldSnapshot>
}

#[derive(Debug)]
enum TaskStatus {
    Pending,
    InProgress,
    Canceled,
    Done,
}

struct Task {
    id: u64,
    info: String,
    status: TaskStatus,
    world_snapshot_store: WorldSnapshotStore,
    created_at: u64,
    updated_at: u64,
    deleted_at: u64,
}
