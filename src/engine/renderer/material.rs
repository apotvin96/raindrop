use std::rc::Rc;

use super::primitives::Pipeline;

pub struct Material {
    pub pipeline: Rc<Pipeline>,
}
