use crate::*;

mod generics;
pub use generics::*;

mod bitmask;
pub use bitmask::*;
mod image;
pub use self::image::*;
mod math;
pub use math::*;
mod mesh;
pub use mesh::*;
mod noise;
pub use self::noise::*;
mod serial;
pub use serial::*;
mod thread;
pub use thread::*;

mod bundle;
pub use bundle::*;

pub struct TankUtilPlugin;
impl Plugin for TankUtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TankUtilGenericsPlugin,
        ));
    }
}