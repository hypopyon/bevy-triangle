use bevy::prelude::*;
use bevy::render::{
    render_resource::{
        CachedRenderPipelineId, FragmentState, PipelineCache, RenderPipelineDescriptor, VertexState
    },
    texture::BevyDefault
};
use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Face, FrontFace, MultisampleState, PolygonMode,
    PrimitiveState, PrimitiveTopology, TextureFormat
};

pub struct CustomPipeline {
    pub id: CachedRenderPipelineId,
}
impl FromWorld for CustomPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let shader: Handle<Shader> = asset_server.load("shader.wgsl");
        let mut cache = world.resource_mut::<PipelineCache>();
        let pipeline_id = cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: VertexState {
                shader: shader.clone(),
                shader_defs: vec![],
                entry_point: "vs_main".into(),
                buffers: vec![]
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            fragment: Some(FragmentState {
                shader: shader.clone(),
                shader_defs: vec![],
                entry_point: "fs_main".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL
                })]
            })
        });

        CustomPipeline {
            id: pipeline_id,
        }
    }
}