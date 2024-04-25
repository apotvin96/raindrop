// TODO: remove the alignment help and when doing upload,
//       have the gpu build the format that it is looking for,
//       less knowing about other places' stuff
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub color: glm::Vec3,
}
