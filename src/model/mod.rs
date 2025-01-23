use std::collections::{HashMap, HashSet};

pub mod model;
use model::ElementModel;

pub trait Model {
    fn draw(&self);
}

pub struct ModelManager {
    models: HashMap<String, Box<dyn Model + Send + 'static>>,
}

impl ModelManager {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, vertices: Vec<f32>, indices: Vec<u32>, description: &str) {
        self.models.insert(
            String::from(name),
            Box::new(ElementModel::new(vertices, indices, description)),
        );
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Model + Send + 'static>> {
        self.models.get(name)
    }
}

use lazy_static::lazy_static;

use crate::{debug, warn, RustCraftWrapper};

lazy_static! {
    /// 模型管理器
    pub static ref MODEL_MANAGER: RustCraftWrapper<ModelManager> =
        RustCraftWrapper::new(ModelManager::new());
    static ref NOT_FOUND_MODEL: RustCraftWrapper<HashSet<String>> =
        RustCraftWrapper::new(HashSet::new());
}

impl RustCraftWrapper<ModelManager> {
    /// 以此参数形式载入模型
    ///
    /// # 参数 Parameters
    ///
    /// - `name`: 模型名称
    /// - `vertices`: 顶点数据
    /// - `indices`: 索引数据
    /// - `description`: 顶点数据描述
    ///
    /// # 注解 Note
    ///
    /// 此函数只有在 OpenGL 上下文激活后才能调用
    ///
    /// 顶点数据描述的格式为： [num type;...]
    /// num: 值个数
    /// type: 值类型
    ///     + f: 单精度浮点数
    ///     + i: 整型
    ///     + u: 无符号整型
    ///     + _: 一字节占位符
    /// 例如："3f;2f", "3f;3f;2f"
    ///
    /// # 示例 Examples
    ///
    /// ```
    /// use rustcraft::MODEL_MANAGER;
    ///
    /// let vertices = vec![
    ///     0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, -0.5,
    ///     -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0,
    /// ];
    /// let indices = vec![0, 1, 2, 2, 3, 0];
    /// MODEL_MANAGER.add_model("Face", vertices, indices, "3f;3f;2f");
    /// ```
    pub fn add_model(&self, name: &str, vertices: Vec<f32>, indices: Vec<u32>, description: &str) {
        self.apply(|manager| manager.add(name, vertices, indices, description));
    }

    pub fn load_from_json(&self, json: &str) {
        debug!("RCW<ModelManager>", "尝试载入模型 {}", json);
        let (name, vertices, indices, description) = match ElementModel::load_from_json(json) {
            Ok(v) => v,
            Err(e) => {
                warn!("RCW<ModelManager>", "模型 {} 载入失败: {}", json, e);
                return;
            }
        };
        self.add_model(&name, vertices, indices, &description);
    }

    /// 渲染参数所指定的模型
    ///
    /// # 参数 Parameters
    ///
    /// - `name`: 模型名称
    pub fn draw_model(&self, name: &str) {
        self.apply(|manager| {
            if let Some(model) = manager.get(name) {
                model.draw();
            } else {
                NOT_FOUND_MODEL.apply(|set| {
                    if set.contains(name) {
                        return;
                    }
                    warn!("RCW<ModelManager>", "找不到模型 {}", name);
                    set.insert(String::from(name));
                });
            }
        });
    }
}
