use std::sync::Arc;
use glam::Mat4;
use vulkano::buffer::Subbuffer;
use vulkano::image::view::ImageView;
use crate::types::Vert;

pub struct RenderedObject {
    pub transform: Mat4,
    pub visible: bool,

    pub(crate) vertex_buffer: Subbuffer<[Vert]>,
    pub(crate) index_buffer: Subbuffer<[u32]>,
    pub(crate) texture: Option<Arc<ImageView>>,
}