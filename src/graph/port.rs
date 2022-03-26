use crate::{color::Rgba, error::DmGuiError};
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub enum GType {
    FNum(f64, f64),
    INum(i64, i64),
    Color,
    FVec(f64, f64),
    IVec(i64, i64),
}

/// This is for being able to connect different types into the same connections, to compare the two
/// directly: use [`is_equal_to`] method
impl PartialEq for GType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (_, _) => true,
        }
    }
}

impl GType {
    pub fn is_equal_to(&self, other: &Self) -> bool {
        use GType::*;
        match (self, other) {
            (FNum(smin, smax), FNum(omin, omax)) => smin == omin && smax == omax,
            (INum(smin, smax), INum(omin, omax)) => smin == omin && smax == omax,
            (Color, Color) => true,
            (FVec(smin, smax), FVec(omin, omax)) => smin == omin && smax == omax,
            (IVec(smin, smax), IVec(omin, omax)) => smin == omin && smax == omax,
            (_, _) => false,
        }
    }
}

impl Eq for GType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Port {
    pub name: &'static str,
    pub gval: GVal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GVal {
    FNum(f64, RangeInclusive<f64>),
    INum(i64, RangeInclusive<i64>),
    Color(Rgba),
    FVec(Vec<f64>, RangeInclusive<f64>),
    IVec(Vec<i64>, RangeInclusive<i64>),
}

impl From<&GType> for GVal {
    fn from(pt: &GType) -> Self {
        match pt {
            GType::FNum(rmin, rmax) => Self::FNum(Default::default(), *rmin..=*rmax),
            GType::INum(rmin, rmax) => Self::INum(Default::default(), *rmin..=*rmax),
            GType::Color => Self::Color(Default::default()),
            GType::FVec(rmin, rmax) => Self::FVec(Default::default(), *rmin..=*rmax),
            GType::IVec(rmin, rmax) => Self::IVec(Default::default(), *rmin..=*rmax),
        }
    }
}

impl From<&GVal> for GType {
    fn from(p: &GVal) -> Self {
        match p {
            GVal::FNum(_, range) => Self::FNum(*range.start(), *range.end()),
            GVal::INum(_, range) => Self::INum(*range.start(), *range.end()),
            GVal::Color(_) => Self::Color,
            GVal::FVec(_, range) => Self::FVec(*range.start(), *range.end()),
            GVal::IVec(_, range) => Self::IVec(*range.start(), *range.end()),
        }
    }
}

impl GVal {
    pub fn as_color(self) -> Result<Rgba, DmGuiError> {
        fn to_u8(n: i64) -> u8 {
            n.clamp(u8::MIN.into(), u8::MAX.into()) as u8
        }
        match self {
            Self::FNum(n, r) => Ok(n.into()),
            Self::INum(n, r) => Ok(to_u8(n).into()),
            Self::Color(c) => Ok(c),
            Self::FVec(v, r) => Ok([
                if let Some(f) = v.get(0) { *f } else { 0. },
                if let Some(f) = v.get(1) { *f } else { 0. },
                if let Some(f) = v.get(2) { *f } else { 0. },
                if let Some(f) = v.get(3) { *f } else { 1. },
            ]
            .into()),
            Self::IVec(v, r) => Ok([
                if let Some(n) = v.get(0) { to_u8(*n) } else { 0 },
                if let Some(n) = v.get(1) { to_u8(*n) } else { 0 },
                if let Some(n) = v.get(2) { to_u8(*n) } else { 0 },
                if let Some(n) = v.get(3) {
                    to_u8(*n)
                } else {
                    u8::MAX
                },
            ]
            .into()),
        }
    }

    pub fn as_fnum(self) -> Result<f64, DmGuiError> {
        match self {
            Self::FNum(n, r) => Ok(n),
            Self::INum(n, r) => Ok(n as f64),
            Self::Color(c) => Err(DmGuiError::evaluation("Color is not convertable to a Num")),
            Self::FVec(v, r) => Ok(v[0]),
            Self::IVec(v, r) => Ok(v[0] as f64),
        }
    }

