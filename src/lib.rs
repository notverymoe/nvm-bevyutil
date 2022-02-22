/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

pub mod collision;
pub mod math;
pub mod physics;
pub mod compact_str;
pub mod resource;
pub mod sync;
pub mod util;
pub mod container;

#[cfg(feature = "bevy/render")]
pub mod camera;

pub mod prelude {
    pub use crate::math::prelude::*;
    pub use crate::util::*;
}