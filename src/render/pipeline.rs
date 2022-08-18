use bevy::prelude::*;
use bevy::render::{
    render_resource::*,
    renderer::RenderDevice,
    texture::BevyDefault,
    view::ViewUniform
};
use wgpu::{BindGroupLayoutDescriptor, BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, FrontFace, MultisampleState, PolygonMode, PrimitiveState, PrimitiveTopology, StencilFaceState, StencilState, TextureFormat};

pub struct CustomPipeline {
    pub id: CachedRenderPipelineId,
    pub view_bind_group_layout: BindGroupLayout,
}
impl FromWorld for CustomPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let shader: Handle<Shader> = asset_server.load("shader.wgsl");
        let render_device = world.resource::<RenderDevice>();
        let view_bind_group_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: Some(ViewUniform::min_size()),
                },
                count: None,
            }],
            label: Some("sprite_view_layout"),
        });

        let formats = vec![
            VertexFormat::Float32x3,
            VertexFormat::Float32x2,
            VertexFormat::Float32x4,
        ];
        let vertex_layout =
            VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);

        let mut cache = world.resource_mut::<PipelineCache>();
        let pipeline_id = cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: None,
            layout: Some(vec![view_bind_group_layout.clone()]),
            vertex: VertexState {
                shader: shader.clone(),
                shader_defs: vec![],
                entry_point: "vs_main".into(),
                buffers: vec![vertex_layout]
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Cw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: false,
                depth_compare: CompareFunction::Greater,
                stencil: StencilState {
                    front: StencilFaceState::IGNORE,
                    back: StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0
                },
                bias: DepthBiasState {
                    ..default()
                }
            }),
            multisample: MultisampleState {
                count: 4,
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
            view_bind_group_layout,
        }
    }
}