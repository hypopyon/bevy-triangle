use bevy::{
    render::{
        render_graph::RenderGraph,
        RenderApp,
    },
    prelude::*
};

mod pipeline;
mod node;

#[derive(Default)]
pub struct CustomRenderPlugin;
impl Plugin for CustomRenderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.get_sub_app_mut(RenderApp).unwrap();

        let mut graph = render_app.world.resource_mut::<RenderGraph>();
        graph.add_node("custom_pass", node::CustomRenderNode {});
        graph.add_node_edge(bevy::render::main_graph::node::CAMERA_DRIVER, "custom_pass").unwrap();

        render_app
            .init_resource::<pipeline::CustomPipeline>();
    }
}