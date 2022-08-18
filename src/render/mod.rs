use bevy::{
    core_pipeline::core_3d::Opaque3d,
    prelude::*,
    render::{
        render_phase::AddRenderCommand,
        render_resource::{BufferUsages, BufferVec},
        RenderApp,
        renderer::{RenderDevice, RenderQueue}, RenderStage
    },
};

pub use {
    draw::*,
    meta::*,
    pipeline::*,
    systems::*,
    vertex::*
};

mod draw;
mod meta;
mod pipeline;
mod systems;
mod vertex;

#[derive(Default)]
pub struct CustomRenderPlugin;
impl Plugin for CustomRenderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.get_sub_app_mut(RenderApp).unwrap();

        let mut vertices = BufferVec::<CustomVertex>::new(BufferUsages::VERTEX);
        vertices.push(CustomVertex {
            position: [0., 1., 0.],
            uv: [0., 0.],
            color: [1., 0., 0., 1.]
        });
        vertices.push(CustomVertex {
            position: [1., -1., 0.],
            uv: [0., 0.],
            color: [0., 1., 0., 1.]
        });
        vertices.push(CustomVertex {
            position: [-1., -1., 0.],
            uv: [0., 0.],
            color: [0., 0., 1., 1.]
        });

        vertices.write_buffer(
            render_app.world.resource::<RenderDevice>(),
            render_app.world.resource::<RenderQueue>()
        );

        render_app
            .add_render_command::<Opaque3d, DrawCustom>()
            .init_resource::<CustomPipeline>()
            .insert_resource(CustomMeta {
                vertices,
                ..default()
            });

        render_app
            .add_system_to_stage(RenderStage::Queue, queue_custom);
    }
}