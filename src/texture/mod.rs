use std::collections::HashMap;
pub mod texture;
use crate::debug;
use texture::Texture;

pub struct TextureManager {
    textures: HashMap<String, Texture>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn init(&mut self, dir: &str) {
        use std::fs::*;
        let entries = if let Ok(entries) = read_dir(dir) {
            entries
        } else {
            warn!("TextureManager", "无法读取目录 {}", dir);
            return;
        };
        // 遍历目录中的文件
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    warn!("TextureManager", "异常由于 {}", err);
                    continue;
                }
            };
            debug!("TextureManager", "检索到文件 {}", entry.path().display());
            let path = entry.path();
            let str_path = if let Some(str_path) = path.to_str() {
                str_path
            } else {
                warn!("TextureManager", "无法转换路径为字符串 {}", path.display());
                continue;
            };
            let name = if let Some(name) = path.file_stem() {
                if let Some(name) = name.to_str() {
                    name.to_string()
                } else {
                    warn!(
                        "TextureManager",
                        "无法转换文件名为字符串 {}",
                        path.display()
                    );
                    continue;
                }
            } else {
                warn!("TextureManager", "无效的文件路径 {}", path.display());
                continue;
            };
            let tex = if let Some(tex) = Texture::new_from(str_path) {
                tex
            } else {
                continue;
            };
            self.textures.insert(name, tex);
        }
    }
}

use lazy_static::lazy_static;

use crate::{warn, RustCraftWrapper};
lazy_static! {
    pub static ref TEXTURE_MANAGER: RustCraftWrapper<TextureManager> =
        RustCraftWrapper::new(TextureManager::new());
}

impl RustCraftWrapper<TextureManager> {
    pub fn load_from(&self, dir: &str) {
        debug!("RCW<TextureManager>", "正在从 {} 加载纹理", dir);
        self.apply(|tm| tm.init(dir));
    }

    pub fn bind(&self, name: &str, id: u32) {
        self.apply(|tm| {
            if tm.textures.contains_key(name) {
                let texture = tm.textures.get(name).unwrap();
                texture.bind(id);
            } else {
                warn!("RCW<TextureManager>", "没有名为 {} 的纹理", name);
            }
        });
    }
}
