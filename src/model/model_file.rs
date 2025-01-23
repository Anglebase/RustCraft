use super::{array_model::ArrayModel, element_model::ElementModel, Model};

pub fn load_from_json(path: &str) -> Result<(String, Box<dyn Model + Send + 'static>), String> {
    let string = match std::fs::read_to_string(path) {
        Ok(string) => string,
        Err(err) => return Err(format!("读取文件时错误: {}", err)),
    };
    let json = match json::parse(&string) {
        Ok(json) => json,
        Err(err) => return Err(format!("JSON 解析错误: {}", err)),
    };
    if !json.is_object() {
        return Err("JSON 不是对象".to_string());
    }
    if !json.has_key("type") {
        return Err("JSON 中缺少 type 字段".to_string());
    }
    let model_type = if let Some(model_type) = json["type"].as_str() {
        model_type
    } else {
        return Err("JSON 中 type 字段不是有效内容".to_string());
    };
    match model_type {
        "element" => {
            let (name, vertices, indices, description) = ElementModel::load_from_json(&json)?;
            Ok((
                name,
                Box::new(ElementModel::new(vertices, indices, &description)),
            ))
        }
        "array" => {
            let (name, vertices, description) = ArrayModel::load_from_json(&json)?;
            Ok((name, Box::new(ArrayModel::new(vertices, &description))))
        }
        _ => Err(format!("无效的模型类型格式: {}", model_type)),
    }
}
