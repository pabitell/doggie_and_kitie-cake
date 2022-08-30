use pabitell_lib::{cli::cmdline, WorldBuilder};

pub use common::items;
pub use common::narrator;
pub use common::world;

pub fn main() {
    let world = world::CakeWorldBuilder::make_world().unwrap();
    let narrator = narrator::Cake;
    cmdline::run("doggie_and_kitie-cake", world, narrator);
}
