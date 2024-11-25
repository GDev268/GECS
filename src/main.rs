use storage::Storage;

mod storage;
mod entity;
mod world;
mod system;
mod query;

fn main() {
    let storage = Storage::new();
    
    println!("Hello, world!");
}