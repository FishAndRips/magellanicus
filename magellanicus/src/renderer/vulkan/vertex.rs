use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use alloc::string::ToString;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[derive(BufferContents, Vertex)]
pub struct VulkanModelVertex {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],

    #[format(R32G32B32_SFLOAT)]
    pub normal: [f32; 3],

    #[format(R32G32B32_SFLOAT)]
    pub binormal: [f32; 3],

    #[format(R32G32B32_SFLOAT)]
    pub tangent: [f32; 3]
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[derive(BufferContents, Vertex)]
pub struct VulkanModelVertexTextureCoords {
    #[format(R32G32_SFLOAT)]
    pub texture_coords: [f32; 2],
}
