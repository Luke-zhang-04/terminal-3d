use crate::world_object::WorldObject;
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

pub struct World {
    objects: BTreeMap<u64, Box<dyn WorldObject>>,
    counter: u64,
}

impl World {
    pub fn new() -> World {
        World {
            objects: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_world_object(&mut self, obj: Box<dyn WorldObject>) {
        self.objects.insert(self.counter, obj);
        self.counter += 1;
    }
}

impl Deref for World {
    type Target = BTreeMap<u64, Box<dyn WorldObject>>;

    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}

impl DerefMut for World {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.objects
    }
}
