/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

use bevy::{
    ecs::{
        entity::Entity,
        world::World, 
        query::{Changed, QueryState, Added, Or}
    },
    utils::HashSet,
};

use crate::{
    math::Vec2,
    collision::{Shape, Projection}
};

use super::{Collider, CacheGrid};

type QueryModifyState<'a> = QueryState<
    (Entity, &'a Collider,), 
    Or<(Added<Collider>, Changed<Collider>)>
>;

pub struct ColliderLookup<'a> {
    query_modify: QueryModifyState<'a>,
    cache_grid: CacheGrid,
}

impl<'a> ColliderLookup<'a> {
    pub fn new(world: &mut World, scale: f32) -> Self {
        Self{
            query_modify: world.query_filtered(),
            cache_grid: CacheGrid::new(scale),
        }
    }

    pub fn update(&mut self, world: &mut World) {
        for (entity, collider,) in self.query_modify.iter(world) {
            self.cache_grid.update(entity, collider.shape);
        }

        for entity in world.removed::<Collider>() {
            // TODO OPT move to Removed<Collider> when it's added
            self.cache_grid.remove(entity);
        }
    }

    pub fn query(&mut self, shape: Shape) -> HashSet<Entity> {
        let (x, y) = shape.project_aligned();
        self.cache_grid.query(x, y)
    }

    pub fn query_motion(&mut self, shape: Shape, motion: Vec2) -> HashSet<Entity> {
        let (x, y) = shape.project_aligned();
        self.cache_grid.query(
            Projection::smear(x, motion.x), 
            Projection::smear(y, motion.y)
        )
    }
}