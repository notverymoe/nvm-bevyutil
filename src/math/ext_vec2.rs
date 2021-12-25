/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

pub use crate::math::prelude::Vec2;

pub trait ExtVec2 {
    fn angle(self) -> f32;
    fn from_angle(angle: f32) -> Self;

    fn rotated_by(self, angle: f32) -> Self;
    fn rotated_by_vec(self, rot_vec: Vec2) -> Self;

    fn negate_x(self) -> Self;
    fn negate_y(self) -> Self;

    fn into_tuple(self) -> (f32, f32);
}


impl ExtVec2 for Vec2 {
    fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    fn from_angle(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Self::new(c, s)
    }

    fn rotated_by(self, angle: f32) -> Self {
        self.rotated_by_vec(Self::from_angle(angle))
    }

    fn rotated_by_vec(self, rot_vec: Vec2) -> Self {
        Self::new(
            self.x*rot_vec.x - self.y*rot_vec.y,
            self.x*rot_vec.y + self.y*rot_vec.x,
        )
    }

    fn negate_x(self) -> Self {
        Self::new(-self.x,  self.y)
    }

    fn negate_y(self) -> Self {
        Self::new( self.x, -self.y)
    }

    fn into_tuple(self) -> (f32, f32) {
        (self.x, self.y)
    }
}