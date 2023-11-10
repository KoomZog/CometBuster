use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

pub struct MaterialBasicPlugin;

impl Plugin for MaterialBasicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<MaterialBasic>::default());
    }
}

impl Material2d for MaterialBasic {
    fn fragment_shader() -> ShaderRef {
        "shaders/basic_texture.wgsl".into()
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "badbcd09-76fc-45a7-86da-b9e227c5634b"]
pub struct MaterialBasic {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Option<Handle<Image>>,
}