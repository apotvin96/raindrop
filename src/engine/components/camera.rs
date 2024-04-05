use bevy_ecs::component::Component;

#[derive(Component)]
pub struct Camera {
    /**
        Camera Position
    */
    translation: glm::Vec3,
    /**
        Camera rotation in radians, z is not used
    */
    rotation: glm::Vec3,
    /**
       The calculated view matrix of the camera
    */
    view_matrix: glm::Mat4,
    /**
       If the camera has been moved or rotated, and the view matrix needs to be recalculated
    */
    dirty: bool,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            translation: glm::Vec3::zeros(),
            rotation: glm::Vec3::zeros(),
            view_matrix: glm::Mat4::identity(),
            dirty: true,
        }
    }

    pub fn view_matrix(&mut self) -> glm::Mat4 {
        if self.dirty {
            let translation = self.translation;
            let rotation = self.rotation;

            let center = translation
                + glm::vec3(
                    rotation.x.cos() * rotation.y.cos(),
                    rotation.y.sin(),
                    rotation.x.sin() * rotation.y.cos(),
                );
            let up = glm::vec3(0.0, 1.0, 0.0);

            self.view_matrix = glm::look_at(&translation, &center, &up);

            self.dirty = false;
        }

        self.view_matrix
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
}
