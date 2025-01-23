use std::collections::HashMap;
pub mod texture;
use crate::debug;
use texture::Texture;

/// 纹理资源管理器
pub struct TextureManager {
    textures: HashMap<String, Texture>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    /// 加载指定目录下的纹理
    /// 
    /// # 参数 Parameters
    /// * `dir` - 目录路径
    /// 
    /// # 注解 Note
    /// 
    /// 加载的纹理将以它的文件名作为名称存储在管理器中
    pub fn load_from(&mut self, dir: &str) {
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
    /// 纹理资源管理器
    pub static ref TEXTURE_MANAGER: RustCraftWrapper<TextureManager> =
        RustCraftWrapper::new(TextureManager::new());
}

impl RustCraftWrapper<TextureManager> {
    /// 加载指定目录下的纹理
    ///
    /// # 注解 Note
    ///
    /// 此函数只有在 OpenGL 上下文激活后才能调用
    ///
    /// # 参数 Parameters
    /// * `path` - 目录路径
    pub fn load_from(&self, dir: &str) {
        debug!("RCW<TextureManager>", "正在从 {} 加载纹理", dir);
        self.apply(|tm| tm.load_from(dir));
    }

    /// 将指定名称的纹理绑定到指定 ID
    ///
    /// # 注解 Note
    ///
    /// 此函数只有在 OpenGL 上下文激活后才能调用
    ///
    /// # 参数 Parameters
    /// * `name` - 纹理名称
    /// * `id` - 纹理 ID
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
