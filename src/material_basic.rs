use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
        render_resource::*,
        renderer::RenderDevice,
    },
    sprite::{Material2dPipeline, Material2dPlugin, Material2d},
};

pub struct MaterialBasicPlugin;

impl Plugin for MaterialBasicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<MaterialBasic>::default())
        ;
    }
}

#[derive(Component, Debug, Clone, TypeUuid)]
#[uuid = "badbcd09-76fc-45a7-86da-b9e227c5634b"]
pub struct MaterialBasic {
    pub texture: Handle<Image>,
}

impl Default for MaterialBasic {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
        }
    }
}

#[derive(Clone)]
pub struct GpuMaterialBasic {
    bind_group: BindGroup,
}

impl RenderAsset for MaterialBasic {
    type ExtractedAsset = MaterialBasic;
    type PreparedAsset = GpuMaterialBasic;
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
        let (texture_view_gradient, sampler_gradient) = if let Some(gpu_image) = gpu_images.get(&extracted_asset.texture) {
            (&gpu_image.texture_view, &gpu_image.sampler)
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(extracted_asset));
        };

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(texture_view_gradient),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(sampler_gradient),
                },
            ],
            label: None,
            layout: &material_pipeline.material2d_layout,
        });

        Ok(GpuMaterialBasic {
            bind_group,
        })
    }
}

impl Material2d for MaterialBasic {
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/basic_texture.wgsl"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
              // Texture
                BindGroupLayoutEntry {
                    binding: 0,
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
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: None,
        })
    }
}