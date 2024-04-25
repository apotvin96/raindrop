extern crate log;
extern crate nalgebra_glm as glm;

#[macro_use]
extern crate lazy_static;

mod boilerplate;
mod debug;
mod material;
mod mesh;
mod primitives;
pub mod renderable;
pub mod renderer;

use boilerplate::Boilerplate;
use material::Material;
pub use renderable::Renderable;
pub use renderer::Renderer;