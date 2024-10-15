mod gs_scene;

use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;
use std::sync::Mutex;

pub struct GaussianSplattingScene {
    pub(crate) file_header_size: u16,
    pub(crate) splat_count: usize,
    pub(crate) buffer: Vec<u8>,
    pub(crate) tex_data: Vec<u32>,
    pub(crate) tex_width: usize,
    pub(crate) tex_height: usize,
    pub(crate) prev_vp: Mutex<Vec<f32>>,
}

/// Loads a .ply or .splat file and returns a [GaussianSplattingScene]
pub async fn load_scene<P: AsRef<Path>>(
    ply_file: P,
) -> anyhow::Result<GaussianSplattingScene> {
    let mut scene = GaussianSplattingScene::new();

    let mut file = File::open(ply_file)?;
    let mut cursor = Cursor::new(Vec::<u8>::new());
    let _size = file.read_to_end(cursor.get_mut());
    match scene.parse_file_header(cursor) {
        Ok((mut c)) => {
            scene.load(&mut c);
            scene.generate_texture();
        }
        Err(e) => {
            tracing::error!("load_scene(): ERROR: {}", e);
            return Err(anyhow::anyhow!("load_scene(): ERROR: {}", e));
        }
    }
    tracing::info!("load_scene(): scene.splat_count={}", scene.splat_count);
    Ok(scene)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
