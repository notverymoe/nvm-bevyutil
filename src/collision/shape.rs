/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

use arrayvec::ArrayVec;

use crate::prelude::*;
use super::Projection;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ShapeKind {
    Ellipse,
    Rectangle,

    TriangleDeclineNormal,
    TriangleDeclineInvert,
    TriangleInclineNormal,
    TriangleInclineInvert,
}

#[derive(Debug, Clone, Copy)]
pub struct Shape {
    pub origin: Vec2,
    pub hbound: Vec2,
    pub shape:  ShapeKind,
}

impl Shape {
    pub fn point(origin: Vec2) -> Self {
        Self{
            origin,
            hbound: Vec2::ZERO,
            shape: ShapeKind::Ellipse
        }
    }

    pub fn circle(origin: Vec2, radius: f32) -> Self {
        Self{
            origin,
            hbound: Vec2::ONE*radius,
            shape: ShapeKind::Ellipse
        }
    }

    pub fn ellipse(origin: Vec2, radius: Vec2) -> Self {
        Self{
            origin,
            hbound: radius,
            shape: ShapeKind::Ellipse
        }
    }

    pub fn square(origin: Vec2, size: f32) -> Self {
        Self{
            origin,
            hbound: Vec2::ONE*size,
            shape: ShapeKind::Rectangle
        }
    }

    pub fn rectangle(origin: Vec2, size: Vec2) -> Self {
        Self{
            origin,
            hbound: size,
            shape: ShapeKind::Rectangle
        }
    }

    pub fn triangle(origin: Vec2, size: Vec2, incline: bool, invert: bool) -> Self {
        Self{
            origin,
            hbound: size,
            shape: match (incline, invert) {
                (false, false) => ShapeKind::TriangleDeclineNormal,
                (false,  true) => ShapeKind::TriangleDeclineInvert,
                ( true, false) => ShapeKind::TriangleInclineNormal,
                ( true,  true) => ShapeKind::TriangleInclineInvert,
            }
        }
    }
}

impl Shape {
    pub fn project_aligned(&self) -> (Projection, Projection) {
        (
            Projection::symmetrical(self.origin.x, self.hbound.x),
            Projection::symmetrical(self.origin.y, self.hbound.y),
        )
    }

    pub fn project_on(&self, axis: Vec2) -> Projection {
        match self.shape {
            ShapeKind::Ellipse   => Projection::symmetrical(axis.dot(self.origin), self.get_radius_on_axis(axis)),
            ShapeKind::Rectangle => project_points(axis, self.get_points_bound()),
            ShapeKind::TriangleDeclineNormal | ShapeKind::TriangleDeclineInvert | ShapeKind::TriangleInclineNormal | ShapeKind::TriangleInclineInvert => {
                project_points(axis, self.get_points_slope())
            },
        }
    }

    pub fn axes_between(&self, other: &Shape) -> ArrayVec<Vec2, 4> {
        use std::f32::consts::FRAC_1_SQRT_2;

        let mut result = ArrayVec::<Vec2, 4>::new_const();
        match self.shape {
            ShapeKind::Rectangle => {},
            ShapeKind::TriangleDeclineNormal => result.push(Vec2::new( FRAC_1_SQRT_2,  FRAC_1_SQRT_2)),
            ShapeKind::TriangleInclineNormal => result.push(Vec2::new(-FRAC_1_SQRT_2,  FRAC_1_SQRT_2)),
            ShapeKind::TriangleDeclineInvert => result.push(Vec2::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2)),
            ShapeKind::TriangleInclineInvert => result.push(Vec2::new( FRAC_1_SQRT_2, -FRAC_1_SQRT_2)),
            ShapeKind::Ellipse => match other.shape {
                ShapeKind::Ellipse => {
                    if let Some(v) = (other.origin - self.origin).try_normalize() { result.push(v); }
                },
                ShapeKind::Rectangle => {
                    other.get_points_bound().iter().for_each(|p| if let Some(v) = (*p - self.origin).try_normalize() { result.push(v) })
                },
                ShapeKind::TriangleDeclineNormal | ShapeKind::TriangleDeclineInvert | ShapeKind::TriangleInclineNormal | ShapeKind::TriangleInclineInvert => {
                    other.get_points_slope().iter().for_each(|p| if let Some(v) = (*p - self.origin).try_normalize() { result.push(v) });
                },
            },
        }
        result
    }
}

impl Shape {

    pub fn get_radius_on_axis(&self, axis: Vec2) -> f32 {
        assert_eq!(self.shape, ShapeKind::Ellipse);
        if self.hbound.x == self.hbound.y {
            self.hbound.x
        } else {
            debug_assert!(axis.is_normalized());
            let (c, s) = axis.into_tuple();
            let (x, y) = self.hbound.into_tuple();
            (x*y)/(x*x*s*s + y*y*c*c).sqrt()
        }
    }

    pub fn get_points_bound(&self) -> [Vec2; 4] {
        [
            self.origin - self.hbound,
            self.origin + self.hbound.negate_y(),
            self.origin + self.hbound,
            self.origin + self.hbound.negate_x(),
        ]
    }

    pub fn get_points_ellipse(&self, segments: usize) -> Vec<Vec2> {
        assert_eq!(self.shape, ShapeKind::Ellipse);
        let mut result = Vec::<Vec2>::with_capacity(segments);
        let step = Vec2::from_angle(std::f32::consts::TAU/(segments as f32));
        let mut axis = Vec2::X;
        for _ in 0..segments {
            let dist = self.project_on(axis).far() - axis.dot(self.origin);
            result.push(self.origin + dist*axis);
            axis = axis.rotated_by_vec(step);
        }
        result
    }

    pub fn get_points_slope(&self) -> [Vec2; 3] {
        match self.shape {
            ShapeKind::TriangleDeclineNormal => [
                self.origin - self.hbound,
                self.origin + self.hbound.negate_y(),
                self.origin + self.hbound.negate_x(),
            ],
            ShapeKind::TriangleDeclineInvert => [
                self.origin + self.hbound,
                self.origin + self.hbound.negate_x(),
                self.origin + self.hbound.negate_y(),
            ],
            ShapeKind::TriangleInclineNormal => [
                self.origin + self.hbound.negate_y(),
                self.origin + self.hbound,
                self.origin - self.hbound,
            ],
            ShapeKind::TriangleInclineInvert => [
                self.origin + self.hbound.negate_x(),
                self.origin - self.hbound,
                self.origin + self.hbound,
            ],
            _ => panic!("Cannot get triangle points for non-triangle"),
        }
    }

}

fn project_points<const N: usize>(axis: Vec2, points: [Vec2; N]) -> Projection {
    debug_assert!(!points.is_empty(), "Points must contain at least one element");
    points.iter().skip(1).fold(
        Projection::point(axis.dot(points[0])), 
        |p, v| Projection::covering_point(p, axis.dot(*v))
    )
}