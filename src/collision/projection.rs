/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

use super::{Overlap, OverlapCase};

/// Represents the minimum and maximum points of a
/// shape projected onto a particular axis. This is
/// useful for SAT-based collision detection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Projection(f32, f32);

impl Projection {
    /// Construct a new projection, without checking the argument
    /// order - in release builds.
    /// 
    /// # Arguments
    /// 
    /// * `near` The dot product of a point along a normal, must be <= `far`
    /// * `far` The dot product of a point along a normal, must be >= `near`
    pub fn new_unchecked(near: f32, far: f32) -> Self {
        debug_assert!(near <= far, "Attempt to construct backwards projection");
        Self(near, far)
    }

    /// Construct a new projection, assigning the near/far values
    /// correctly regardless of order. Slow.
    /// 
    /// # Arguments
    /// 
    /// * `a` The dot product of a point along a normal
    /// * `b` The dot product of a point along a normal
    pub fn new(a: f32, b: f32) -> Self {
        if a <= b {
            Self::new_unchecked(a, b)
        } else {
            Self::new_unchecked(b, a)
        }
    }

    /// Attempts to create a new projection, returning `None` if `far` < `near`
    /// 
    /// # Arguments
    /// 
    /// * `near` The dot product of a point along a normal, returns None if > `far`
    /// * `far` The dot product of a point along a normal, returns None if < `near`
    pub fn try_new(near: f32, far: f32) -> Option<Self> {
        if near > far {
            None
        } else {
            Some(Self::new_unchecked(near, far))
        }
    }
}

impl Projection {
    /// Construct a new projection, from a projected origin point, assigning it
    /// to both `near` and `far`.
    /// 
    /// # Arguments
    /// 
    /// * `origin` The dot product of a point along a normal
    pub fn point(origin: f32) -> Self {
        Self::new_unchecked(origin, origin)
    }

    /// Construct a new projection, from a projected origin point and an offset
    /// applied to get `far` value. 
    /// 
    /// # Arguments
    /// 
    /// * `origin` The dot product of a point along a normal
    /// * `dist` The distance to offset the max from the origin
    pub fn extent_from(origin: f32, dist: f32) -> Self {
        Self::new_unchecked(origin, origin + dist)
    }

    /// Construct a new projection, from a projected origin point and an offset
    /// applied symmetrically to get the near and far values. Useful for circles.
    /// 
    /// # Arguments
    /// 
    /// * `origin` The dot product of a point along a normal
    /// * `dist` The distance to offset the `near` and `far` from the origin symmetrically
    pub fn symmetrical(origin: f32, dist: f32) -> Self {
        Self::new_unchecked(origin - dist, origin + dist)
    }

    /// Creates a projection that covers both of the projections given
    pub fn covering_both(a: Self, b: Self) -> Self {
        Self::new_unchecked(a.0.min(b.0), b.0.max(b.1))
    }

    /// Creates a projection that covers both the given projection and point
    pub fn covering_point(a: Self, b: f32) -> Self {
        Self::new_unchecked(a.0.min(b), a.1.max(b))
    }

    /// Smears a projection by offsetting near/far based on the sign of b.
    /// This is only useful for specific situations and therefore isn't suitable
    /// for sweeps.
    pub fn smear(a: Self, b: f32) -> Self {
        if b < 0.0 {
            Self::new_unchecked(a.0 + b, a.1)
        } else {
            Self::new_unchecked(a.0, a.1 + b)
        }
    }
}

impl Projection {
    /// Gets the minimum value of the projection
    pub fn near(self) -> f32 {
        self.0
    }

    /// Gets the maximum value of the projection
    pub fn far(self) -> f32 {
        self.1
    }

    /// Gets the length of the projection
    pub fn length(self) -> f32 {
        self.1 - self.0
    }

    /// The midpoint of the projection
    pub fn mid(self) -> f32 {
        self.length()/2.0 + self.near()
    }

    /// Converts to a regular tuple (near, far)
    pub fn as_tuple(self) -> (f32, f32) {
        (self.0, self.1)
    }
}

impl Projection {
    /// Checks if a projection overlaps another one, including exact contact.
    /// 
    /// # Arguments
    /// 
    /// * `other` The projection to check for an overlap with
    pub fn are_separate(self, other: Self) -> bool {
        (self.1 < other.0) || (self.0 > other.1)
    }
}

impl Projection {
    /// Attempts to calculate both of the signed penetrations between two shapes, 
    /// returning overlaps with the correct sign to put the two objects in contact
    /// when `self` is moved along the axis by the length of the overlap.
    /// 
    /// Usually the minimum penetration vector is desired, in that case see
    /// [Self::get_penetration].
    /// 
    /// # Arguments
    /// 
    /// * `other` The projection to calculate penetrations with
    pub fn get_penetration_all(self, other: Self) -> Option<(Overlap, Overlap)> {
        match (self.are_separate(other), self.0 <= other.0) {
            (true,     _) => None,
            (false, true) => Some((
                Overlap(Self::new_unchecked( self.0, other.1), OverlapCase::Positive),
                Overlap(Self::new_unchecked(other.0,  self.1), OverlapCase::Negative),
            )),
            (false, false) => Some((
                Overlap(Self::new_unchecked(other.0,  self.1), OverlapCase::Negative),
                Overlap(Self::new_unchecked( self.0, other.1), OverlapCase::Positive),
            )),
        }
    }

    /// Attempts to calculate the minimum signed penetration between two shapes, 
    /// returning an overlap with the correct sign to put the two objects in 
    /// contact when `self` is moved along the axis by the length of the overlap.
    /// 
    /// There's always more than one way to resolve a penetration,
    /// see [Self::get_penetration_all] for custom resolution.
    /// 
    /// # Arguments
    /// 
    /// * `other` The projection to calculate the penetration into
    pub fn get_penetration(self, other: Self) -> Option<Overlap> {
        self.get_penetration_all(other).map(|(near, far)| {
            if near.projection() == far.projection() {
                near.with_case(OverlapCase::Unsigned)
            } else if near.length() <= far.length() { 
                near
            } else { 
                far 
            }
        })
    }
}

impl Projection {
    /// Attempts to calculate the signed gap between two shapes, returning an overlap
    /// with the correct sign to put the two objects in contact when `self` is moved
    /// along the axis by the length of the overlap.
    /// 
    /// # Arguments
    /// 
    /// * `other` The projection to calculate the separation from
    pub fn get_separation(self, other: Self) -> Option<Overlap> {
        if self.1 < other.0 {
            Some(Overlap(Self::new_unchecked( self.1, other.0), OverlapCase::Positive)) 
        } else if self.0 > other.1 {
            Some(Overlap(Self::new_unchecked(other.1,  self.0), OverlapCase::Negative))
        } else {
            None
        }
    }
}
