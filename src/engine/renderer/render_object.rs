use std::{cell::RefCell, rc::Rc};

use super::{material::Material, mesh::Mesh};

pub struct RenderObject {
    pub mesh: Rc<RefCell<Mesh>>,
    pub material: Rc<RefCell<Material>>,
    pub transform: glm::Mat4,
}
