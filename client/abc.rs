#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use image::GenericImageView;
use serde::{Serialize, Deserialize, ser::Error};
use serde_with::skip_serializing_none;
#[serde(rename_all = "PascalCase")]
struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_length: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bits_per_sample: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photometric_interpretation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    make: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strip_offsets: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orientation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    samples_per_pixel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rows_per_strip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strip_byte_counts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x_resolution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y_resolution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    planar_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolution_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_function: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    software: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    white_point: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_chromaticities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tile_offsets: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tile_byte_counts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_black_white: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exposure_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    f_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exposure_program: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spectral_sensitivity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photographic_sensitivity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oecf: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitivity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standard_output_sensitivity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recommended_exposure_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iso_speed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iso_speed_latitudeyyy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iso_speed_latitudezzz: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exif_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_time_original: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_time_digitized: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset_time_original: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset_time_digitized: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compressed_bits_per_pixel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shutter_speed_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aperture_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brightness_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exposure_bias_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_aperture_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_distance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metering_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    light_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    focal_length: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_area: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_sec_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_sec_time_original: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_sec_time_digitized: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    humidity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pressure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    water_depth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acceleration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    camera_elevation_angle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flashpix_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color_space: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pixel_x_dimension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pixel_y_dimension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_sound_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flash_energy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spatial_frequency_response: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contrast: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saturation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sharpness: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_exposure_times_of_composite_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gamma: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gps_version_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gps_dest_distance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gps_processing_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gps_area_information: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gps_date_stamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gps_differential: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gps_positioning_error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interoperability_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interoperability_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_image_file_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_image_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_image_length: Option<String>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Metadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "image_width",
            "image_length",
            "bits_per_sample",
            "compression",
            "photometric_interpretation",
            "image_description",
            "make",
            "model",
            "strip_offsets",
            "orientation",
            "samples_per_pixel",
            "rows_per_strip",
            "strip_byte_counts",
            "x_resolution",
            "y_resolution",
            "planar_configuration",
            "resolution_unit",
            "transfer_function",
            "software",
            "date_time",
            "artist",
            "white_point",
            "primary_chromaticities",
            "tile_offsets",
            "tile_byte_counts",
            "reference_black_white",
            "copyright",
            "exposure_time",
            "f_number",
            "exposure_program",
            "spectral_sensitivity",
            "photographic_sensitivity",
            "oecf",
            "sensitivity_type",
            "standard_output_sensitivity",
            "recommended_exposure_index",
            "iso_speed",
            "iso_speed_latitudeyyy",
            "iso_speed_latitudezzz",
            "exif_version",
            "date_time_original",
            "date_time_digitized",
            "offset_time",
            "offset_time_original",
            "offset_time_digitized",
            "components_configuration",
            "compressed_bits_per_pixel",
            "shutter_speed_value",
            "aperture_value",
            "brightness_value",
            "exposure_bias_value",
            "max_aperture_value",
            "subject_distance",
            "metering_mode",
            "light_source",
            "flash",
            "focal_length",
            "subject_area",
            "sub_sec_time",
            "sub_sec_time_original",
            "sub_sec_time_digitized",
            "temperature",
            "humidity",
            "pressure",
            "water_depth",
            "acceleration",
            "camera_elevation_angle",
            "flashpix_version",
            "color_space",
            "pixel_x_dimension",
            "pixel_y_dimension",
            "related_sound_file",
            "flash_energy",
            "spatial_frequency_response",
            "contrast",
            "saturation",
            "sharpness",
            "source_exposure_times_of_composite_image",
            "gamma",
            "gps_version_id",
            "gps_dest_distance",
            "gps_processing_method",
            "gps_area_information",
            "gps_date_stamp",
            "gps_differential",
            "gps_positioning_error",
            "interoperability_index",
            "interoperability_version",
            "related_image_file_format",
            "related_image_width",
            "related_image_length",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self.image_width,
            &&self.image_length,
            &&self.bits_per_sample,
            &&self.compression,
            &&self.photometric_interpretation,
            &&self.image_description,
            &&self.make,
            &&self.model,
            &&self.strip_offsets,
            &&self.orientation,
            &&self.samples_per_pixel,
            &&self.rows_per_strip,
            &&self.strip_byte_counts,
            &&self.x_resolution,
            &&self.y_resolution,
            &&self.planar_configuration,
            &&self.resolution_unit,
            &&self.transfer_function,
            &&self.software,
            &&self.date_time,
            &&self.artist,
            &&self.white_point,
            &&self.primary_chromaticities,
            &&self.tile_offsets,
            &&self.tile_byte_counts,
            &&self.reference_black_white,
            &&self.copyright,
            &&self.exposure_time,
            &&self.f_number,
            &&self.exposure_program,
            &&self.spectral_sensitivity,
            &&self.photographic_sensitivity,
            &&self.oecf,
            &&self.sensitivity_type,
            &&self.standard_output_sensitivity,
            &&self.recommended_exposure_index,
            &&self.iso_speed,
            &&self.iso_speed_latitudeyyy,
            &&self.iso_speed_latitudezzz,
            &&self.exif_version,
            &&self.date_time_original,
            &&self.date_time_digitized,
            &&self.offset_time,
            &&self.offset_time_original,
            &&self.offset_time_digitized,
            &&self.components_configuration,
            &&self.compressed_bits_per_pixel,
            &&self.shutter_speed_value,
            &&self.aperture_value,
            &&self.brightness_value,
            &&self.exposure_bias_value,
            &&self.max_aperture_value,
            &&self.subject_distance,
            &&self.metering_mode,
            &&self.light_source,
            &&self.flash,
            &&self.focal_length,
            &&self.subject_area,
            &&self.sub_sec_time,
            &&self.sub_sec_time_original,
            &&self.sub_sec_time_digitized,
            &&self.temperature,
            &&self.humidity,
            &&self.pressure,
            &&self.water_depth,
            &&self.acceleration,
            &&self.camera_elevation_angle,
            &&self.flashpix_version,
            &&self.color_space,
            &&self.pixel_x_dimension,
            &&self.pixel_y_dimension,
            &&self.related_sound_file,
            &&self.flash_energy,
            &&self.spatial_frequency_response,
            &&self.contrast,
            &&self.saturation,
            &&self.sharpness,
            &&self.source_exposure_times_of_composite_image,
            &&self.gamma,
            &&self.gps_version_id,
            &&self.gps_dest_distance,
            &&self.gps_processing_method,
            &&self.gps_area_information,
            &&self.gps_date_stamp,
            &&self.gps_differential,
            &&self.gps_positioning_error,
            &&self.interoperability_index,
            &&self.interoperability_version,
            &&self.related_image_file_format,
            &&self.related_image_width,
            &&self.related_image_length,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "Metadata", names, values)
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Metadata {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "Metadata",
                false as usize
                    + if Option::is_none(&self.image_width) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.image_length) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.bits_per_sample) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.compression) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.photometric_interpretation) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.image_description) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.make) { 0 } else { 1 }
                    + if Option::is_none(&self.model) { 0 } else { 1 }
                    + if Option::is_none(&self.strip_offsets) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.orientation) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.samples_per_pixel) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.rows_per_strip) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.strip_byte_counts) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.x_resolution) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.y_resolution) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.planar_configuration) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.resolution_unit) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.transfer_function) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.software) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.date_time) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.artist) { 0 } else { 1 }
                    + if Option::is_none(&self.white_point) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.primary_chromaticities) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.tile_offsets) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.tile_byte_counts) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.reference_black_white) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.copyright) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.exposure_time) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.f_number) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.exposure_program) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.spectral_sensitivity) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.photographic_sensitivity) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.oecf) { 0 } else { 1 }
                    + if Option::is_none(&self.sensitivity_type) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.standard_output_sensitivity) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.recommended_exposure_index) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.iso_speed) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.iso_speed_latitudeyyy) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.iso_speed_latitudezzz) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.exif_version) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.date_time_original) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.date_time_digitized) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.offset_time) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.offset_time_original) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.offset_time_digitized) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.components_configuration) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.compressed_bits_per_pixel) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.shutter_speed_value) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.aperture_value) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.brightness_value) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.exposure_bias_value) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.max_aperture_value) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.subject_distance) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.metering_mode) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.light_source) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.flash) { 0 } else { 1 }
                    + if Option::is_none(&self.focal_length) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.subject_area) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.sub_sec_time) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.sub_sec_time_original) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.sub_sec_time_digitized) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.temperature) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.humidity) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.pressure) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.water_depth) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.acceleration) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.camera_elevation_angle) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.flashpix_version) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.color_space) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.pixel_x_dimension) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.pixel_y_dimension) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.related_sound_file) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.flash_energy) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.spatial_frequency_response) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.contrast) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.saturation) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.sharpness) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.source_exposure_times_of_composite_image) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.gamma) { 0 } else { 1 }
                    + if Option::is_none(&self.gps_version_id) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.gps_dest_distance) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.gps_processing_method) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.gps_area_information) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.gps_date_stamp) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.gps_differential) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.gps_positioning_error) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.interoperability_index) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.interoperability_version) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.related_image_file_format) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.related_image_width) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.related_image_length) {
                        0
                    } else {
                        1
                    },
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            if !Option::is_none(&self.image_width) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ImageWidth",
                    &self.image_width,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "ImageWidth") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.image_length) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ImageLength",
                    &self.image_length,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "ImageLength") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.bits_per_sample) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "BitsPerSample",
                    &self.bits_per_sample,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "BitsPerSample")
                {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.compression) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Compression",
                    &self.compression,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Compression") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.photometric_interpretation) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "PhotometricInterpretation",
                    &self.photometric_interpretation,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "PhotometricInterpretation",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.image_description) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ImageDescription",
                    &self.image_description,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "ImageDescription",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.make) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Make",
                    &self.make,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Make") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.model) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Model",
                    &self.model,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Model") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.strip_offsets) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "StripOffsets",
                    &self.strip_offsets,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "StripOffsets") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.orientation) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Orientation",
                    &self.orientation,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Orientation") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.samples_per_pixel) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "SamplesPerPixel",
                    &self.samples_per_pixel,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "SamplesPerPixel",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.rows_per_strip) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "RowsPerStrip",
                    &self.rows_per_strip,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "RowsPerStrip") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.strip_byte_counts) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "StripByteCounts",
                    &self.strip_byte_counts,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "StripByteCounts",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.x_resolution) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "XResolution",
                    &self.x_resolution,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "XResolution") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.y_resolution) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "YResolution",
                    &self.y_resolution,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "YResolution") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.planar_configuration) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "PlanarConfiguration",
                    &self.planar_configuration,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "PlanarConfiguration",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.resolution_unit) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ResolutionUnit",
                    &self.resolution_unit,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "ResolutionUnit")
                {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.transfer_function) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "TransferFunction",
                    &self.transfer_function,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "TransferFunction",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.software) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Software",
                    &self.software,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Software") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.date_time) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "DateTime",
                    &self.date_time,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "DateTime") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.artist) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Artist",
                    &self.artist,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Artist") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.white_point) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "WhitePoint",
                    &self.white_point,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "WhitePoint") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.primary_chromaticities) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "PrimaryChromaticities",
                    &self.primary_chromaticities,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "PrimaryChromaticities",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.tile_offsets) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "TileOffsets",
                    &self.tile_offsets,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "TileOffsets") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.tile_byte_counts) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "TileByteCounts",
                    &self.tile_byte_counts,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "TileByteCounts")
                {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.reference_black_white) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ReferenceBlackWhite",
                    &self.reference_black_white,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "ReferenceBlackWhite",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.copyright) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Copyright",
                    &self.copyright,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Copyright") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.exposure_time) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ExposureTime",
                    &self.exposure_time,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "ExposureTime") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.f_number) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "FNumber",
                    &self.f_number,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "FNumber") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.exposure_program) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ExposureProgram",
                    &self.exposure_program,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "ExposureProgram",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.spectral_sensitivity) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "SpectralSensitivity",
                    &self.spectral_sensitivity,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "SpectralSensitivity",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.photographic_sensitivity) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "PhotographicSensitivity",
                    &self.photographic_sensitivity,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "PhotographicSensitivity",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.oecf) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Oecf",
                    &self.oecf,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Oecf") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.sensitivity_type) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "SensitivityType",
                    &self.sensitivity_type,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "SensitivityType",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.standard_output_sensitivity) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "StandardOutputSensitivity",
                    &self.standard_output_sensitivity,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "StandardOutputSensitivity",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.recommended_exposure_index) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "RecommendedExposureIndex",
                    &self.recommended_exposure_index,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "RecommendedExposureIndex",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.iso_speed) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "IsoSpeed",
                    &self.iso_speed,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "IsoSpeed") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.iso_speed_latitudeyyy) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "IsoSpeedLatitudeyyy",
                    &self.iso_speed_latitudeyyy,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "IsoSpeedLatitudeyyy",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.iso_speed_latitudezzz) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "IsoSpeedLatitudezzz",
                    &self.iso_speed_latitudezzz,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "IsoSpeedLatitudezzz",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.exif_version) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ExifVersion",
                    &self.exif_version,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "ExifVersion") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.date_time_original) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "DateTimeOriginal",
                    &self.date_time_original,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "DateTimeOriginal",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.date_time_digitized) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "DateTimeDigitized",
                    &self.date_time_digitized,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "DateTimeDigitized",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.offset_time) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "OffsetTime",
                    &self.offset_time,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "OffsetTime") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.offset_time_original) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "OffsetTimeOriginal",
                    &self.offset_time_original,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "OffsetTimeOriginal",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.offset_time_digitized) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "OffsetTimeDigitized",
                    &self.offset_time_digitized,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "OffsetTimeDigitized",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.components_configuration) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ComponentsConfiguration",
                    &self.components_configuration,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "ComponentsConfiguration",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.compressed_bits_per_pixel) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "CompressedBitsPerPixel",
                    &self.compressed_bits_per_pixel,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "CompressedBitsPerPixel",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.shutter_speed_value) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ShutterSpeedValue",
                    &self.shutter_speed_value,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "ShutterSpeedValue",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.aperture_value) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ApertureValue",
                    &self.aperture_value,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "ApertureValue")
                {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.brightness_value) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "BrightnessValue",
                    &self.brightness_value,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "BrightnessValue",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.exposure_bias_value) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ExposureBiasValue",
                    &self.exposure_bias_value,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "ExposureBiasValue",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.max_aperture_value) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "MaxApertureValue",
                    &self.max_aperture_value,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "MaxApertureValue",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.subject_distance) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "SubjectDistance",
                    &self.subject_distance,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "SubjectDistance",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.metering_mode) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "MeteringMode",
                    &self.metering_mode,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "MeteringMode") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.light_source) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "LightSource",
                    &self.light_source,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "LightSource") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.flash) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Flash",
                    &self.flash,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Flash") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.focal_length) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "FocalLength",
                    &self.focal_length,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "FocalLength") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.subject_area) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "SubjectArea",
                    &self.subject_area,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "SubjectArea") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.sub_sec_time) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "SubSecTime",
                    &self.sub_sec_time,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "SubSecTime") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.sub_sec_time_original) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "SubSecTimeOriginal",
                    &self.sub_sec_time_original,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "SubSecTimeOriginal",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.sub_sec_time_digitized) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "SubSecTimeDigitized",
                    &self.sub_sec_time_digitized,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "SubSecTimeDigitized",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.temperature) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Temperature",
                    &self.temperature,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Temperature") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.humidity) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Humidity",
                    &self.humidity,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Humidity") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.pressure) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Pressure",
                    &self.pressure,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Pressure") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.water_depth) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "WaterDepth",
                    &self.water_depth,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "WaterDepth") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.acceleration) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Acceleration",
                    &self.acceleration,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Acceleration") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.camera_elevation_angle) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "CameraElevationAngle",
                    &self.camera_elevation_angle,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "CameraElevationAngle",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.flashpix_version) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "FlashpixVersion",
                    &self.flashpix_version,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "FlashpixVersion",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.color_space) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ColorSpace",
                    &self.color_space,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "ColorSpace") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.pixel_x_dimension) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "PixelXDimension",
                    &self.pixel_x_dimension,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "PixelXDimension",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.pixel_y_dimension) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "PixelYDimension",
                    &self.pixel_y_dimension,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "PixelYDimension",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.related_sound_file) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "RelatedSoundFile",
                    &self.related_sound_file,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "RelatedSoundFile",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.flash_energy) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "FlashEnergy",
                    &self.flash_energy,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "FlashEnergy") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.spatial_frequency_response) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "SpatialFrequencyResponse",
                    &self.spatial_frequency_response,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "SpatialFrequencyResponse",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.contrast) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Contrast",
                    &self.contrast,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Contrast") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.saturation) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Saturation",
                    &self.saturation,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Saturation") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.sharpness) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Sharpness",
                    &self.sharpness,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Sharpness") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.source_exposure_times_of_composite_image) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "SourceExposureTimesOfCompositeImage",
                    &self.source_exposure_times_of_composite_image,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "SourceExposureTimesOfCompositeImage",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.gamma) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "Gamma",
                    &self.gamma,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "Gamma") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.gps_version_id) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "GpsVersionId",
                    &self.gps_version_id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "GpsVersionId") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.gps_dest_distance) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "GpsDestDistance",
                    &self.gps_dest_distance,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "GpsDestDistance",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.gps_processing_method) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "GpsProcessingMethod",
                    &self.gps_processing_method,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "GpsProcessingMethod",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.gps_area_information) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "GpsAreaInformation",
                    &self.gps_area_information,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "GpsAreaInformation",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.gps_date_stamp) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "GpsDateStamp",
                    &self.gps_date_stamp,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "GpsDateStamp") {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.gps_differential) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "GpsDifferential",
                    &self.gps_differential,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "GpsDifferential",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.gps_positioning_error) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "GpsPositioningError",
                    &self.gps_positioning_error,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "GpsPositioningError",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.interoperability_index) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "InteroperabilityIndex",
                    &self.interoperability_index,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "InteroperabilityIndex",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.interoperability_version) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "InteroperabilityVersion",
                    &self.interoperability_version,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "InteroperabilityVersion",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.related_image_file_format) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "RelatedImageFileFormat",
                    &self.related_image_file_format,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "RelatedImageFileFormat",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.related_image_width) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "RelatedImageWidth",
                    &self.related_image_width,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "RelatedImageWidth",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            if !Option::is_none(&self.related_image_length) {
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "RelatedImageLength",
                    &self.related_image_length,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            } else {
                match _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "RelatedImageLength",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            }
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Metadata {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
                __field10,
                __field11,
                __field12,
                __field13,
                __field14,
                __field15,
                __field16,
                __field17,
                __field18,
                __field19,
                __field20,
                __field21,
                __field22,
                __field23,
                __field24,
                __field25,
                __field26,
                __field27,
                __field28,
                __field29,
                __field30,
                __field31,
                __field32,
                __field33,
                __field34,
                __field35,
                __field36,
                __field37,
                __field38,
                __field39,
                __field40,
                __field41,
                __field42,
                __field43,
                __field44,
                __field45,
                __field46,
                __field47,
                __field48,
                __field49,
                __field50,
                __field51,
                __field52,
                __field53,
                __field54,
                __field55,
                __field56,
                __field57,
                __field58,
                __field59,
                __field60,
                __field61,
                __field62,
                __field63,
                __field64,
                __field65,
                __field66,
                __field67,
                __field68,
                __field69,
                __field70,
                __field71,
                __field72,
                __field73,
                __field74,
                __field75,
                __field76,
                __field77,
                __field78,
                __field79,
                __field80,
                __field81,
                __field82,
                __field83,
                __field84,
                __field85,
                __field86,
                __field87,
                __field88,
                __field89,
                __field90,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        5u64 => _serde::__private::Ok(__Field::__field5),
                        6u64 => _serde::__private::Ok(__Field::__field6),
                        7u64 => _serde::__private::Ok(__Field::__field7),
                        8u64 => _serde::__private::Ok(__Field::__field8),
                        9u64 => _serde::__private::Ok(__Field::__field9),
                        10u64 => _serde::__private::Ok(__Field::__field10),
                        11u64 => _serde::__private::Ok(__Field::__field11),
                        12u64 => _serde::__private::Ok(__Field::__field12),
                        13u64 => _serde::__private::Ok(__Field::__field13),
                        14u64 => _serde::__private::Ok(__Field::__field14),
                        15u64 => _serde::__private::Ok(__Field::__field15),
                        16u64 => _serde::__private::Ok(__Field::__field16),
                        17u64 => _serde::__private::Ok(__Field::__field17),
                        18u64 => _serde::__private::Ok(__Field::__field18),
                        19u64 => _serde::__private::Ok(__Field::__field19),
                        20u64 => _serde::__private::Ok(__Field::__field20),
                        21u64 => _serde::__private::Ok(__Field::__field21),
                        22u64 => _serde::__private::Ok(__Field::__field22),
                        23u64 => _serde::__private::Ok(__Field::__field23),
                        24u64 => _serde::__private::Ok(__Field::__field24),
                        25u64 => _serde::__private::Ok(__Field::__field25),
                        26u64 => _serde::__private::Ok(__Field::__field26),
                        27u64 => _serde::__private::Ok(__Field::__field27),
                        28u64 => _serde::__private::Ok(__Field::__field28),
                        29u64 => _serde::__private::Ok(__Field::__field29),
                        30u64 => _serde::__private::Ok(__Field::__field30),
                        31u64 => _serde::__private::Ok(__Field::__field31),
                        32u64 => _serde::__private::Ok(__Field::__field32),
                        33u64 => _serde::__private::Ok(__Field::__field33),
                        34u64 => _serde::__private::Ok(__Field::__field34),
                        35u64 => _serde::__private::Ok(__Field::__field35),
                        36u64 => _serde::__private::Ok(__Field::__field36),
                        37u64 => _serde::__private::Ok(__Field::__field37),
                        38u64 => _serde::__private::Ok(__Field::__field38),
                        39u64 => _serde::__private::Ok(__Field::__field39),
                        40u64 => _serde::__private::Ok(__Field::__field40),
                        41u64 => _serde::__private::Ok(__Field::__field41),
                        42u64 => _serde::__private::Ok(__Field::__field42),
                        43u64 => _serde::__private::Ok(__Field::__field43),
                        44u64 => _serde::__private::Ok(__Field::__field44),
                        45u64 => _serde::__private::Ok(__Field::__field45),
                        46u64 => _serde::__private::Ok(__Field::__field46),
                        47u64 => _serde::__private::Ok(__Field::__field47),
                        48u64 => _serde::__private::Ok(__Field::__field48),
                        49u64 => _serde::__private::Ok(__Field::__field49),
                        50u64 => _serde::__private::Ok(__Field::__field50),
                        51u64 => _serde::__private::Ok(__Field::__field51),
                        52u64 => _serde::__private::Ok(__Field::__field52),
                        53u64 => _serde::__private::Ok(__Field::__field53),
                        54u64 => _serde::__private::Ok(__Field::__field54),
                        55u64 => _serde::__private::Ok(__Field::__field55),
                        56u64 => _serde::__private::Ok(__Field::__field56),
                        57u64 => _serde::__private::Ok(__Field::__field57),
                        58u64 => _serde::__private::Ok(__Field::__field58),
                        59u64 => _serde::__private::Ok(__Field::__field59),
                        60u64 => _serde::__private::Ok(__Field::__field60),
                        61u64 => _serde::__private::Ok(__Field::__field61),
                        62u64 => _serde::__private::Ok(__Field::__field62),
                        63u64 => _serde::__private::Ok(__Field::__field63),
                        64u64 => _serde::__private::Ok(__Field::__field64),
                        65u64 => _serde::__private::Ok(__Field::__field65),
                        66u64 => _serde::__private::Ok(__Field::__field66),
                        67u64 => _serde::__private::Ok(__Field::__field67),
                        68u64 => _serde::__private::Ok(__Field::__field68),
                        69u64 => _serde::__private::Ok(__Field::__field69),
                        70u64 => _serde::__private::Ok(__Field::__field70),
                        71u64 => _serde::__private::Ok(__Field::__field71),
                        72u64 => _serde::__private::Ok(__Field::__field72),
                        73u64 => _serde::__private::Ok(__Field::__field73),
                        74u64 => _serde::__private::Ok(__Field::__field74),
                        75u64 => _serde::__private::Ok(__Field::__field75),
                        76u64 => _serde::__private::Ok(__Field::__field76),
                        77u64 => _serde::__private::Ok(__Field::__field77),
                        78u64 => _serde::__private::Ok(__Field::__field78),
                        79u64 => _serde::__private::Ok(__Field::__field79),
                        80u64 => _serde::__private::Ok(__Field::__field80),
                        81u64 => _serde::__private::Ok(__Field::__field81),
                        82u64 => _serde::__private::Ok(__Field::__field82),
                        83u64 => _serde::__private::Ok(__Field::__field83),
                        84u64 => _serde::__private::Ok(__Field::__field84),
                        85u64 => _serde::__private::Ok(__Field::__field85),
                        86u64 => _serde::__private::Ok(__Field::__field86),
                        87u64 => _serde::__private::Ok(__Field::__field87),
                        88u64 => _serde::__private::Ok(__Field::__field88),
                        89u64 => _serde::__private::Ok(__Field::__field89),
                        90u64 => _serde::__private::Ok(__Field::__field90),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "ImageWidth" => _serde::__private::Ok(__Field::__field0),
                        "ImageLength" => _serde::__private::Ok(__Field::__field1),
                        "BitsPerSample" => _serde::__private::Ok(__Field::__field2),
                        "Compression" => _serde::__private::Ok(__Field::__field3),
                        "PhotometricInterpretation" => _serde::__private::Ok(__Field::__field4),
                        "ImageDescription" => _serde::__private::Ok(__Field::__field5),
                        "Make" => _serde::__private::Ok(__Field::__field6),
                        "Model" => _serde::__private::Ok(__Field::__field7),
                        "StripOffsets" => _serde::__private::Ok(__Field::__field8),
                        "Orientation" => _serde::__private::Ok(__Field::__field9),
                        "SamplesPerPixel" => _serde::__private::Ok(__Field::__field10),
                        "RowsPerStrip" => _serde::__private::Ok(__Field::__field11),
                        "StripByteCounts" => _serde::__private::Ok(__Field::__field12),
                        "XResolution" => _serde::__private::Ok(__Field::__field13),
                        "YResolution" => _serde::__private::Ok(__Field::__field14),
                        "PlanarConfiguration" => _serde::__private::Ok(__Field::__field15),
                        "ResolutionUnit" => _serde::__private::Ok(__Field::__field16),
                        "TransferFunction" => _serde::__private::Ok(__Field::__field17),
                        "Software" => _serde::__private::Ok(__Field::__field18),
                        "DateTime" => _serde::__private::Ok(__Field::__field19),
                        "Artist" => _serde::__private::Ok(__Field::__field20),
                        "WhitePoint" => _serde::__private::Ok(__Field::__field21),
                        "PrimaryChromaticities" => _serde::__private::Ok(__Field::__field22),
                        "TileOffsets" => _serde::__private::Ok(__Field::__field23),
                        "TileByteCounts" => _serde::__private::Ok(__Field::__field24),
                        "ReferenceBlackWhite" => _serde::__private::Ok(__Field::__field25),
                        "Copyright" => _serde::__private::Ok(__Field::__field26),
                        "ExposureTime" => _serde::__private::Ok(__Field::__field27),
                        "FNumber" => _serde::__private::Ok(__Field::__field28),
                        "ExposureProgram" => _serde::__private::Ok(__Field::__field29),
                        "SpectralSensitivity" => _serde::__private::Ok(__Field::__field30),
                        "PhotographicSensitivity" => _serde::__private::Ok(__Field::__field31),
                        "Oecf" => _serde::__private::Ok(__Field::__field32),
                        "SensitivityType" => _serde::__private::Ok(__Field::__field33),
                        "StandardOutputSensitivity" => _serde::__private::Ok(__Field::__field34),
                        "RecommendedExposureIndex" => _serde::__private::Ok(__Field::__field35),
                        "IsoSpeed" => _serde::__private::Ok(__Field::__field36),
                        "IsoSpeedLatitudeyyy" => _serde::__private::Ok(__Field::__field37),
                        "IsoSpeedLatitudezzz" => _serde::__private::Ok(__Field::__field38),
                        "ExifVersion" => _serde::__private::Ok(__Field::__field39),
                        "DateTimeOriginal" => _serde::__private::Ok(__Field::__field40),
                        "DateTimeDigitized" => _serde::__private::Ok(__Field::__field41),
                        "OffsetTime" => _serde::__private::Ok(__Field::__field42),
                        "OffsetTimeOriginal" => _serde::__private::Ok(__Field::__field43),
                        "OffsetTimeDigitized" => _serde::__private::Ok(__Field::__field44),
                        "ComponentsConfiguration" => _serde::__private::Ok(__Field::__field45),
                        "CompressedBitsPerPixel" => _serde::__private::Ok(__Field::__field46),
                        "ShutterSpeedValue" => _serde::__private::Ok(__Field::__field47),
                        "ApertureValue" => _serde::__private::Ok(__Field::__field48),
                        "BrightnessValue" => _serde::__private::Ok(__Field::__field49),
                        "ExposureBiasValue" => _serde::__private::Ok(__Field::__field50),
                        "MaxApertureValue" => _serde::__private::Ok(__Field::__field51),
                        "SubjectDistance" => _serde::__private::Ok(__Field::__field52),
                        "MeteringMode" => _serde::__private::Ok(__Field::__field53),
                        "LightSource" => _serde::__private::Ok(__Field::__field54),
                        "Flash" => _serde::__private::Ok(__Field::__field55),
                        "FocalLength" => _serde::__private::Ok(__Field::__field56),
                        "SubjectArea" => _serde::__private::Ok(__Field::__field57),
                        "SubSecTime" => _serde::__private::Ok(__Field::__field58),
                        "SubSecTimeOriginal" => _serde::__private::Ok(__Field::__field59),
                        "SubSecTimeDigitized" => _serde::__private::Ok(__Field::__field60),
                        "Temperature" => _serde::__private::Ok(__Field::__field61),
                        "Humidity" => _serde::__private::Ok(__Field::__field62),
                        "Pressure" => _serde::__private::Ok(__Field::__field63),
                        "WaterDepth" => _serde::__private::Ok(__Field::__field64),
                        "Acceleration" => _serde::__private::Ok(__Field::__field65),
                        "CameraElevationAngle" => _serde::__private::Ok(__Field::__field66),
                        "FlashpixVersion" => _serde::__private::Ok(__Field::__field67),
                        "ColorSpace" => _serde::__private::Ok(__Field::__field68),
                        "PixelXDimension" => _serde::__private::Ok(__Field::__field69),
                        "PixelYDimension" => _serde::__private::Ok(__Field::__field70),
                        "RelatedSoundFile" => _serde::__private::Ok(__Field::__field71),
                        "FlashEnergy" => _serde::__private::Ok(__Field::__field72),
                        "SpatialFrequencyResponse" => _serde::__private::Ok(__Field::__field73),
                        "Contrast" => _serde::__private::Ok(__Field::__field74),
                        "Saturation" => _serde::__private::Ok(__Field::__field75),
                        "Sharpness" => _serde::__private::Ok(__Field::__field76),
                        "SourceExposureTimesOfCompositeImage" => {
                            _serde::__private::Ok(__Field::__field77)
                        }
                        "Gamma" => _serde::__private::Ok(__Field::__field78),
                        "GpsVersionId" => _serde::__private::Ok(__Field::__field79),
                        "GpsDestDistance" => _serde::__private::Ok(__Field::__field80),
                        "GpsProcessingMethod" => _serde::__private::Ok(__Field::__field81),
                        "GpsAreaInformation" => _serde::__private::Ok(__Field::__field82),
                        "GpsDateStamp" => _serde::__private::Ok(__Field::__field83),
                        "GpsDifferential" => _serde::__private::Ok(__Field::__field84),
                        "GpsPositioningError" => _serde::__private::Ok(__Field::__field85),
                        "InteroperabilityIndex" => _serde::__private::Ok(__Field::__field86),
                        "InteroperabilityVersion" => _serde::__private::Ok(__Field::__field87),
                        "RelatedImageFileFormat" => _serde::__private::Ok(__Field::__field88),
                        "RelatedImageWidth" => _serde::__private::Ok(__Field::__field89),
                        "RelatedImageLength" => _serde::__private::Ok(__Field::__field90),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"ImageWidth" => _serde::__private::Ok(__Field::__field0),
                        b"ImageLength" => _serde::__private::Ok(__Field::__field1),
                        b"BitsPerSample" => _serde::__private::Ok(__Field::__field2),
                        b"Compression" => _serde::__private::Ok(__Field::__field3),
                        b"PhotometricInterpretation" => _serde::__private::Ok(__Field::__field4),
                        b"ImageDescription" => _serde::__private::Ok(__Field::__field5),
                        b"Make" => _serde::__private::Ok(__Field::__field6),
                        b"Model" => _serde::__private::Ok(__Field::__field7),
                        b"StripOffsets" => _serde::__private::Ok(__Field::__field8),
                        b"Orientation" => _serde::__private::Ok(__Field::__field9),
                        b"SamplesPerPixel" => _serde::__private::Ok(__Field::__field10),
                        b"RowsPerStrip" => _serde::__private::Ok(__Field::__field11),
                        b"StripByteCounts" => _serde::__private::Ok(__Field::__field12),
                        b"XResolution" => _serde::__private::Ok(__Field::__field13),
                        b"YResolution" => _serde::__private::Ok(__Field::__field14),
                        b"PlanarConfiguration" => _serde::__private::Ok(__Field::__field15),
                        b"ResolutionUnit" => _serde::__private::Ok(__Field::__field16),
                        b"TransferFunction" => _serde::__private::Ok(__Field::__field17),
                        b"Software" => _serde::__private::Ok(__Field::__field18),
                        b"DateTime" => _serde::__private::Ok(__Field::__field19),
                        b"Artist" => _serde::__private::Ok(__Field::__field20),
                        b"WhitePoint" => _serde::__private::Ok(__Field::__field21),
                        b"PrimaryChromaticities" => _serde::__private::Ok(__Field::__field22),
                        b"TileOffsets" => _serde::__private::Ok(__Field::__field23),
                        b"TileByteCounts" => _serde::__private::Ok(__Field::__field24),
                        b"ReferenceBlackWhite" => _serde::__private::Ok(__Field::__field25),
                        b"Copyright" => _serde::__private::Ok(__Field::__field26),
                        b"ExposureTime" => _serde::__private::Ok(__Field::__field27),
                        b"FNumber" => _serde::__private::Ok(__Field::__field28),
                        b"ExposureProgram" => _serde::__private::Ok(__Field::__field29),
                        b"SpectralSensitivity" => _serde::__private::Ok(__Field::__field30),
                        b"PhotographicSensitivity" => _serde::__private::Ok(__Field::__field31),
                        b"Oecf" => _serde::__private::Ok(__Field::__field32),
                        b"SensitivityType" => _serde::__private::Ok(__Field::__field33),
                        b"StandardOutputSensitivity" => _serde::__private::Ok(__Field::__field34),
                        b"RecommendedExposureIndex" => _serde::__private::Ok(__Field::__field35),
                        b"IsoSpeed" => _serde::__private::Ok(__Field::__field36),
                        b"IsoSpeedLatitudeyyy" => _serde::__private::Ok(__Field::__field37),
                        b"IsoSpeedLatitudezzz" => _serde::__private::Ok(__Field::__field38),
                        b"ExifVersion" => _serde::__private::Ok(__Field::__field39),
                        b"DateTimeOriginal" => _serde::__private::Ok(__Field::__field40),
                        b"DateTimeDigitized" => _serde::__private::Ok(__Field::__field41),
                        b"OffsetTime" => _serde::__private::Ok(__Field::__field42),
                        b"OffsetTimeOriginal" => _serde::__private::Ok(__Field::__field43),
                        b"OffsetTimeDigitized" => _serde::__private::Ok(__Field::__field44),
                        b"ComponentsConfiguration" => _serde::__private::Ok(__Field::__field45),
                        b"CompressedBitsPerPixel" => _serde::__private::Ok(__Field::__field46),
                        b"ShutterSpeedValue" => _serde::__private::Ok(__Field::__field47),
                        b"ApertureValue" => _serde::__private::Ok(__Field::__field48),
                        b"BrightnessValue" => _serde::__private::Ok(__Field::__field49),
                        b"ExposureBiasValue" => _serde::__private::Ok(__Field::__field50),
                        b"MaxApertureValue" => _serde::__private::Ok(__Field::__field51),
                        b"SubjectDistance" => _serde::__private::Ok(__Field::__field52),
                        b"MeteringMode" => _serde::__private::Ok(__Field::__field53),
                        b"LightSource" => _serde::__private::Ok(__Field::__field54),
                        b"Flash" => _serde::__private::Ok(__Field::__field55),
                        b"FocalLength" => _serde::__private::Ok(__Field::__field56),
                        b"SubjectArea" => _serde::__private::Ok(__Field::__field57),
                        b"SubSecTime" => _serde::__private::Ok(__Field::__field58),
                        b"SubSecTimeOriginal" => _serde::__private::Ok(__Field::__field59),
                        b"SubSecTimeDigitized" => _serde::__private::Ok(__Field::__field60),
                        b"Temperature" => _serde::__private::Ok(__Field::__field61),
                        b"Humidity" => _serde::__private::Ok(__Field::__field62),
                        b"Pressure" => _serde::__private::Ok(__Field::__field63),
                        b"WaterDepth" => _serde::__private::Ok(__Field::__field64),
                        b"Acceleration" => _serde::__private::Ok(__Field::__field65),
                        b"CameraElevationAngle" => _serde::__private::Ok(__Field::__field66),
                        b"FlashpixVersion" => _serde::__private::Ok(__Field::__field67),
                        b"ColorSpace" => _serde::__private::Ok(__Field::__field68),
                        b"PixelXDimension" => _serde::__private::Ok(__Field::__field69),
                        b"PixelYDimension" => _serde::__private::Ok(__Field::__field70),
                        b"RelatedSoundFile" => _serde::__private::Ok(__Field::__field71),
                        b"FlashEnergy" => _serde::__private::Ok(__Field::__field72),
                        b"SpatialFrequencyResponse" => _serde::__private::Ok(__Field::__field73),
                        b"Contrast" => _serde::__private::Ok(__Field::__field74),
                        b"Saturation" => _serde::__private::Ok(__Field::__field75),
                        b"Sharpness" => _serde::__private::Ok(__Field::__field76),
                        b"SourceExposureTimesOfCompositeImage" => {
                            _serde::__private::Ok(__Field::__field77)
                        }
                        b"Gamma" => _serde::__private::Ok(__Field::__field78),
                        b"GpsVersionId" => _serde::__private::Ok(__Field::__field79),
                        b"GpsDestDistance" => _serde::__private::Ok(__Field::__field80),
                        b"GpsProcessingMethod" => _serde::__private::Ok(__Field::__field81),
                        b"GpsAreaInformation" => _serde::__private::Ok(__Field::__field82),
                        b"GpsDateStamp" => _serde::__private::Ok(__Field::__field83),
                        b"GpsDifferential" => _serde::__private::Ok(__Field::__field84),
                        b"GpsPositioningError" => _serde::__private::Ok(__Field::__field85),
                        b"InteroperabilityIndex" => _serde::__private::Ok(__Field::__field86),
                        b"InteroperabilityVersion" => _serde::__private::Ok(__Field::__field87),
                        b"RelatedImageFileFormat" => _serde::__private::Ok(__Field::__field88),
                        b"RelatedImageWidth" => _serde::__private::Ok(__Field::__field89),
                        b"RelatedImageLength" => _serde::__private::Ok(__Field::__field90),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Metadata>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Metadata;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Metadata")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field1 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field2 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                2usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field3 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                3usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field4 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                4usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field5 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                5usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field6 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                6usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field7 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                7usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field8 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                8usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field9 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                9usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field10 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                10usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field11 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                11usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field12 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                12usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field13 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                13usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field14 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                14usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field15 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                15usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field16 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                16usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field17 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                17usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field18 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                18usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field19 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                19usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field20 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                20usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field21 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                21usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field22 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                22usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field23 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                23usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field24 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                24usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field25 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                25usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field26 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                26usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field27 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                27usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field28 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                28usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field29 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                29usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field30 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                30usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field31 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                31usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field32 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                32usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field33 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                33usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field34 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                34usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field35 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                35usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field36 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                36usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field37 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                37usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field38 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                38usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field39 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                39usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field40 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                40usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field41 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                41usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field42 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                42usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field43 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                43usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field44 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                44usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field45 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                45usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field46 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                46usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field47 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                47usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field48 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                48usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field49 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                49usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field50 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                50usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field51 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                51usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field52 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                52usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field53 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                53usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field54 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                54usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field55 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                55usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field56 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                56usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field57 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                57usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field58 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                58usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field59 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                59usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field60 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                60usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field61 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                61usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field62 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                62usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field63 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                63usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field64 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                64usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field65 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                65usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field66 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                66usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field67 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                67usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field68 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                68usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field69 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                69usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field70 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                70usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field71 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                71usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field72 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                72usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field73 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                73usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field74 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                74usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field75 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                75usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field76 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                76usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field77 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                77usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field78 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                78usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field79 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                79usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field80 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                80usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field81 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                81usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field82 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                82usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field83 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                83usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field84 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                84usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field85 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                85usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field86 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                86usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field87 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                87usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field88 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                88usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field89 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                89usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    let __field90 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                90usize,
                                &"struct Metadata with 91 elements",
                            ));
                        }
                    };
                    _serde::__private::Ok(Metadata {
                        image_width: __field0,
                        image_length: __field1,
                        bits_per_sample: __field2,
                        compression: __field3,
                        photometric_interpretation: __field4,
                        image_description: __field5,
                        make: __field6,
                        model: __field7,
                        strip_offsets: __field8,
                        orientation: __field9,
                        samples_per_pixel: __field10,
                        rows_per_strip: __field11,
                        strip_byte_counts: __field12,
                        x_resolution: __field13,
                        y_resolution: __field14,
                        planar_configuration: __field15,
                        resolution_unit: __field16,
                        transfer_function: __field17,
                        software: __field18,
                        date_time: __field19,
                        artist: __field20,
                        white_point: __field21,
                        primary_chromaticities: __field22,
                        tile_offsets: __field23,
                        tile_byte_counts: __field24,
                        reference_black_white: __field25,
                        copyright: __field26,
                        exposure_time: __field27,
                        f_number: __field28,
                        exposure_program: __field29,
                        spectral_sensitivity: __field30,
                        photographic_sensitivity: __field31,
                        oecf: __field32,
                        sensitivity_type: __field33,
                        standard_output_sensitivity: __field34,
                        recommended_exposure_index: __field35,
                        iso_speed: __field36,
                        iso_speed_latitudeyyy: __field37,
                        iso_speed_latitudezzz: __field38,
                        exif_version: __field39,
                        date_time_original: __field40,
                        date_time_digitized: __field41,
                        offset_time: __field42,
                        offset_time_original: __field43,
                        offset_time_digitized: __field44,
                        components_configuration: __field45,
                        compressed_bits_per_pixel: __field46,
                        shutter_speed_value: __field47,
                        aperture_value: __field48,
                        brightness_value: __field49,
                        exposure_bias_value: __field50,
                        max_aperture_value: __field51,
                        subject_distance: __field52,
                        metering_mode: __field53,
                        light_source: __field54,
                        flash: __field55,
                        focal_length: __field56,
                        subject_area: __field57,
                        sub_sec_time: __field58,
                        sub_sec_time_original: __field59,
                        sub_sec_time_digitized: __field60,
                        temperature: __field61,
                        humidity: __field62,
                        pressure: __field63,
                        water_depth: __field64,
                        acceleration: __field65,
                        camera_elevation_angle: __field66,
                        flashpix_version: __field67,
                        color_space: __field68,
                        pixel_x_dimension: __field69,
                        pixel_y_dimension: __field70,
                        related_sound_file: __field71,
                        flash_energy: __field72,
                        spatial_frequency_response: __field73,
                        contrast: __field74,
                        saturation: __field75,
                        sharpness: __field76,
                        source_exposure_times_of_composite_image: __field77,
                        gamma: __field78,
                        gps_version_id: __field79,
                        gps_dest_distance: __field80,
                        gps_processing_method: __field81,
                        gps_area_information: __field82,
                        gps_date_stamp: __field83,
                        gps_differential: __field84,
                        gps_positioning_error: __field85,
                        interoperability_index: __field86,
                        interoperability_version: __field87,
                        related_image_file_format: __field88,
                        related_image_width: __field89,
                        related_image_length: __field90,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field1: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field2: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field3: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field4: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field5: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field6: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field7: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field8: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field9: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field10: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field11: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field12: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field13: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field14: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field15: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field16: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field17: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field18: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field19: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field20: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field21: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field22: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field23: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field24: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field25: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field26: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field27: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field28: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field29: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field30: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field31: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field32: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field33: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field34: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field35: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field36: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field37: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field38: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field39: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field40: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field41: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field42: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field43: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field44: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field45: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field46: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field47: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field48: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field49: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field50: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field51: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field52: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field53: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field54: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field55: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field56: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field57: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field58: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field59: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field60: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field61: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field62: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field63: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field64: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field65: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field66: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field67: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field68: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field69: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field70: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field71: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field72: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field73: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field74: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field75: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field76: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field77: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field78: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field79: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field80: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field81: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field82: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field83: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field84: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field85: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field86: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field87: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field88: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field89: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field90: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ImageWidth",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ImageLength",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "BitsPerSample",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Compression",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field4 => {
                                if _serde::__private::Option::is_some(&__field4) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "PhotometricInterpretation",
                                        ),
                                    );
                                }
                                __field4 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field5 => {
                                if _serde::__private::Option::is_some(&__field5) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ImageDescription",
                                        ),
                                    );
                                }
                                __field5 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field6 => {
                                if _serde::__private::Option::is_some(&__field6) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("Make"),
                                    );
                                }
                                __field6 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field7 => {
                                if _serde::__private::Option::is_some(&__field7) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("Model"),
                                    );
                                }
                                __field7 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field8 => {
                                if _serde::__private::Option::is_some(&__field8) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "StripOffsets",
                                        ),
                                    );
                                }
                                __field8 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field9 => {
                                if _serde::__private::Option::is_some(&__field9) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Orientation",
                                        ),
                                    );
                                }
                                __field9 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field10 => {
                                if _serde::__private::Option::is_some(&__field10) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "SamplesPerPixel",
                                        ),
                                    );
                                }
                                __field10 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field11 => {
                                if _serde::__private::Option::is_some(&__field11) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "RowsPerStrip",
                                        ),
                                    );
                                }
                                __field11 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field12 => {
                                if _serde::__private::Option::is_some(&__field12) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "StripByteCounts",
                                        ),
                                    );
                                }
                                __field12 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field13 => {
                                if _serde::__private::Option::is_some(&__field13) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "XResolution",
                                        ),
                                    );
                                }
                                __field13 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field14 => {
                                if _serde::__private::Option::is_some(&__field14) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "YResolution",
                                        ),
                                    );
                                }
                                __field14 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field15 => {
                                if _serde::__private::Option::is_some(&__field15) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "PlanarConfiguration",
                                        ),
                                    );
                                }
                                __field15 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field16 => {
                                if _serde::__private::Option::is_some(&__field16) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ResolutionUnit",
                                        ),
                                    );
                                }
                                __field16 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field17 => {
                                if _serde::__private::Option::is_some(&__field17) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "TransferFunction",
                                        ),
                                    );
                                }
                                __field17 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field18 => {
                                if _serde::__private::Option::is_some(&__field18) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Software",
                                        ),
                                    );
                                }
                                __field18 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field19 => {
                                if _serde::__private::Option::is_some(&__field19) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "DateTime",
                                        ),
                                    );
                                }
                                __field19 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field20 => {
                                if _serde::__private::Option::is_some(&__field20) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Artist",
                                        ),
                                    );
                                }
                                __field20 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field21 => {
                                if _serde::__private::Option::is_some(&__field21) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "WhitePoint",
                                        ),
                                    );
                                }
                                __field21 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field22 => {
                                if _serde::__private::Option::is_some(&__field22) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "PrimaryChromaticities",
                                        ),
                                    );
                                }
                                __field22 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field23 => {
                                if _serde::__private::Option::is_some(&__field23) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "TileOffsets",
                                        ),
                                    );
                                }
                                __field23 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field24 => {
                                if _serde::__private::Option::is_some(&__field24) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "TileByteCounts",
                                        ),
                                    );
                                }
                                __field24 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field25 => {
                                if _serde::__private::Option::is_some(&__field25) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ReferenceBlackWhite",
                                        ),
                                    );
                                }
                                __field25 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field26 => {
                                if _serde::__private::Option::is_some(&__field26) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Copyright",
                                        ),
                                    );
                                }
                                __field26 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field27 => {
                                if _serde::__private::Option::is_some(&__field27) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ExposureTime",
                                        ),
                                    );
                                }
                                __field27 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field28 => {
                                if _serde::__private::Option::is_some(&__field28) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "FNumber",
                                        ),
                                    );
                                }
                                __field28 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field29 => {
                                if _serde::__private::Option::is_some(&__field29) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ExposureProgram",
                                        ),
                                    );
                                }
                                __field29 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field30 => {
                                if _serde::__private::Option::is_some(&__field30) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "SpectralSensitivity",
                                        ),
                                    );
                                }
                                __field30 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field31 => {
                                if _serde::__private::Option::is_some(&__field31) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "PhotographicSensitivity",
                                        ),
                                    );
                                }
                                __field31 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field32 => {
                                if _serde::__private::Option::is_some(&__field32) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("Oecf"),
                                    );
                                }
                                __field32 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field33 => {
                                if _serde::__private::Option::is_some(&__field33) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "SensitivityType",
                                        ),
                                    );
                                }
                                __field33 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field34 => {
                                if _serde::__private::Option::is_some(&__field34) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "StandardOutputSensitivity",
                                        ),
                                    );
                                }
                                __field34 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field35 => {
                                if _serde::__private::Option::is_some(&__field35) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "RecommendedExposureIndex",
                                        ),
                                    );
                                }
                                __field35 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field36 => {
                                if _serde::__private::Option::is_some(&__field36) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "IsoSpeed",
                                        ),
                                    );
                                }
                                __field36 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field37 => {
                                if _serde::__private::Option::is_some(&__field37) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "IsoSpeedLatitudeyyy",
                                        ),
                                    );
                                }
                                __field37 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field38 => {
                                if _serde::__private::Option::is_some(&__field38) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "IsoSpeedLatitudezzz",
                                        ),
                                    );
                                }
                                __field38 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field39 => {
                                if _serde::__private::Option::is_some(&__field39) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ExifVersion",
                                        ),
                                    );
                                }
                                __field39 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field40 => {
                                if _serde::__private::Option::is_some(&__field40) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "DateTimeOriginal",
                                        ),
                                    );
                                }
                                __field40 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field41 => {
                                if _serde::__private::Option::is_some(&__field41) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "DateTimeDigitized",
                                        ),
                                    );
                                }
                                __field41 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field42 => {
                                if _serde::__private::Option::is_some(&__field42) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "OffsetTime",
                                        ),
                                    );
                                }
                                __field42 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field43 => {
                                if _serde::__private::Option::is_some(&__field43) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "OffsetTimeOriginal",
                                        ),
                                    );
                                }
                                __field43 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field44 => {
                                if _serde::__private::Option::is_some(&__field44) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "OffsetTimeDigitized",
                                        ),
                                    );
                                }
                                __field44 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field45 => {
                                if _serde::__private::Option::is_some(&__field45) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ComponentsConfiguration",
                                        ),
                                    );
                                }
                                __field45 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field46 => {
                                if _serde::__private::Option::is_some(&__field46) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "CompressedBitsPerPixel",
                                        ),
                                    );
                                }
                                __field46 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field47 => {
                                if _serde::__private::Option::is_some(&__field47) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ShutterSpeedValue",
                                        ),
                                    );
                                }
                                __field47 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field48 => {
                                if _serde::__private::Option::is_some(&__field48) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ApertureValue",
                                        ),
                                    );
                                }
                                __field48 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field49 => {
                                if _serde::__private::Option::is_some(&__field49) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "BrightnessValue",
                                        ),
                                    );
                                }
                                __field49 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field50 => {
                                if _serde::__private::Option::is_some(&__field50) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ExposureBiasValue",
                                        ),
                                    );
                                }
                                __field50 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field51 => {
                                if _serde::__private::Option::is_some(&__field51) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "MaxApertureValue",
                                        ),
                                    );
                                }
                                __field51 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field52 => {
                                if _serde::__private::Option::is_some(&__field52) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "SubjectDistance",
                                        ),
                                    );
                                }
                                __field52 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field53 => {
                                if _serde::__private::Option::is_some(&__field53) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "MeteringMode",
                                        ),
                                    );
                                }
                                __field53 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field54 => {
                                if _serde::__private::Option::is_some(&__field54) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "LightSource",
                                        ),
                                    );
                                }
                                __field54 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field55 => {
                                if _serde::__private::Option::is_some(&__field55) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("Flash"),
                                    );
                                }
                                __field55 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field56 => {
                                if _serde::__private::Option::is_some(&__field56) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "FocalLength",
                                        ),
                                    );
                                }
                                __field56 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field57 => {
                                if _serde::__private::Option::is_some(&__field57) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "SubjectArea",
                                        ),
                                    );
                                }
                                __field57 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field58 => {
                                if _serde::__private::Option::is_some(&__field58) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "SubSecTime",
                                        ),
                                    );
                                }
                                __field58 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field59 => {
                                if _serde::__private::Option::is_some(&__field59) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "SubSecTimeOriginal",
                                        ),
                                    );
                                }
                                __field59 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field60 => {
                                if _serde::__private::Option::is_some(&__field60) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "SubSecTimeDigitized",
                                        ),
                                    );
                                }
                                __field60 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field61 => {
                                if _serde::__private::Option::is_some(&__field61) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Temperature",
                                        ),
                                    );
                                }
                                __field61 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field62 => {
                                if _serde::__private::Option::is_some(&__field62) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Humidity",
                                        ),
                                    );
                                }
                                __field62 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field63 => {
                                if _serde::__private::Option::is_some(&__field63) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Pressure",
                                        ),
                                    );
                                }
                                __field63 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field64 => {
                                if _serde::__private::Option::is_some(&__field64) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "WaterDepth",
                                        ),
                                    );
                                }
                                __field64 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field65 => {
                                if _serde::__private::Option::is_some(&__field65) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Acceleration",
                                        ),
                                    );
                                }
                                __field65 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field66 => {
                                if _serde::__private::Option::is_some(&__field66) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "CameraElevationAngle",
                                        ),
                                    );
                                }
                                __field66 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field67 => {
                                if _serde::__private::Option::is_some(&__field67) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "FlashpixVersion",
                                        ),
                                    );
                                }
                                __field67 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field68 => {
                                if _serde::__private::Option::is_some(&__field68) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ColorSpace",
                                        ),
                                    );
                                }
                                __field68 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field69 => {
                                if _serde::__private::Option::is_some(&__field69) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "PixelXDimension",
                                        ),
                                    );
                                }
                                __field69 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field70 => {
                                if _serde::__private::Option::is_some(&__field70) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "PixelYDimension",
                                        ),
                                    );
                                }
                                __field70 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field71 => {
                                if _serde::__private::Option::is_some(&__field71) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "RelatedSoundFile",
                                        ),
                                    );
                                }
                                __field71 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field72 => {
                                if _serde::__private::Option::is_some(&__field72) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "FlashEnergy",
                                        ),
                                    );
                                }
                                __field72 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field73 => {
                                if _serde::__private::Option::is_some(&__field73) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "SpatialFrequencyResponse",
                                        ),
                                    );
                                }
                                __field73 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field74 => {
                                if _serde::__private::Option::is_some(&__field74) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Contrast",
                                        ),
                                    );
                                }
                                __field74 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field75 => {
                                if _serde::__private::Option::is_some(&__field75) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Saturation",
                                        ),
                                    );
                                }
                                __field75 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field76 => {
                                if _serde::__private::Option::is_some(&__field76) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Sharpness",
                                        ),
                                    );
                                }
                                __field76 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field77 => {
                                if _serde::__private::Option::is_some(&__field77) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "SourceExposureTimesOfCompositeImage",
                                        ),
                                    );
                                }
                                __field77 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field78 => {
                                if _serde::__private::Option::is_some(&__field78) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("Gamma"),
                                    );
                                }
                                __field78 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field79 => {
                                if _serde::__private::Option::is_some(&__field79) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "GpsVersionId",
                                        ),
                                    );
                                }
                                __field79 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field80 => {
                                if _serde::__private::Option::is_some(&__field80) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "GpsDestDistance",
                                        ),
                                    );
                                }
                                __field80 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field81 => {
                                if _serde::__private::Option::is_some(&__field81) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "GpsProcessingMethod",
                                        ),
                                    );
                                }
                                __field81 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field82 => {
                                if _serde::__private::Option::is_some(&__field82) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "GpsAreaInformation",
                                        ),
                                    );
                                }
                                __field82 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field83 => {
                                if _serde::__private::Option::is_some(&__field83) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "GpsDateStamp",
                                        ),
                                    );
                                }
                                __field83 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field84 => {
                                if _serde::__private::Option::is_some(&__field84) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "GpsDifferential",
                                        ),
                                    );
                                }
                                __field84 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field85 => {
                                if _serde::__private::Option::is_some(&__field85) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "GpsPositioningError",
                                        ),
                                    );
                                }
                                __field85 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field86 => {
                                if _serde::__private::Option::is_some(&__field86) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "InteroperabilityIndex",
                                        ),
                                    );
                                }
                                __field86 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field87 => {
                                if _serde::__private::Option::is_some(&__field87) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "InteroperabilityVersion",
                                        ),
                                    );
                                }
                                __field87 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field88 => {
                                if _serde::__private::Option::is_some(&__field88) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "RelatedImageFileFormat",
                                        ),
                                    );
                                }
                                __field88 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field89 => {
                                if _serde::__private::Option::is_some(&__field89) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "RelatedImageWidth",
                                        ),
                                    );
                                }
                                __field89 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field90 => {
                                if _serde::__private::Option::is_some(&__field90) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "RelatedImageLength",
                                        ),
                                    );
                                }
                                __field90 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("ImageWidth") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("ImageLength") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("BitsPerSample") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Compression") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field4 = match __field4 {
                        _serde::__private::Some(__field4) => __field4,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("PhotometricInterpretation")
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field5 = match __field5 {
                        _serde::__private::Some(__field5) => __field5,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("ImageDescription") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field6 = match __field6 {
                        _serde::__private::Some(__field6) => __field6,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Make") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field7 = match __field7 {
                        _serde::__private::Some(__field7) => __field7,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Model") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field8 = match __field8 {
                        _serde::__private::Some(__field8) => __field8,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("StripOffsets") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field9 = match __field9 {
                        _serde::__private::Some(__field9) => __field9,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Orientation") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field10 = match __field10 {
                        _serde::__private::Some(__field10) => __field10,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("SamplesPerPixel") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field11 = match __field11 {
                        _serde::__private::Some(__field11) => __field11,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("RowsPerStrip") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field12 = match __field12 {
                        _serde::__private::Some(__field12) => __field12,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("StripByteCounts") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field13 = match __field13 {
                        _serde::__private::Some(__field13) => __field13,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("XResolution") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field14 = match __field14 {
                        _serde::__private::Some(__field14) => __field14,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("YResolution") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field15 = match __field15 {
                        _serde::__private::Some(__field15) => __field15,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("PlanarConfiguration") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field16 = match __field16 {
                        _serde::__private::Some(__field16) => __field16,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("ResolutionUnit") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field17 = match __field17 {
                        _serde::__private::Some(__field17) => __field17,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("TransferFunction") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field18 = match __field18 {
                        _serde::__private::Some(__field18) => __field18,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Software") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field19 = match __field19 {
                        _serde::__private::Some(__field19) => __field19,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("DateTime") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field20 = match __field20 {
                        _serde::__private::Some(__field20) => __field20,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Artist") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field21 = match __field21 {
                        _serde::__private::Some(__field21) => __field21,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("WhitePoint") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field22 = match __field22 {
                        _serde::__private::Some(__field22) => __field22,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("PrimaryChromaticities") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field23 = match __field23 {
                        _serde::__private::Some(__field23) => __field23,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("TileOffsets") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field24 = match __field24 {
                        _serde::__private::Some(__field24) => __field24,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("TileByteCounts") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field25 = match __field25 {
                        _serde::__private::Some(__field25) => __field25,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("ReferenceBlackWhite") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field26 = match __field26 {
                        _serde::__private::Some(__field26) => __field26,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Copyright") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field27 = match __field27 {
                        _serde::__private::Some(__field27) => __field27,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("ExposureTime") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field28 = match __field28 {
                        _serde::__private::Some(__field28) => __field28,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("FNumber") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field29 = match __field29 {
                        _serde::__private::Some(__field29) => __field29,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("ExposureProgram") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field30 = match __field30 {
                        _serde::__private::Some(__field30) => __field30,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("SpectralSensitivity") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field31 = match __field31 {
                        _serde::__private::Some(__field31) => __field31,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("PhotographicSensitivity") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field32 = match __field32 {
                        _serde::__private::Some(__field32) => __field32,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Oecf") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field33 = match __field33 {
                        _serde::__private::Some(__field33) => __field33,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("SensitivityType") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field34 = match __field34 {
                        _serde::__private::Some(__field34) => __field34,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("StandardOutputSensitivity")
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field35 = match __field35 {
                        _serde::__private::Some(__field35) => __field35,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("RecommendedExposureIndex") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field36 = match __field36 {
                        _serde::__private::Some(__field36) => __field36,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("IsoSpeed") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field37 = match __field37 {
                        _serde::__private::Some(__field37) => __field37,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("IsoSpeedLatitudeyyy") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field38 = match __field38 {
                        _serde::__private::Some(__field38) => __field38,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("IsoSpeedLatitudezzz") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field39 = match __field39 {
                        _serde::__private::Some(__field39) => __field39,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("ExifVersion") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field40 = match __field40 {
                        _serde::__private::Some(__field40) => __field40,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("DateTimeOriginal") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field41 = match __field41 {
                        _serde::__private::Some(__field41) => __field41,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("DateTimeDigitized") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field42 = match __field42 {
                        _serde::__private::Some(__field42) => __field42,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("OffsetTime") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field43 = match __field43 {
                        _serde::__private::Some(__field43) => __field43,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("OffsetTimeOriginal") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field44 = match __field44 {
                        _serde::__private::Some(__field44) => __field44,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("OffsetTimeDigitized") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field45 = match __field45 {
                        _serde::__private::Some(__field45) => __field45,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("ComponentsConfiguration") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field46 = match __field46 {
                        _serde::__private::Some(__field46) => __field46,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("CompressedBitsPerPixel") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field47 = match __field47 {
                        _serde::__private::Some(__field47) => __field47,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("ShutterSpeedValue") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field48 = match __field48 {
                        _serde::__private::Some(__field48) => __field48,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("ApertureValue") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field49 = match __field49 {
                        _serde::__private::Some(__field49) => __field49,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("BrightnessValue") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field50 = match __field50 {
                        _serde::__private::Some(__field50) => __field50,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("ExposureBiasValue") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field51 = match __field51 {
                        _serde::__private::Some(__field51) => __field51,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("MaxApertureValue") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field52 = match __field52 {
                        _serde::__private::Some(__field52) => __field52,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("SubjectDistance") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field53 = match __field53 {
                        _serde::__private::Some(__field53) => __field53,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("MeteringMode") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field54 = match __field54 {
                        _serde::__private::Some(__field54) => __field54,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("LightSource") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field55 = match __field55 {
                        _serde::__private::Some(__field55) => __field55,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Flash") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field56 = match __field56 {
                        _serde::__private::Some(__field56) => __field56,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("FocalLength") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field57 = match __field57 {
                        _serde::__private::Some(__field57) => __field57,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("SubjectArea") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field58 = match __field58 {
                        _serde::__private::Some(__field58) => __field58,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("SubSecTime") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field59 = match __field59 {
                        _serde::__private::Some(__field59) => __field59,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("SubSecTimeOriginal") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field60 = match __field60 {
                        _serde::__private::Some(__field60) => __field60,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("SubSecTimeDigitized") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field61 = match __field61 {
                        _serde::__private::Some(__field61) => __field61,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Temperature") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field62 = match __field62 {
                        _serde::__private::Some(__field62) => __field62,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Humidity") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field63 = match __field63 {
                        _serde::__private::Some(__field63) => __field63,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Pressure") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field64 = match __field64 {
                        _serde::__private::Some(__field64) => __field64,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("WaterDepth") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field65 = match __field65 {
                        _serde::__private::Some(__field65) => __field65,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Acceleration") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field66 = match __field66 {
                        _serde::__private::Some(__field66) => __field66,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("CameraElevationAngle") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field67 = match __field67 {
                        _serde::__private::Some(__field67) => __field67,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("FlashpixVersion") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field68 = match __field68 {
                        _serde::__private::Some(__field68) => __field68,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("ColorSpace") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field69 = match __field69 {
                        _serde::__private::Some(__field69) => __field69,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("PixelXDimension") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field70 = match __field70 {
                        _serde::__private::Some(__field70) => __field70,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("PixelYDimension") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field71 = match __field71 {
                        _serde::__private::Some(__field71) => __field71,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("RelatedSoundFile") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field72 = match __field72 {
                        _serde::__private::Some(__field72) => __field72,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("FlashEnergy") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field73 = match __field73 {
                        _serde::__private::Some(__field73) => __field73,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("SpatialFrequencyResponse") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field74 = match __field74 {
                        _serde::__private::Some(__field74) => __field74,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Contrast") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field75 = match __field75 {
                        _serde::__private::Some(__field75) => __field75,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Saturation") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field76 = match __field76 {
                        _serde::__private::Some(__field76) => __field76,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Sharpness") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field77 = match __field77 {
                        _serde::__private::Some(__field77) => __field77,
                        _serde::__private::None => match _serde::__private::de::missing_field(
                            "SourceExposureTimesOfCompositeImage",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    };
                    let __field78 = match __field78 {
                        _serde::__private::Some(__field78) => __field78,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("Gamma") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field79 = match __field79 {
                        _serde::__private::Some(__field79) => __field79,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("GpsVersionId") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field80 = match __field80 {
                        _serde::__private::Some(__field80) => __field80,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("GpsDestDistance") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field81 = match __field81 {
                        _serde::__private::Some(__field81) => __field81,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("GpsProcessingMethod") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field82 = match __field82 {
                        _serde::__private::Some(__field82) => __field82,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("GpsAreaInformation") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field83 = match __field83 {
                        _serde::__private::Some(__field83) => __field83,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("GpsDateStamp") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field84 = match __field84 {
                        _serde::__private::Some(__field84) => __field84,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("GpsDifferential") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field85 = match __field85 {
                        _serde::__private::Some(__field85) => __field85,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("GpsPositioningError") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field86 = match __field86 {
                        _serde::__private::Some(__field86) => __field86,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("InteroperabilityIndex") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field87 = match __field87 {
                        _serde::__private::Some(__field87) => __field87,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("InteroperabilityVersion") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field88 = match __field88 {
                        _serde::__private::Some(__field88) => __field88,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("RelatedImageFileFormat") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field89 = match __field89 {
                        _serde::__private::Some(__field89) => __field89,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("RelatedImageWidth") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field90 = match __field90 {
                        _serde::__private::Some(__field90) => __field90,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("RelatedImageLength") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(Metadata {
                        image_width: __field0,
                        image_length: __field1,
                        bits_per_sample: __field2,
                        compression: __field3,
                        photometric_interpretation: __field4,
                        image_description: __field5,
                        make: __field6,
                        model: __field7,
                        strip_offsets: __field8,
                        orientation: __field9,
                        samples_per_pixel: __field10,
                        rows_per_strip: __field11,
                        strip_byte_counts: __field12,
                        x_resolution: __field13,
                        y_resolution: __field14,
                        planar_configuration: __field15,
                        resolution_unit: __field16,
                        transfer_function: __field17,
                        software: __field18,
                        date_time: __field19,
                        artist: __field20,
                        white_point: __field21,
                        primary_chromaticities: __field22,
                        tile_offsets: __field23,
                        tile_byte_counts: __field24,
                        reference_black_white: __field25,
                        copyright: __field26,
                        exposure_time: __field27,
                        f_number: __field28,
                        exposure_program: __field29,
                        spectral_sensitivity: __field30,
                        photographic_sensitivity: __field31,
                        oecf: __field32,
                        sensitivity_type: __field33,
                        standard_output_sensitivity: __field34,
                        recommended_exposure_index: __field35,
                        iso_speed: __field36,
                        iso_speed_latitudeyyy: __field37,
                        iso_speed_latitudezzz: __field38,
                        exif_version: __field39,
                        date_time_original: __field40,
                        date_time_digitized: __field41,
                        offset_time: __field42,
                        offset_time_original: __field43,
                        offset_time_digitized: __field44,
                        components_configuration: __field45,
                        compressed_bits_per_pixel: __field46,
                        shutter_speed_value: __field47,
                        aperture_value: __field48,
                        brightness_value: __field49,
                        exposure_bias_value: __field50,
                        max_aperture_value: __field51,
                        subject_distance: __field52,
                        metering_mode: __field53,
                        light_source: __field54,
                        flash: __field55,
                        focal_length: __field56,
                        subject_area: __field57,
                        sub_sec_time: __field58,
                        sub_sec_time_original: __field59,
                        sub_sec_time_digitized: __field60,
                        temperature: __field61,
                        humidity: __field62,
                        pressure: __field63,
                        water_depth: __field64,
                        acceleration: __field65,
                        camera_elevation_angle: __field66,
                        flashpix_version: __field67,
                        color_space: __field68,
                        pixel_x_dimension: __field69,
                        pixel_y_dimension: __field70,
                        related_sound_file: __field71,
                        flash_energy: __field72,
                        spatial_frequency_response: __field73,
                        contrast: __field74,
                        saturation: __field75,
                        sharpness: __field76,
                        source_exposure_times_of_composite_image: __field77,
                        gamma: __field78,
                        gps_version_id: __field79,
                        gps_dest_distance: __field80,
                        gps_processing_method: __field81,
                        gps_area_information: __field82,
                        gps_date_stamp: __field83,
                        gps_differential: __field84,
                        gps_positioning_error: __field85,
                        interoperability_index: __field86,
                        interoperability_version: __field87,
                        related_image_file_format: __field88,
                        related_image_width: __field89,
                        related_image_length: __field90,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &[
                "ImageWidth",
                "ImageLength",
                "BitsPerSample",
                "Compression",
                "PhotometricInterpretation",
                "ImageDescription",
                "Make",
                "Model",
                "StripOffsets",
                "Orientation",
                "SamplesPerPixel",
                "RowsPerStrip",
                "StripByteCounts",
                "XResolution",
                "YResolution",
                "PlanarConfiguration",
                "ResolutionUnit",
                "TransferFunction",
                "Software",
                "DateTime",
                "Artist",
                "WhitePoint",
                "PrimaryChromaticities",
                "TileOffsets",
                "TileByteCounts",
                "ReferenceBlackWhite",
                "Copyright",
                "ExposureTime",
                "FNumber",
                "ExposureProgram",
                "SpectralSensitivity",
                "PhotographicSensitivity",
                "Oecf",
                "SensitivityType",
                "StandardOutputSensitivity",
                "RecommendedExposureIndex",
                "IsoSpeed",
                "IsoSpeedLatitudeyyy",
                "IsoSpeedLatitudezzz",
                "ExifVersion",
                "DateTimeOriginal",
                "DateTimeDigitized",
                "OffsetTime",
                "OffsetTimeOriginal",
                "OffsetTimeDigitized",
                "ComponentsConfiguration",
                "CompressedBitsPerPixel",
                "ShutterSpeedValue",
                "ApertureValue",
                "BrightnessValue",
                "ExposureBiasValue",
                "MaxApertureValue",
                "SubjectDistance",
                "MeteringMode",
                "LightSource",
                "Flash",
                "FocalLength",
                "SubjectArea",
                "SubSecTime",
                "SubSecTimeOriginal",
                "SubSecTimeDigitized",
                "Temperature",
                "Humidity",
                "Pressure",
                "WaterDepth",
                "Acceleration",
                "CameraElevationAngle",
                "FlashpixVersion",
                "ColorSpace",
                "PixelXDimension",
                "PixelYDimension",
                "RelatedSoundFile",
                "FlashEnergy",
                "SpatialFrequencyResponse",
                "Contrast",
                "Saturation",
                "Sharpness",
                "SourceExposureTimesOfCompositeImage",
                "Gamma",
                "GpsVersionId",
                "GpsDestDistance",
                "GpsProcessingMethod",
                "GpsAreaInformation",
                "GpsDateStamp",
                "GpsDifferential",
                "GpsPositioningError",
                "InteroperabilityIndex",
                "InteroperabilityVersion",
                "RelatedImageFileFormat",
                "RelatedImageWidth",
                "RelatedImageLength",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Metadata",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Metadata>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(std::fmt::Error::custom)?;
        {
            let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&json)],
            ));
            result
        }
    }
}
struct ImageData {
    id: String,
    name: String,
    metadata: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for ImageData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "ImageData",
            "id",
            &&self.id,
            "name",
            &&self.name,
            "metadata",
            &&self.metadata,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for ImageData {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "ImageData",
                false as usize + 1 + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "id", &self.id)
            {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "name",
                &self.name,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "metadata",
                &self.metadata,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ImageData {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "id" => _serde::__private::Ok(__Field::__field0),
                        "name" => _serde::__private::Ok(__Field::__field1),
                        "metadata" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => _serde::__private::Ok(__Field::__field0),
                        b"name" => _serde::__private::Ok(__Field::__field1),
                        b"metadata" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<ImageData>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ImageData;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct ImageData")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct ImageData with 3 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct ImageData with 3 elements",
                                ));
                            }
                        };
                    let __field2 =
                        match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct ImageData with 3 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(ImageData {
                        id: __field0,
                        name: __field1,
                        metadata: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<String>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<String>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "metadata",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<String>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => match _serde::__private::de::missing_field("id")
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("name") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("metadata") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(ImageData {
                        id: __field0,
                        name: __field1,
                        metadata: __field2,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["id", "name", "metadata"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "ImageData",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ImageData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
fn main() -> Result<(), reqwest::Error> {
    let body = async {
        let args: Vec<String> = std::env::args().collect();
        if args.len() != 2 {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["Usage: cargo run <image_path>"],
                &[],
            ));
        }
        let filepath = &args[1].split("/").collect::<Vec<&str>>();
        let filename = &filepath[filepath.len() - 1];
        let img_buff = image::open(&args[1]).unwrap();
        let img_dimension = img_buff.dimensions();
        let img_base64 = base64::encode(&img_buff.into_bytes());
        let req_metadata: serde_json::Value = reqwest::Client::new()
            .post("http://localhost:8000/upload")
            .json(&::serde_json::Value::Object({
                let mut object = ::serde_json::Map::new();
                let _ = object.insert(("name").into(), ::serde_json::to_value(&filename).unwrap());
                let _ = object.insert(
                    ("width").into(),
                    ::serde_json::to_value(&img_dimension.0).unwrap(),
                );
                let _ = object.insert(
                    ("height").into(),
                    ::serde_json::to_value(&img_dimension.1).unwrap(),
                );
                let _ = object.insert(
                    ("data").into(),
                    ::serde_json::to_value(&img_base64).unwrap(),
                );
                object
            }))
            .send()
            .await?
            .json()
            .await?;
        let data: ImageData = serde_json::from_value(req_metadata).unwrap();
        let meta: Metadata = serde_json::from_str(data.metadata.as_str()).unwrap();
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["\n\nid: ", ",\nname: ", ", \nmetadata: ", "\n"],
                &[
                    ::core::fmt::ArgumentV1::new_display(&data.id),
                    ::core::fmt::ArgumentV1::new_display(&data.name),
                    ::core::fmt::ArgumentV1::new_display(&meta),
                ],
            ));
        };
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
