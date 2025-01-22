use crate::{debug, gl_utils, info, warn, RustCraftWrapper};
use gl::types::*;
use shader::Shader;
use std::collections::{HashMap, HashSet};

mod shader;

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
        debug!("ShaderManager", "正在从 {} 加载着色器", path);
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
                    "ShaderManager",
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
                    warn!("ShaderManager", "遍历项目出现错误，由于 \"{}\"", e);
                    continue;
                }
            };
            let path = entry.path();
            // 读取文件内容
            let content = match read_to_string(path.clone()) {
                Ok(content) => content,
                Err(e) => {
                    warn!(
                        "ShaderManager",
                        "无法读取文件: {}, 由于 \"{}\"",
                        path.display(),
                        e
                    );
                    continue;
                }
            };
            debug!("ShaderManager", "检索到文件: {}", path.display());
            // 文件扩展名
            let ext: &str = if let Some(ext) = path.extension() {
                ext.to_str().unwrap()
            } else {
                info!("ShaderManager", "忽略未知文件: {}", path.display());
                continue;
            };
            // 文件本名
            let filename = if let Some(ext) = path.file_stem() {
                ext.to_str().unwrap()
            } else {
                info!("ShaderManager", "忽略未知文件: {}", path.display());
                continue;
            };
            // 标记着色器
            if vert_ext.contains(ext) {
                debug!("ShaderManager", "顶点着色器: {}", path.display());
                vert_codes.insert(filename.to_string(), content);
            } else if frag_ext.contains(ext) {
                debug!("ShaderManager", "片段着色器: {}", path.display());
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
                "ShaderManager",
                "顶点着色器未找到匹配的片段着色器: {}", path
            );
        }
        for path in frag_set_ignore {
            warn!(
                "ShaderManager",
                "片段着色器未找到匹配的顶点着色器: {}", path
            );
        }
        vert_codes.retain(|path, _| frag_set.contains(path));
        frag_codes.retain(|path, _| vert_set.contains(path));
        // 编译着色器
        info!("ShaderManager", "正在编译着色器...");
        let mut vert_shader = HashMap::new();
        let mut frag_shader = HashMap::new();
        for (path, code) in vert_codes {
            let shader = match unsafe { gl_utils::complie_shader(gl::VERTEX_SHADER, &code) } {
                Ok(shader) => Some(shader),
                Err(e) => {
                    warn!(
                        "ShaderManager",
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
                        "ShaderManager",
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
            warn!("ShaderManager", "没有与\"{}\"匹配的片段着色器", path);
        }
        for path in frag_set_ignore {
            warn!("ShaderManager", "没有与\"{}\"匹配的顶点着色器", path);
        }
        vert_shader.retain(|path, shader| frag_set.contains(path) && shader.is_some());
        frag_shader.retain(|path, shader| vert_set.contains(path) && shader.is_some());
        // 链接着色器
        info!("ShaderManager", "正在链接着色器...");
        for (path, vert) in vert_shader {
            let frag = frag_shader.get(&path).unwrap().unwrap();
            let program = match unsafe { gl_utils::link_program(vert.unwrap(), frag) } {
                Ok(program) => program,
                Err(e) => {
                    warn!(
                        "ShaderManager",
                        "着色器\"{}\"链接失败, 由于 \"{}\"", path, e
                    );
                    continue;
                }
            };
            self.programs.insert(path, program);
        }
    }
}

use lazy_static::lazy_static;

lazy_static! {
    static ref NOT_FOUND: RustCraftWrapper<HashSet<String>> = RustCraftWrapper::new(HashSet::new());
    /// 着色器管理器
    pub static ref SHADER_MANAGER: RustCraftWrapper<ShaderManager> =
        RustCraftWrapper::new(ShaderManager::new());
}

impl RustCraftWrapper<ShaderManager> {
    /// 加载指定目录下的着色器
    ///
    /// # 注解 Note
    ///
    /// 此函数只有在 OpenGL 上下文激活后才能调用
    ///
    /// 从文件载入着色器时，只会保留同时具有成功编译后的顶点着色器和片段着色器的有效的着色器程序
    /// 此函数依据文件名对着色器进行配对，如果没有找到匹配的着色器，则会输出警告信息
    ///
    /// # 参数 Parameters
    /// * `path` - 目录路径
    pub fn load_from(&self, path: &str) {
        self.apply(|manager| {
            manager.load_from(path);
        });
    }

    /// 获取指定名称的着色器
    ///
    /// # 注解 Note
    /// 着色器名称与着色器的源代码文件名一致
    ///
    /// # 参数 Parameters
    /// * `name` - 着色器名称
    ///
    /// # 返回值 Returns
    /// 成功获取着色器时返回 `Some(Shader)`，否则返回 `None`
    pub fn get(&self, name: &str) -> Option<Shader> {
        let mut ret = None;
        self.apply(|manager| {
            if let Some(&program) = manager.programs.get(name) {
                ret = Some(Shader { program });
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
        ret
    }
}
