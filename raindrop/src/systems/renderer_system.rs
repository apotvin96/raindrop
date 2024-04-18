use crate::{
    components::{Camera, Material, Mesh, Player, Transform},
    resources::RendererResource,
};
use bevy_ecs::{
    query::{With, Without},
    system::{NonSendMut, Query},
};

use renderer::Renderable;

pub fn renderer_system(
    mut player_camera: Query<(&mut Camera, &mut Transform), With<Player>>,
    mut renderable_objects: Query<(&mut Transform, &Mesh, &Material), Without<Player>>,
    mut renderer: NonSendMut<RendererResource>,
) {
    let (camera, mut transform) = player_camera.iter_mut().next().unwrap();

    let view_matrix = transform.view_matrix();
    let projection_matrix = camera.matrix();

    let mut renderables: Vec<Renderable> = vec![];
    for (mut transform, mesh, material) in renderable_objects.iter_mut() {
        renderables.push(Renderable {
            mesh: mesh.id.clone(),
            material: material.id.clone(),
            matrix: transform.model_matrix(),
        });
    }

    renderables
        .sort_unstable_by_key(|renderable| (renderable.mesh.clone(), renderable.material.clone()));

    renderer
        .as_mut()
        .renderer
        .render(projection_matrix, view_matrix, &renderables);
}
