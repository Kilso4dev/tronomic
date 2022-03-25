use crate::{color::Rgba, error::DmGuiError};
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq)]
pub enum GType {
    FNum(f64, f64),
    INum(i64, i64),
    Color,
    FVec(f64, f64),
    IVec(i64, i64),
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

pub mod graph_values {
    use super::*;

    pub fn gval_as_fnum(
        v: GVal,
        er: Option<RangeInclusive<f64>>,
    ) -> Result<f64, DmGuiError> {
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
    pub fn gval_as_inum(
        v: GVal,
        er: Option<RangeInclusive<i64>>,
    ) -> Result<i64, DmGuiError> {
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

    pub fn gval_as_fvec(
        v: GVal,
        er: Option<RangeInclusive<f64>>,
    ) -> Result<Vec<f64>, DmGuiError> {
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

    pub fn gval_as_ivec(
        v: GVal,
        er: Option<RangeInclusive<i64>>,
    ) -> Result<Vec<i64>, DmGuiError> {
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

