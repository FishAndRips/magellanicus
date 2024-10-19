use crate::error::MResult;
use std::sync::Arc;
use std::vec::Vec;
use std::vec;
use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::image::SampleCount;
use vulkano::pipeline::graphics::color_blend::{ColorBlendAttachmentState, ColorBlendState};
use vulkano::pipeline::graphics::depth_stencil::{CompareOp, DepthState, DepthStencilState};
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::{FrontFace, RasterizationState};
use vulkano::pipeline::graphics::subpass::PipelineRenderingCreateInfo;
use vulkano::pipeline::graphics::vertex_input::{VertexBufferDescription, VertexDefinition};
use vulkano::pipeline::graphics::viewport::ViewportState;
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo};
use crate::renderer::vulkan::OFFLINE_PIPELINE_COLOR_FORMAT;

#[derive(Copy, Clone, Default, PartialEq)]
pub enum DepthAccess {
    /// The depth as determined by the vertex shader must be less than or equal.
    ///
    /// This will pass as long as nothing is in front of the vertices.
    ///
    /// This is used primarily for transparent shaders.
    DepthReadOnlyTransparent,

    #[default]
    /// The depth as determined by the vertex shader has to equal.
    ///
    /// This will pass if the depth buffer was written to already with the exact vertices.
    ///
    /// This is used if one needs to overlay on top of something already written.
    DepthReadOnly,

    /// The depth as determined by the vertex must be less than or equal.
    ///
    /// This will pass as long as nothing is in front of the vertices.
    ///
    /// This is used if one needs to write to the depth buffer.
    DepthWrite,

    /// The depth buffer is completely ignored.
    ///
    /// Draw on top of whatever is there.
    NoDepth
}

#[derive(Clone)]
pub struct PipelineSettings {
    /// Determines how depth is accessed.
    pub depth_access: DepthAccess,

    /// Vertex data expected to be bound and sent to the shader.
    pub vertex_buffer_descriptions: Vec<VertexBufferDescription>,

    /// Determines how to blend
    pub color_blend_attachment_state: ColorBlendAttachmentState,

    /// Sample count to use.
    pub samples: SampleCount,

    /// Color format to use
    pub format: Format
}

impl Default for PipelineSettings {
    fn default() -> Self {
        Self {
            depth_access: Default::default(),
            vertex_buffer_descriptions: Default::default(),
            color_blend_attachment_state: Default::default(),
            samples: SampleCount::Sample1,
            format: OFFLINE_PIPELINE_COLOR_FORMAT
        }
    }
}

pub fn load_pipeline(
    device: Arc<Device>,
    load_vertex_shader: fn (Arc<Device>) -> Result<Arc<vulkano::shader::ShaderModule>, vulkano::Validated<vulkano::VulkanError>>,
    load_fragment_shader: fn (Arc<Device>) -> Result<Arc<vulkano::shader::ShaderModule>, vulkano::Validated<vulkano::VulkanError>>,
    settings: &PipelineSettings
) -> MResult<Arc<GraphicsPipeline>> {
    let vertex_shader = load_vertex_shader(device.clone())?
        .entry_point("main")
        .expect("Missing main() entry point for vertex pipeline!");
    let fragment_shader = load_fragment_shader(device.clone())?
        .entry_point("main")
        .expect("Missing main() entry point for fragment pipeline!");

    let vertex_input_state = settings
        .vertex_buffer_descriptions
        .definition(&vertex_shader.info().input_interface)?;

    let stages = [
        PipelineShaderStageCreateInfo::new(vertex_shader),
        PipelineShaderStageCreateInfo::new(fragment_shader),
    ];

    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
    )?;

    let subpass = PipelineRenderingCreateInfo {
        color_attachment_formats: vec![Some(settings.format)],
        depth_attachment_format: Some(Format::D32_SFLOAT),
        ..Default::default()
    };

    let blend = ColorBlendState::with_attachment_states(
        subpass.color_attachment_formats.len() as u32,
        settings.color_blend_attachment_state.clone(),
    );

    let pipeline = GraphicsPipeline::new(
        device.clone(),
        None,
        GraphicsPipelineCreateInfo {
            stages: stages.into_iter().collect(),
            vertex_input_state: Some(vertex_input_state),
            input_assembly_state: Some(InputAssemblyState::default()),
            viewport_state: Some(ViewportState::default()),
            rasterization_state: Some(RasterizationState {
                front_face: FrontFace::Clockwise,
                ..RasterizationState::default()
            }),
            multisample_state: Some(MultisampleState {
                rasterization_samples: settings.samples,
                ..MultisampleState::default()
            }),
            color_blend_state: Some(blend),
            dynamic_state: [
                DynamicState::Viewport,
                DynamicState::CullMode,
            ].into_iter().collect(),
            depth_stencil_state: Some(DepthStencilState {
                depth: Some(DepthState {
                    write_enable: settings.depth_access == DepthAccess::DepthWrite,
                    compare_op: match settings.depth_access {
                        DepthAccess::DepthWrite => CompareOp::LessOrEqual,
                        DepthAccess::DepthReadOnly => CompareOp::Equal,
                        DepthAccess::DepthReadOnlyTransparent => CompareOp::LessOrEqual,
                        DepthAccess::NoDepth => CompareOp::Always
                    }
                }),
                ..DepthStencilState::default()
            }),
            subpass: Some(subpass.into()),

            ..GraphicsPipelineCreateInfo::layout(layout)
        }
    )?;

    Ok(pipeline)
}
