/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

use bevy::{
    ecs::{component::Component}
};

use crate::collision::Shape;

#[derive(Debug, Component)]
pub struct Collider {
    pub shape: Shape,
}
