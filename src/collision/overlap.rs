/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

use super::Projection;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Overlap(pub Projection, pub OverlapCase);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OverlapCase {
    Positive,
    Negative,
    Unsigned,
}

pub enum OverlapOffset {
    Known(f32),
    Unsure(f32),
}

impl OverlapOffset {

    pub fn unwrap_or(self, value: f32) -> f32 {
        match self {
            Self::Known(v)  => v,
            Self::Unsure(_) => value,
        }
    }

    pub fn unwrap_or_else<F: FnOnce(f32) -> f32>(self, func: F) -> f32 {
        match self {
            Self::Known(v)  => v,
            Self::Unsure(v) => func(v),
        }
    }

    pub fn unwrap_or_sign(self, sign: f32) -> f32 {
        match self {
            Self::Known(v)  => v,
            Self::Unsure(v) => v*sign,
        }
    }

}

impl Overlap {
    pub fn negate(self) -> Self {
        match self.1 {
            OverlapCase::Positive => Self(self.0, OverlapCase::Negative),
            OverlapCase::Negative => Self(self.0, OverlapCase::Positive),
            OverlapCase::Unsigned => Self(self.0, OverlapCase::Unsigned),
        }
    }

    pub fn with_case(self, case: OverlapCase) -> Self {
        Self(self.0, case)
    }
}

impl Overlap {

    pub fn length(self) -> f32 {
        self.0.length()
    }

}

impl Overlap {

    pub fn offset(self) -> OverlapOffset {
        match self.1 {
            OverlapCase::Positive =>  OverlapOffset::Known( self.0.length()),
            OverlapCase::Negative =>  OverlapOffset::Known(-self.0.length()),
            OverlapCase::Unsigned => OverlapOffset::Unsure( self.0.length()),
        }
    }

}

impl Overlap {
    pub fn projection(&self) -> &Projection {
        &self.0
    }

    pub fn case(&self) -> &OverlapCase {
        &self.1
    }
}


impl Overlap {
    pub fn into_tuple(self) -> (Projection, OverlapCase) {
        (self.0, self.1)
    }
}