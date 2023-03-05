use serde::Serialize;
use std::fmt::Debug;

pub trait CombinatorialClass: Debug + Clone + PartialEq + Serialize {}
