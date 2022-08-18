use bevy::ecs::system::lifetimeless::{Read, SQuery, SRes};
use bevy::ecs::system::SystemParamItem;
use bevy::prelude::*;
use bevy::render::render_phase::{EntityRenderCommand, PhaseItem, RenderCommand, RenderCommandResult, SetItemPipeline, TrackedRenderPass};
use bevy::render::view::ViewUniformOffset;
use crate::render::CustomMeta;

pub type DrawCustom = (
    SetItemPipeline,
    SetCustomViewBindGroup<0>,
    DrawCustomVertices,
);

pub struct SetCustomViewBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetCustomViewBindGroup<I> {
    type Param = (SRes<super::CustomMeta>, SQuery<Read<ViewUniformOffset>>);

    fn render<'w>(
        view: Entity,
        _item: Entity,
        (meta, view_query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>
    ) -> RenderCommandResult {
        let view_uniform = view_query.get(view).unwrap();
        pass.set_bind_group(
            I,
            meta.into_inner().view_bind_group.as_ref().unwrap(),
            &[view_uniform.offset]
        );
        RenderCommandResult::Success
    }
}

pub struct DrawCustomVertices;
impl<P: PhaseItem> RenderCommand<P> for DrawCustomVertices {
    type Param = SRes<CustomMeta>;

    fn render<'w>(
        _view: Entity,
        _item: &P,
        meta: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>
    ) -> RenderCommandResult {
        let meta: &CustomMeta = meta.into_inner();

        pass.set_vertex_buffer(0, meta.vertices.buffer().unwrap().slice(..));
        pass.draw(0..meta.vertices.len() as u32, 0..1);
        RenderCommandResult::Success
    }
}