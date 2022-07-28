use image::GenericImageView;
use serde::{Serialize, Deserialize, ser::Error};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Metadata {
    image_width: Option<String>,
    image_length: Option<String>,
    bits_per_sample: Option<String>,
    compression: Option<String>,
    photometric_interpretation: Option<String>,
    image_description: Option<String>,
    make: Option<String>,
    model: Option<String>,
    strip_offsets: Option<String>,
    orientation: Option<String>,
    samples_per_pixel: Option<String>,
    rows_per_strip: Option<String>,
    strip_byte_counts: Option<String>,
    x_resolution: Option<String>,
    y_resolution: Option<String>,
    planar_configuration: Option<String>,
    resolution_unit: Option<String>,
    transfer_function: Option<String>,
    software: Option<String>,
    date_time: Option<String>,
    artist: Option<String>,
    white_point: Option<String>,
    primary_chromaticities: Option<String>,
    tile_offsets: Option<String>,
    tile_byte_counts: Option<String>,
    reference_black_white: Option<String>,
    copyright: Option<String>,
    exposure_time: Option<String>,
    f_number: Option<String>,
    exposure_program: Option<String>,
    spectral_sensitivity: Option<String>,
    photographic_sensitivity: Option<String>,
    oecf: Option<String>,
    sensitivity_type: Option<String>,
    standard_output_sensitivity: Option<String>,
    recommended_exposure_index: Option<String>,
    iso_speed: Option<String>,
    iso_speed_latitudeyyy: Option<String>,
    iso_speed_latitudezzz: Option<String>,
    exif_version: Option<String>,
    date_time_original: Option<String>,
    date_time_digitized: Option<String>,
    offset_time: Option<String>,
    offset_time_original: Option<String>,
    offset_time_digitized: Option<String>,
    components_configuration: Option<String>,
    compressed_bits_per_pixel: Option<String>,
    shutter_speed_value: Option<String>,
    aperture_value: Option<String>,
    brightness_value: Option<String>,
    exposure_bias_value: Option<String>,
    max_aperture_value: Option<String>,
    subject_distance: Option<String>,
    metering_mode: Option<String>,
    light_source: Option<String>,
    flash: Option<String>,
    focal_length: Option<String>,
    subject_area: Option<String>,
    sub_sec_time: Option<String>,
    sub_sec_time_original: Option<String>,
    sub_sec_time_digitized: Option<String>,
    temperature: Option<String>,
    humidity: Option<String>,
    pressure: Option<String>,
    water_depth: Option<String>,
    acceleration: Option<String>,
    camera_elevation_angle: Option<String>,
    flashpix_version: Option<String>,
    color_space: Option<String>,
    pixel_x_dimension: Option<String>,
    pixel_y_dimension: Option<String>,
    related_sound_file: Option<String>,
    flash_energy: Option<String>,
    spatial_frequency_response: Option<String>,
    contrast: Option<String>,
    saturation: Option<String>,
    sharpness: Option<String>,
    source_exposure_times_of_composite_image: Option<String>,
    gamma: Option<String>,
    gps_version_id: Option<String>,
    gps_dest_distance: Option<String>,
    gps_processing_method: Option<String>,
    gps_area_information: Option<String>,
    gps_date_stamp: Option<String>,
    gps_differential: Option<String>,
    gps_positioning_error: Option<String>,
    interoperability_index: Option<String>,
    interoperability_version: Option<String>,
    related_image_file_format: Option<String>,
    related_image_width: Option<String>,
    related_image_length: Option<String>,
}

impl std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(std::fmt::Error::custom)?;
        write!(f, "{}", json)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ImageData {
    id: String,
    name: String,
    metadata: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Usage: cargo run <image_path>");
    }

    let filepath = &args[1].split("/").collect::<Vec<&str>>();
    let filename = &filepath[filepath.len() - 1];
    let img_buff = image::open(&args[1]).unwrap();
    let img_dimension = img_buff.dimensions();
    let img_base64 = base64::encode(&img_buff.into_bytes());    

    let req_metadata: serde_json::Value = reqwest::Client::new()
          .post("http://localhost:8000/upload")
          .json(&serde_json::json!({
              "name": filename,
              "width": img_dimension.0,
              "height": img_dimension.1,
              "data": img_base64,
          }))
          .send()
          .await?
          .json()
          .await?;
    
    let data: ImageData = serde_json::from_value(req_metadata).unwrap();
    let meta: Metadata = serde_json::from_str(data.metadata.as_str()).unwrap();
    println!("\n\nid: {},\nname: {}, \nmetadata: {}", data.id, data.name, meta);

    Ok(())
}
