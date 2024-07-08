pub fn get_image_name_with_version(image: &str, version: &Option<String>) -> String {
    let mut image_version = "latest";
    if let Some(v) = version {
        image_version = v;
    }
    format!("{}:{}", image, image_version)
}

pub fn sanitize_app_name(app: &str) -> String {
    app.chars()
        .filter(|&c| c.is_alphanumeric() || c == '.' || c == '_' || c == '-')
        .collect::<String>()
        .trim_start_matches('/')
        .trim_end()
        .to_string()
}
