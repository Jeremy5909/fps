use crate::{program::Program, texture::Texture};

use super::Element;

impl Element {
    pub fn add_texture(&mut self, texture_path: &str) -> Result<(), String> {
        let texture = Texture::new();
        let file_type = texture_path
            .split(|x| x == '.')
            .skip(1)
            .next()
            .ok_or("Must have file type")?;
        match file_type {
            "jpg" => texture.load_jpg(texture_path).map_err(|e| e.to_string())?,
            "png" => texture.load_png(texture_path).map_err(|e| e.to_string())?,
            _ => return Err(String::from("Unkown file type")),
        }
        self.textures.push(texture);
        Ok(())
    }
    pub fn add_program(&mut self, program: Program) -> Result<(), String> {
        self.program = Some(program);
        Ok(())
    }
}
