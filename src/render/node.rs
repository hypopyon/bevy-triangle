use bevy::{
    prelude::{World},
    render::{
        render_graph::{Node, NodeRunError, RenderGraphContext},
        render_resource::PipelineCache,
        renderer::RenderContext,
        view::ExtractedWindows
    },
    window::WindowId
};
use super::pipeline::CustomPipeline;

pub struct CustomRenderNode {}

impl Node for CustomRenderNode {
    fn update(&mut self, _world: &mut World) {}

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World
    ) -> Result<(), NodeRunError> {
        let extracted_windows = &world.resource::<ExtractedWindows>();
        let Some(primary_window) = extracted_windows.get(&WindowId::primary())
            else { return Ok(()) };
        let Some(swap_chain_texture) = &primary_window.swap_chain_texture else { return Ok(()) };
        {
            let mut pass = render_context.command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &swap_chain_texture,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None
            });
            if let Some(pipeline) = world
                .resource::<PipelineCache>()
                .get_render_pipeline(world.resource::<CustomPipeline>().id) {
                pass.set_pipeline(pipeline);
                pass.draw(0..3, 0..1);
            }
        }
        Ok(())
    }
}
