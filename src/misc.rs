pub fn get_image_name_with_version(image: &str, version: &Option<String>) -> String {
    let mut image_version = "latest";
    if let Some(v) = version {
        image_version = v;
    }
    format!("{}:{}", image, image_version)
}
