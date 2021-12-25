/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

mod ext_vec2;
pub use ext_vec2::*;

pub mod prelude {
    // We re-export bevy math so we can port to 
    // a different math library more easily
    pub use bevy::math::*;

    pub use super::ext_vec2::*;
}