    pub fn as_inum(self) -> Result<i64, DmGuiError> {
        match self {
            Self::FNum(n, r) => Ok(n as i64),
            Self::INum(n, r) => Ok(n),
            Self::Color(c) => Err(DmGuiError::evaluation("Color is not convertable to a Num")),
            Self::FVec(v, r) => Ok(v[0] as i64),
            Self::IVec(v, r) => Ok(v[0]),
        }
    }
    pub fn as_fvec(self) -> Result<Vec<f64>, DmGuiError> {
        match self {
            Self::FNum(n, r) => Ok(vec![n]),
            Self::INum(n, r) => Ok(vec![n as f64]),
            Self::Color(c) => Ok(c.0.into_iter().collect()),
            Self::FVec(v, r) => Ok(v),
            Self::IVec(v, r) => Ok(v.into_iter().map(|c| c as f64).collect()),
        }
    }
    pub fn as_ivec(self) -> Result<Vec<i64>, DmGuiError> {
        match self {
            Self::FNum(n, r) => Ok(vec![n as i64]),
            Self::INum(n, r) => Ok(vec![n]),
            Self::Color(c) => Ok(c
                .0
                .into_iter()
                .map(|c| ((c * 255.) as i64).clamp(u8::MIN as i64, u8::MAX as i64))
                .collect()),
            Self::FVec(v, r) => Ok(v.into_iter().map(|c| c as i64).collect()),
            Self::IVec(v, r) => Ok(v),
        }
    }
}

pub mod graph_values {
    use super::*;

    pub fn gval_as_fnum(v: GVal, er: Option<RangeInclusive<f64>>) -> Result<f64, DmGuiError> {
        match v {
            GVal::FNum(v, r) => match er {
                Some(er) => {
                    if r.start() == er.start() && r.end() == er.end() {
                        Ok(v)
                    } else {
                        Err(DmGuiError::evaluation(
                            "range of values is not correct (given: {r} expected: {er})",
                        ))
                    }
                }
                None => Ok(v),
            },
            o => Err(DmGuiError::evaluation(format!(
                "input value is not correct (given: {o:?})"
            ))),
        }
    }
    pub fn gval_as_inum(v: GVal, er: Option<RangeInclusive<i64>>) -> Result<i64, DmGuiError> {
        match v {
            GVal::INum(v, r) => match er {
                Some(er) => {
                    if r.start() == er.start() && r.end() == er.end() {
                        Ok(v)
                    } else {
                        Err(DmGuiError::evaluation(
                            "range of values is not correct (given: {r} expected: {er})",
                        ))
                    }
                }
                None => Ok(v),
            },
            o => Err(DmGuiError::evaluation(format!(
                "input value is not correct (given: {o:?})"
            ))),
        }
    }

    pub fn gval_as_fvec(v: GVal, er: Option<RangeInclusive<f64>>) -> Result<Vec<f64>, DmGuiError> {
        match v {
            GVal::FVec(v, r) => match er {
                Some(er) => {
                    if r.start() == er.start() && r.end() == er.end() {
                        Ok(v)
                    } else {
                        Err(DmGuiError::evaluation(
                            "range of values is not correct (given: {r} expected: {er})",
                        ))
                    }
                }
                None => Ok(v),
            },
            o => Err(DmGuiError::evaluation(format!(
                "input value is not correct (given: {o:?})"
            ))),
        }
    }

    pub fn gval_as_ivec(v: GVal, er: Option<RangeInclusive<i64>>) -> Result<Vec<i64>, DmGuiError> {
        match v {
            GVal::IVec(v, r) => match er {
                Some(er) => {
                    if r.start() == er.start() && r.end() == er.end() {
                        Ok(v)
                    } else {
                        Err(DmGuiError::evaluation(
                            "range of values is not correct (given: {r} expected: {er})",
                        ))
                    }
                }
                None => Ok(v),
            },
            o => Err(DmGuiError::evaluation(format!(
                "input value is not correct (given: {o:?})"
            ))),
        }
    }

    pub fn gval_as_col(v: GVal) -> Result<Rgba, DmGuiError> {
        match v {
            GVal::Color(c) => Ok(c),
            o => Err(DmGuiError::evaluation(format!(
                "input value is not correct (given: {o:?})"
            ))),
        }
    }
}
