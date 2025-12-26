use crate::world_object::WorldObject;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

pub struct World {
    objects: HashMap<i64, Box<dyn WorldObject>>,
    counter: i64,
}

impl World {
    pub fn new() -> World {
        World {
            objects: HashMap::new(),
            counter: 0,
        }
    }

    pub fn add_world_object(&mut self, obj: Box<dyn WorldObject>) {
        self.counter += 1;

        self.objects.insert(self.counter - 1, obj);
    }
}

impl Deref for World {
    type Target = HashMap<i64, Box<dyn WorldObject>>;

    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}

impl DerefMut for World {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.objects
    }
}
