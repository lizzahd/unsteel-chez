use macroquad::prelude::*;
use macroquad::audio::{Sound, load_sound};
use std::collections::HashMap;
use std::fs;

pub struct AssetManager {
    pub images: HashMap<String, Texture2D>,
    pub sounds: HashMap<String, Sound>,
}

// TODO: Optimize
impl AssetManager {
    // This sucks up all assets from the assets folder into the game
    pub async fn new(path: &str) -> Self {
        let mut manager = Self {
            images: HashMap::new(),
            sounds: HashMap::new(),
        };

        let texture_paths = fs::read_dir(path).unwrap();

        for path in texture_paths {
            let p = path.unwrap().path();
            if let Some(extension) = p.extension() {
                if let Some(ext_str) = extension.to_str() {
                    let name = p.file_stem().unwrap().to_str().unwrap().to_string();
                    match ext_str {
                        "png" => {
                            manager.images.insert(name, load_texture(p.to_str().unwrap()).await.unwrap());
                        },
                        "wav" => {
                            manager.sounds.insert(name, load_sound(p.to_str().unwrap()).await.unwrap());
                        },
                        _ => {}
                    }
                }
            }
        }

        return manager;
    }
}