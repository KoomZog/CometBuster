use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
        render_resource::{
            std140::{AsStd140, Std140},
            *,
        },
        renderer::RenderDevice,
    },
    sprite::{Material2dPipeline, Material2dPlugin, Material2d},
};

use crate::c_appstate::AppState;
use crate::helpers::*;
use crate::consts::*;
use crate::c_events::EvShieldCollision;

pub struct MaterialShieldPlugin;

impl Plugin for MaterialShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<MaterialShield>::default())
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(update_material_shield_time))
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(shield_collision))
        ;
    }
}

fn update_material_shield_time(
    mut res_shader_time: ResMut<Assets<MaterialShield>>,
    res_time: Res<Time>,
) {
    for (_, shield) in res_shader_time.iter_mut(){
        shield.time = res_time.seconds_since_startup() as f32;
        shield.time_since_activation += res_time.delta_seconds() as f32;
//        shield.time_since_deactivation += res_time.delta_seconds() as f32;
        shield.time_since_collision += res_time.delta_seconds() as f32;
    }
}

fn shield_collision (
    mut shield_collision_reader: EventReader<EvShieldCollision>,
    mut res_shield: ResMut<Assets<MaterialShield>>,
) {
    for event in shield_collision_reader.iter() {
        let shield_position = event.shield_position;
        let other_position = event.other_position;
        let other_closest_position = closest_position(shield_position.x, shield_position.y, other_position.x, other_position.y);
        let delta = Vec2::new(other_closest_position.x - shield_position.x, other_closest_position.y - shield_position.y);

        let mut collision_angle = (delta.y / delta.x).atan(); // Angle of collision
        if delta.x < 0.0 { collision_angle += PI; } // .atan() can only calculate an angle, not which direction along that angle

        for (_, shield) in res_shield.iter_mut(){
            shield.time_since_collision = 0.0 as f32;
            shield.collision_angle = collision_angle;
        }
    }
}


#[derive(Component, Debug, Clone, TypeUuid)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct MaterialShield {
    pub time: f32,
    pub color: i32, // Blue, Green, Orange, Purple
    pub time_since_activation: f32,
    pub time_since_deactivation: f32,
    pub time_since_collision: f32,
    pub collision_angle: f32,
    pub ring_deactivation_flash: i32,
    pub texture_gradient: Handle<Image>,
}

impl Default for MaterialShield {
    fn default() -> Self {
        Self {
            time: 0.0,
            color: 0, // Blue, Green, Orange, Purple
            time_since_activation: 0.0,
            time_since_deactivation: 0.0,
            time_since_collision: 100.0,
            collision_angle: 0.0,
            ring_deactivation_flash: 0,
            texture_gradient: Handle::default(),
        }
    }
}

#[derive(Clone)]
pub struct GpuMaterialShield {
    _buffer_time: Buffer,
    _buffer_color: Buffer,
    _buffer_time_since_activation: Buffer,
    _buffer_time_since_deactivation: Buffer,
    _buffer_time_since_collision: Buffer,
    _buffer_collision_angle: Buffer,
    _buffer_ring_deactivation_flash: Buffer,
    bind_group: BindGroup,
}

impl RenderAsset for MaterialShield {
    type ExtractedAsset = MaterialShield;
    type PreparedAsset = GpuMaterialShield;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<Self>>,
        SRes<RenderAssets<Image>>,
    );
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline, gpu_images): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let buffer_time = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("shield_material_uniform_buffer_time"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: extracted_asset.time.as_std140().as_bytes(),
        });
        let buffer_color = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("shield_material_uniform_buffer_color"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: extracted_asset.color.as_std140().as_bytes(),
        });
        let buffer_time_since_activation = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("shield_material_uniform_buffer_time_since_activation"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: extracted_asset.time_since_activation.as_std140().as_bytes(),
        });
        let buffer_time_since_deactivation = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("shield_material_uniform_buffer_time_since_deactivation"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: extracted_asset.time_since_deactivation.as_std140().as_bytes(),
        });
        let buffer_time_since_collision = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("shield_material_uniform_buffer_time_since_collision"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: extracted_asset.time_since_collision.as_std140().as_bytes(),
        });
        let buffer_collision_angle = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("shield_material_uniform_buffer_collision_angle"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: extracted_asset.collision_angle.as_std140().as_bytes(),
        });
        let buffer_ring_deactivation_flash = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("shield_material_uniform_buffer_ring_deactivation_flash"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: extracted_asset.ring_deactivation_flash.as_std140().as_bytes(),
        });
        let (texture_view_gradient, sampler_gradient) = if let Some(gpu_image) = gpu_images.get(&extracted_asset.texture_gradient) {
            (&gpu_image.texture_view, &gpu_image.sampler)
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(extracted_asset));
        };

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer_time.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: buffer_color.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: buffer_time_since_activation.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: buffer_time_since_deactivation.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: buffer_time_since_collision.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: buffer_collision_angle.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 6,
                    resource: buffer_ring_deactivation_flash.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 7,
                    resource: BindingResource::TextureView(texture_view_gradient),
                },
                BindGroupEntry {
                    binding: 8,
                    resource: BindingResource::Sampler(sampler_gradient),
                },
            ],
            label: None,
            layout: &material_pipeline.material2d_layout,
        });

        Ok(GpuMaterialShield {
            _buffer_time: buffer_time,
            _buffer_color: buffer_color,
            _buffer_time_since_activation: buffer_time_since_activation,
            _buffer_time_since_deactivation: buffer_time_since_deactivation,
            _buffer_time_since_collision: buffer_time_since_collision,
            _buffer_collision_angle: buffer_collision_angle,
            _buffer_ring_deactivation_flash: buffer_ring_deactivation_flash,
            bind_group,
        })
    }
}

impl Material2d for MaterialShield {
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/shield.wgsl"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            f32::std140_size_static() as u64,
                        ),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            i32::std140_size_static() as u64,
                        ),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            f32::std140_size_static() as u64,
                        ),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            f32::std140_size_static() as u64,
                        ),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 4,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            f32::std140_size_static() as u64,
                        ),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 5,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            f32::std140_size_static() as u64,
                        ),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 6,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            i32::std140_size_static() as u64,
                        ),
                    },
                    count: None,
                },
               // Texture
                BindGroupLayoutEntry {
                    binding: 7,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                // Texture Sampler
                BindGroupLayoutEntry {
                    binding: 8,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: None,
        })
    }
}