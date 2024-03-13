use std::rc::Rc;

use super::{material::Material, mesh::Mesh};

pub struct RenderObject {
    pub mesh: Rc<Mesh>,
    pub material: Rc<Material>,

    transform: glm::Mat4,
}
