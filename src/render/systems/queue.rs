use bevy::{
    render::{
        view::{ViewUniforms, ExtractedView},
        render_phase::{DrawFunctions, RenderPhase},
        renderer::RenderDevice
    },
    prelude::*,
    core_pipeline::core_3d::Opaque3d
};
use wgpu::{BindGroupDescriptor, BindGroupEntry};
use crate::render::{CustomMeta, CustomPipeline, DrawCustom};

pub fn queue_custom(
    mut commands: Commands,
    view_uniforms: Res<ViewUniforms>,
    custom_pipeline: Res<CustomPipeline>,
    draw_functions: Res<DrawFunctions<Opaque3d>>,
    mut meta: ResMut<CustomMeta>,
    render_device: Res<RenderDevice>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Opaque3d>)>,
) {
    let Some(view_binding) = view_uniforms.uniforms.binding() else { return };
    let pipeline = custom_pipeline.id;
    let draw_function = draw_functions.read().get_id::<DrawCustom>().unwrap();

    meta.view_bind_group = Some(render_device.create_bind_group(&BindGroupDescriptor {
        entries: &[BindGroupEntry {
            binding: 0,
            resource: view_binding,
        }],
        label: Some("custom_view_bind_group"),
        layout: &custom_pipeline.view_bind_group_layout,
    }));

    for i in &mut views {
        let (view, mut opaque_phase):
            (&ExtractedView, Mut<RenderPhase<Opaque3d>>) = i;
        let rangefinder = view.rangefinder3d();
        
        opaque_phase.add(Opaque3d {
            distance: rangefinder.distance(&Transform::from_xyz(0., 0., 0.).compute_matrix()),
            entity: commands.spawn().id(),
            pipeline,
            draw_function,
        });
    }
}