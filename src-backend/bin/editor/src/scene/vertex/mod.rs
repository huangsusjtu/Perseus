use iced::widget::shader::wgpu;

mod vertex_color;
pub use vertex_color::*;
mod vertex_fix_color;
pub use vertex_fix_color::VertexFixColor;
mod vertex_texture;


pub use vertex_texture::*;

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}
