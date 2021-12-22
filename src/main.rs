mod types;

use crate::types::location;

fn main() {
    let p = types::location::Point { x: 0, y: 0 };
    println!("{:?}", p);
}


//
// #[derive(Debug)]
// struct Rider {
//     id: u64,
//     name: String,
//     position: Point,
// }
//
//
// #[derive(Debug)]
// struct World {
//     id: u64,
//     height: u32,
//     weight: u32,
//
//     orders: Vec<Order>,
//     riders: Vec<Rider>,
// }
// struct Action {
//
// }
//
// #[derive(Debug)]
// struct WorldSnapshot {
//     timestamp: u64,
//     world: World
// }
//
// #[derive(Debug)]
// struct WorldSnapshotStore {
//     snapshots: Vec<WorldSnapshot>
// }
//
// #[derive(Debug)]
// enum TaskStatus {
//     Pending,
//     InProgress,
//     Canceled,
//     Done,
// }
//
// struct Task {
//     id: u64,
//     info: String,
//     status: TaskStatus,
//     world_snapshot_store: WorldSnapshotStore,
//     created_at: u64,
//     updated_at: u64,
//     deleted_at: u64,
// }
