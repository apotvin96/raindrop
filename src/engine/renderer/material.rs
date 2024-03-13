use std::rc::Rc;

use super::primitives::Pipeline;

pub struct Material {
    pipeline: Rc<Pipeline>,
}
