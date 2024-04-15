use bevy_ecs::component::Component;

#[derive(Component)]
pub struct Transform {
    /// 3D Translation
    translation: glm::Vec3,
    /// 3D Rotation, in radians
    rotation: glm::Vec3,
    /// Combined matrix of translation and rotation
    matrix: glm::Mat4,
    /// Whether the matrix needs to be recalculated
    dirty: bool,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            translation: glm::Vec3::zeros(),
            rotation: glm::Vec3::zeros(),
            matrix: glm::Mat4::identity(),
            dirty: true,
        }
    }

    pub fn get_translation(&self) -> glm::Vec3 {
        self.translation
    }

    pub fn set_translation(&mut self, translation: glm::Vec3) {
        self.translation = translation;
        self.dirty = true;
    }

    pub fn translate(&mut self, translation: glm::Vec3) {
        self.translation += translation;
        self.dirty = true;
    }

    pub fn get_rotation(&self) -> glm::Vec3 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: glm::Vec3) {
        self.rotation = rotation;
        self.dirty = true;
    }

    pub fn rotate(&mut self, rotation: glm::Vec3) {
        self.rotation += rotation;
        self.dirty = true;
    }

    pub fn matrix(&mut self) -> glm::Mat4 {
        if self.dirty {
            let translation = self.translation;
            let rotation = self.rotation;

            let center = translation
                + glm::normalize(&glm::vec3(
                    rotation.x.cos() * rotation.y.cos(),
                    rotation.x.sin(),
                    rotation.x.cos() * rotation.y.sin(),
                ));
            let up = glm::vec3(0.0, 1.0, 0.0);

            self.matrix = glm::look_at(&translation, &center, &up);

            self.dirty = false;
        }

        self.matrix
    }
}
