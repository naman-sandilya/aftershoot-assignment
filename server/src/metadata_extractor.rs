use rocket::serde::json::{json, Value};
use std::collections::HashMap;

pub fn regenerate(
    data: &str,
    name: &str,
    width: u32,
    height: u32,
) -> Result<Value, crate::errors::Error> {
    let filepath = format!("./recovered/{}", name);
    let re_img_buff = base64::decode(data)?;

    let img = image::RgbaImage::from_raw(width, height, re_img_buff).ok_or(
        image::ImageError::Encoding(image::error::EncodingError::new(
            image::error::ImageFormatHint::Unknown,
            "Unable to encode buffer",
        )),
    )?;
    // image::save_buffer_with_format(&filepath, &re_img_buff, width, height, image::ColorType::Rgba8, image::ImageFormat::Jpeg)?;
    img.save(&filepath)?;

    Ok(extract_meta(&filepath)?)
}

pub fn extract_meta(img_path: &str) -> Result<Value, crate::errors::Error> {
    let img = std::fs::File::open(img_path)?;
    let mut bufreader = std::io::BufReader::new(&img);
    let exif_data = exif::Reader::new().read_from_container(&mut bufreader)?;

    let mut metadata = HashMap::new();
    for f in exif_data.fields() {
        if f.tag.description() == None
            || f.tag.to_string() == "Manufacturer notes"
            || f.tag.to_string() == "User comments"
        {
            continue;
        }

        metadata.insert(f.tag.to_string(), f.display_value().to_string());
    }

    Ok(json!(metadata))
}
