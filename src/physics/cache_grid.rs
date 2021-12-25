/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

use std::collections::{hash_map::Entry, VecDeque};

use bevy::{
    ecs::entity::Entity,
    utils::{HashMap, HashSet}, 
};

use crate::collision::{Shape, Projection};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CacheGridKey(i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CacheGridAxis {
    near: i32,
    far:  i32    
}

impl CacheGridAxis {
    fn from(projection: Projection, scale: f32) -> Self {
        Self{
            near: (projection.near()*scale).floor() as i32,
            far:  (projection.far() *scale).ceil()  as i32
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CacheGridData {
    x: CacheGridAxis, 
    y: CacheGridAxis,
}

impl CacheGridData {
    pub fn from((x, y): (Projection, Projection), scale: f32) -> Self {
        Self{
            x: CacheGridAxis::from(x, scale),
            y: CacheGridAxis::from(y, scale),
        }
    }
}

pub struct CacheGrid {
    scale: f32,
    entities: HashMap<Entity, CacheGridData>,
    cells:    HashMap<CacheGridKey, HashSet<Entity>>,
    freelist: VecDeque<HashSet<Entity>>
}

impl CacheGrid {

    pub fn new(scale: f32) -> Self {
        Self{
            scale,
            entities: Default::default(),
            cells:    Default::default(),
            freelist: Default::default(),
        }
    }

    pub fn query(&mut self, x: Projection, y: Projection) -> HashSet<Entity> {
        let data = CacheGridData::from((x, y), self.scale);


        let mut result: HashSet<Entity> = Default::default();
        for x in data.x.near..data.x.far {
            for y in data.y.near..data.y.far {
                let key = CacheGridKey(x, y);
                if let Some(cell) = self.cells.get(&key) {
                    result.extend(cell);
                }
            }
        }

        result
    }

    pub fn update(&mut self, entity: Entity, shape: Shape) {
        let data_new = CacheGridData::from(shape.project_aligned(), self.scale);

        if let Some(&data_old) = self.entities.get(&entity) {
            // No change to occupancy
            if data_new == data_old { return; }
            self.remove(entity);
        }

        self.entities.insert(entity, data_new);
        for x in data_new.x.near..data_new.x.far {
            for y in data_new.y.near..data_new.y.far {
                match self.cells.entry(CacheGridKey(x, y)) {
                    Entry::Occupied(mut v) => { v.get_mut().insert(entity); },
                    Entry::Vacant(v)       => { v.insert(self.freelist.pop_back().unwrap_or_default()).insert(entity); },
                }
            }
        }

    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some(&data_old) = self.entities.get(&entity) {
            self.remove_impl(entity, data_old);
        }
    }

    fn remove_impl(&mut self, entity: Entity, data: CacheGridData) {

        self.entities.remove(&entity);
        for x in data.x.near..data.x.far {
            for y in data.y.near..data.y.far {
                let key = CacheGridKey(x, y);
                let cell = self.cells.get_mut(&key).unwrap();
                cell.remove(&entity);
                if cell.is_empty() {
                    self.freelist.push_back(self.cells.remove(&key).unwrap());
                }
            }
        }

    }
}