use macroquad::prelude::*;
use std::collections::HashMap;
use std::fs;

pub struct AssetManager {
    pub images: HashMap<String, Texture2D>,
}

// TODO: Optimize
impl AssetManager {
    // This sucks up all assets from the assets folder into the game
    pub async fn new() -> Self {
        let mut manager = Self {
            images: HashMap::new(),
        };

        let texture_paths = fs::read_dir("assets/images").unwrap();

        for path in texture_paths {
            let p = path.unwrap().path();
            manager.images.insert(p.file_stem().unwrap().to_str().unwrap().to_string(), load_texture(p.to_str().unwrap()).await.unwrap());
        }

        return manager;
    }
}