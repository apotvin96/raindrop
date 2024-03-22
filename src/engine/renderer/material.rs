use std::{cell::RefCell, rc::Rc};

use super::primitives::Pipeline;

pub struct Material {
    pub pipeline: Rc<RefCell<Pipeline>>,
}

impl Drop for Material {
    fn drop(&mut self) {
        // We don't need to do anything here because the pipeline will be dropped when the Material is dropped
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.pipeline, &other.pipeline)
    }
}
