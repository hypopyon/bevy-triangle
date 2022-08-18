use bevy::render::render_resource::{BindGroup, BufferUsages, BufferVec};
use crate::render::CustomVertex;

pub struct CustomMeta {
    pub vertices: BufferVec<CustomVertex>,
    pub view_bind_group: Option<BindGroup>,
}
impl Default for CustomMeta {
    fn default() -> Self {
        Self {
            vertices: BufferVec::new(BufferUsages::VERTEX),
            view_bind_group: None,
        }
    }
}

