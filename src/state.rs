mod mesh;
mod shader;
mod texture;

use glutin::dpi::PhysicalSize;
use glutin::{PossiblyCurrent, WindowedContext};
use mesh::{Mesh, Vertex};
use shader::Program;
use std::env;
use texture::Texture;

/// アプリケーションの状態
pub struct State {
    program: Program,
    texture: Texture,
    mesh: Mesh,
}

impl State {
    /// Stateを生成
    pub fn new(context: &WindowedContext<PossiblyCurrent>) -> Result<Self, String> {
        gl::load_with(|s| context.get_proc_address(s));

        let size = context.window().inner_size();
        unsafe {
            gl::Viewport(0, 0, size.width as i32, size.height as i32);
        }

        let program =
            Program::from_sources(include_str!("shader.vert"), include_str!("shader.frag"))
                .unwrap();

        let path = match env::args().nth(1) {
            Some(path) => path,
            None => return Err("引数の数が正しくありません".into()),
        };
        let texture = match Texture::from_path(&path) {
            Ok(texture) => texture,
            Err(_) => return Err(format!("`{}` を読み込むことができません", path)),
        };
        context.window().set_title(&format!("{} - imgview", path));

        let window_aspect = size.width as f32 / size.height as f32;
        let texture_aspect = texture.width() as f32 / texture.height() as f32;
        let (w, h) = if window_aspect > texture_aspect {
            (texture_aspect / window_aspect, 1.0)
        } else {
            (1.0, window_aspect / texture_aspect)
        };
        let mesh = Mesh::new(
            vec![
                Vertex {
                    position: [-w, -h],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [-w, h],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [w, h],
                    tex_coords: [1.0, 1.0],
                },
                Vertex {
                    position: [w, -h],
                    tex_coords: [1.0, 0.0],
                },
            ],
            &[0, 2, 1, 0, 3, 2],
        );

        Ok(Self {
            program,
            texture,
            mesh,
        })
    }

    /// リサイズ時の処理
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        unsafe {
            gl::Viewport(0, 0, size.width as i32, size.height as i32);
        }

        let window_aspect = size.width as f32 / size.height as f32;
        let texture_aspect = self.texture.width() as f32 / self.texture.height() as f32;
        let (w, h) = if window_aspect > texture_aspect {
            (texture_aspect / window_aspect, 1.0)
        } else {
            (1.0, window_aspect / texture_aspect)
        };
        self.mesh.update_data(vec![
            Vertex {
                position: [-w, -h],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-w, h],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [w, h],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [w, -h],
                tex_coords: [1.0, 0.0],
            },
        ]);
    }

    /// 描画
    pub fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.program.set_used();
        self.mesh.render(&mut self.program, &self.texture);
    }
}
