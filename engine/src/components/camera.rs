use bevy_ecs::component::Component;

#[derive(Component)]
pub struct Camera {
    projection_matrix: glm::Mat4,
}

impl Camera {
    pub fn new(aspect: f32, fovy: f32, near: f32, far: f32) -> Camera {
        Camera {
            projection_matrix: glm::perspective(aspect, fovy, near, far),
        }
    }

    pub fn matrix(&self) -> glm::Mat4 {
        self.projection_matrix
    }
}
