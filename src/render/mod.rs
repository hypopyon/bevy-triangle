pub mod resize_window;
pub mod sprite;
mod util;
mod camera;

use bevy::prelude::*;
use bevy::render::render_resource::Texture;
use futures_lite::future;
use wgpu::{include_wgsl, ShaderModule};

#[derive(Default, Component)]
pub struct Material {
    pub blend: Color,
    pub shader: Option<Handle<ShaderModule>>,
    pub diffuse: Option<Handle<wgpu::Texture>>
}

#[derive(Default)]
pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let (window_handle, window_size) = util::get_primary_window_info(&app.world);
        let surface = unsafe { instance.create_surface(&window_handle.get_handle()) };
        let (adapter, device, queue):
            (wgpu::Adapter, wgpu::Device, wgpu::Queue) = future::block_on(util::init_device(&instance, &surface));
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: window_size.x as u32,
            height: window_size.y as u32,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        let shader = device.create_shader_module(include_wgsl!("shaders/shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        });

        app
            .insert_resource(instance)
            .insert_resource(surface)
            .insert_resource(adapter)
            .insert_resource(device)
            .insert_resource(queue)
            .insert_resource(surface_config)
            .add_asset::<ShaderModule>()
            .add_asset::<ShaderModule>()
            .add_plugin(bevy::render::texture::ImagePlugin)
            .add_system(resize_window::resize_window);
    }
}