use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait HeightData { fn height(&self) -> u8; }
pub trait ShapeData { fn shape(&self) -> u8; }