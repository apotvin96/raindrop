use bevy_ecs::component::Component;

#[derive(Component)]
pub struct Transform {
    /// 3D Translation
    translation: glm::Vec3,
    /// 3D Rotation, in radians
    rotation: glm::Vec3,
    /// 3D Scale
    scale: glm::Vec3,
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
            scale: glm::vec3(1.0, 1.0, 1.0),
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

    pub fn set_scale(&mut self, scale: glm::Vec3) {
        self.scale = scale;
        self.dirty = true;
    }

    pub fn model_matrix(&mut self) -> glm::Mat4 {
        if self.dirty {
            let identity = glm::Mat4::identity();

            let translation = glm::translate(&identity, &self.translation);
            let rotation = glm::rotate_x(&identity, self.rotation.x)
                * glm::rotate_y(&identity, self.rotation.y)
                * glm::rotate_z(&identity, self.rotation.z);
            let scale = glm::scale(&identity, &self.scale);

            self.matrix = translation * rotation * scale;

            self.dirty = false;
        }

        self.matrix
    }

    pub fn view_matrix(&mut self) -> glm::Mat4 {
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

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_new() {
        let transform = Transform::new();

        assert_eq!(transform.translation, glm::vec3(0.0, 0.0, 0.0));
        assert_eq!(transform.rotation, glm::vec3(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_translation_get() {
        let mut transform = Transform::new();

        assert_eq!(transform.get_translation(), glm::vec3(0.0, 0.0, 0.0));

        transform.translation.x = 1.0;
        transform.translation.y = 2.0;
        transform.translation.z = 3.0;

        assert_eq!(transform.get_translation(), glm::vec3(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_translation_set() {
        let mut transform = Transform::new();

        transform.set_translation(glm::vec3(1.0, 2.0, 3.0));

        assert_eq!(transform.translation, glm::vec3(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_translation_translate() {
        let mut transform = Transform::new();

        transform.translate(glm::vec3(1.0, 2.0, 3.0));
        assert_eq!(transform.translation, glm::vec3(1.0, 2.0, 3.0));

        transform.translate(glm::vec3(1.0, 2.0, 3.0));
        assert_eq!(transform.translation, glm::vec3(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_rotation_get() {
        let mut transform = Transform::new();

        assert_eq!(transform.get_rotation(), glm::vec3(0.0, 0.0, 0.0));

        transform.rotation.x = 1.0;
        transform.rotation.y = 2.0;
        transform.rotation.z = 3.0;

        assert_eq!(transform.get_rotation(), glm::vec3(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_rotation_set() {
        let mut transform = Transform::new();

        transform.set_rotation(glm::vec3(1.0, 2.0, 3.0));

        assert_eq!(transform.rotation, glm::vec3(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_rotation_rotate() {
        let mut transform = Transform::new();

        transform.rotate(glm::vec3(1.0, 2.0, 3.0));
        assert_eq!(transform.rotation, glm::vec3(1.0, 2.0, 3.0));

        transform.rotate(glm::vec3(1.0, 2.0, 3.0));
        assert_eq!(transform.rotation, glm::vec3(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_set_scale() {
        let mut transform = Transform::new();

        transform.set_scale(glm::vec3(1.0, 2.0, 3.0));

        assert_eq!(transform.scale, glm::vec3(1.0, 2.0, 3.0));
    }
}
