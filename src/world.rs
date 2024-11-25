use crate::storage::Storage;

pub struct World {
    pub storage: Storage,

    pub end_loop: bool
}

impl World {
    pub fn new() -> Self {
        Self {
            storage: Storage::new(),
            end_loop: false
        }
    }


    //pub fn spawn(&mut self, components)
}