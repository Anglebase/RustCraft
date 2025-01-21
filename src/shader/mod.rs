use crate::{debug, info, warn, RustCraftWrapper};
use gl::types::*;
use std::collections::{HashMap, HashSet};

mod gl_utils;

pub struct ShaderManager {
    programs: HashMap<String, GLuint>,
}

impl ShaderManager {
    fn new() -> Self {
        Self {
            programs: HashMap::new(),
        }
    }

    fn load_from(&mut self, path: &str) {
        debug!("ShaderManager::load_from()", "正在从 {} 加载着色器", path);
        // 顶点着色器文件扩展名集合
        let vert_ext = {
            let mut set = HashSet::new();
            set.insert("vert");
            set.insert("vs");
            set
        };
        let frag_ext = {
            let mut set = HashSet::new();
            set.insert("frag");
            set.insert("fs");
            set
        };
        use std::fs::*;
        // 遍历指定目录
        let dir = match read_dir(path) {
            Ok(dir) => dir,
            Err(e) => {
                warn!(
                    "ShaderManager::load_from()",
                    "无法读取指定目录: {}, 由于 \"{}\"", path, e
                );
                return;
            }
        };
        let mut vert_codes = HashMap::new();
        let mut frag_codes = HashMap::new();
        for entry in dir {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    warn!(
                        "ShaderManager::load_from()",
                        "遍历项目出现错误，由于 \"{}\"", e
                    );
                    continue;
                }
            };
            let path = entry.path();
            // 读取文件内容
            let content = match read_to_string(path.clone()) {
                Ok(content) => content,
                Err(e) => {
                    warn!(
                        "ShaderManager::load_from()",
                        "无法读取文件: {}, 由于 \"{}\"",
                        path.display(),
                        e
                    );
                    continue;
                }
            };
            debug!(
                "ShaderManager::load_from()",
                "检索到文件: {}",
                path.display()
            );
            // 文件扩展名
            let ext: &str = if let Some(ext) = path.extension() {
                ext.to_str().unwrap()
            } else {
                info!(
                    "ShaderManager::load_from()",
                    "忽略未知文件: {}",
                    path.display()
                );
                continue;
            };
            // 文件本名
            let filename = if let Some(ext) = path.file_stem() {
                ext.to_str().unwrap()
            } else {
                info!(
                    "ShaderManager::load_from()",
                    "忽略未知文件: {}",
                    path.display()
                );
                continue;
            };
            // 标记着色器
            if vert_ext.contains(ext) {
                debug!(
                    "ShaderManager::load_from()",
                    "顶点着色器: {}",
                    path.display()
                );
                vert_codes.insert(filename.to_string(), content);
            } else if frag_ext.contains(ext) {
                debug!(
                    "ShaderManager::load_from()",
                    "片段着色器: {}",
                    path.display()
                );
                frag_codes.insert(filename.to_string(), content);
            }
        }
        // 移除不完整的着色器
        let vert_set = vert_codes.keys().cloned().collect::<HashSet<_>>();
        let frag_set = frag_codes.keys().cloned().collect::<HashSet<_>>();
        let vert_set_ignore = vert_set.difference(&frag_set);
        let frag_set_ignore = frag_set.difference(&vert_set);
        for path in vert_set_ignore {
            warn!(
                "ShaderManager::load_from()",
                "顶点着色器未找到匹配的片段着色器: {}", path
            );
        }
        for path in frag_set_ignore {
            warn!(
                "ShaderManager::load_from()",
                "片段着色器未找到匹配的顶点着色器: {}", path
            );
        }
        vert_codes.retain(|path, _| frag_set.contains(path));
        frag_codes.retain(|path, _| vert_set.contains(path));
        // 编译着色器
        info!("ShaderManager::load_from()", "正在编译着色器...");
        let mut vert_shader = HashMap::new();
        let mut frag_shader = HashMap::new();
        for (path, code) in vert_codes {
            let shader = match unsafe { gl_utils::complie_shader(gl::VERTEX_SHADER, &code) } {
                Ok(shader) => Some(shader),
                Err(e) => {
                    warn!(
                        "ShaderManager::load_from()",
                        "顶点着色器\"{}\"编译失败, 由于 \"{}\"", path, e
                    );
                    None
                }
            };
            vert_shader.insert(path, shader);
        }
        for (path, code) in frag_codes {
            let shader = match unsafe { gl_utils::complie_shader(gl::FRAGMENT_SHADER, &code) } {
                Ok(shader) => Some(shader),
                Err(e) => {
                    warn!(
                        "ShaderManager::load_from()",
                        "片段着色器\"{}\"编译失败, 由于 \"{}\"", path, e
                    );
                    None
                }
            };
            frag_shader.insert(path, shader);
        }
        // 移除无效的着色器
        vert_shader.retain(|_, shader| shader.is_some());
        frag_shader.retain(|_, shader| shader.is_some());
        let vert_set = vert_shader.keys().cloned().collect::<HashSet<_>>();
        let frag_set = frag_shader.keys().cloned().collect::<HashSet<_>>();
        let vert_set_ignore = vert_set.difference(&frag_set);
        let frag_set_ignore = frag_set.difference(&vert_set);
        for path in vert_set_ignore {
            warn!(
                "ShaderManager::load_from()",
                "没有与\"{}\"匹配的片段着色器", path
            );
        }
        for path in frag_set_ignore {
            warn!(
                "ShaderManager::load_from()",
                "没有与\"{}\"匹配的顶点着色器", path
            );
        }
        vert_shader.retain(|path, shader| frag_set.contains(path) && shader.is_some());
        frag_shader.retain(|path, shader| vert_set.contains(path) && shader.is_some());
        // 链接着色器
        info!("ShaderManager::load_from()", "正在链接着色器...");
        for (path, vert) in vert_shader {
            let frag = frag_shader.get(&path).unwrap().unwrap();
            let program = match unsafe { gl_utils::link_program(vert.unwrap(), frag) } {
                Ok(program) => program,
                Err(e) => {
                    warn!(
                        "ShaderManager::load_from()",
                        "着色器\"{}\"链接失败, 由于 \"{}\"", path, e
                    );
                    continue;
                }
            };
            self.programs.insert(path, program);
        }
    }
}

lazy_static! {
    static ref NOT_FOUND: RustCraftWrapper<HashSet<String>> = RustCraftWrapper::new(HashSet::new());
}

impl RustCraftWrapper<ShaderManager> {
    pub fn load_from(&self, path: &str) {
        self.apply(|manager| {
            manager.load_from(path);
        });
    }

    pub fn use_program(&self, name: &str) {
        self.apply(|manager| {
            if let Some(program) = manager.programs.get(name) {
                unsafe {
                    gl_utils::use_program(*program);
                }
            } else {
                NOT_FOUND.apply(|set| {
                    if set.contains(name) {
                        return;
                    }
                    warn!(
                        "RCW<ShaderManager>::use_program()",
                        "没有找到名为\"{}\"的着色器", name
                    );
                    set.insert(name.to_string());
                });
            }
        });
    }
}

use lazy_static::lazy_static;
lazy_static! {
    pub static ref SHADER_MANAGER: RustCraftWrapper<ShaderManager> =
        RustCraftWrapper::new(ShaderManager::new());
}
