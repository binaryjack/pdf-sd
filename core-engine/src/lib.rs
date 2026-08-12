use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use lopdf::Document;

pub mod pipeline;
pub mod deserializer;
pub mod serializer;
pub mod stitcher;
pub mod strategy;
pub mod strategies;

use pipeline::{DataSlot, PdfStructure};
use deserializer::PdfDeserializer;
use serializer::PdfSerializer;

#[no_mangle]
pub extern "C" fn extract_pdf_structure(pdf_path_ptr: *const c_char, struct_out_path_ptr: *const c_char) -> *mut c_char {
    if pdf_path_ptr.is_null() || struct_out_path_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let pdf_path = unsafe { CStr::from_ptr(pdf_path_ptr) }.to_string_lossy().into_owned();
    let struct_out_path = unsafe { CStr::from_ptr(struct_out_path_ptr) }.to_string_lossy().into_owned();

    match extract_internal(&pdf_path, &struct_out_path) {
        Ok(json_str) => match CString::new(json_str) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => std::ptr::null_mut(),
        },
        Err(e) => {
            eprintln!("Error extracting PDF: {}", e);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn inject_pdf_data(
    struct_path_ptr: *const c_char,
    json_data_ptr: *const c_char,
    output_path_ptr: *const c_char,
) -> bool {
    if struct_path_ptr.is_null() || json_data_ptr.is_null() || output_path_ptr.is_null() {
        return false;
    }

    let struct_path = unsafe { CStr::from_ptr(struct_path_ptr) }.to_string_lossy().into_owned();
    let json_data = unsafe { CStr::from_ptr(json_data_ptr) }.to_string_lossy().into_owned();
    let output_path = unsafe { CStr::from_ptr(output_path_ptr) }.to_string_lossy().into_owned();

    match inject_internal(&struct_path, &json_data, &output_path) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error injecting PDF: {}", e);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn free_rust_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}

fn extract_internal(pdf_path: &str, struct_out_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pdf_bytes = std::fs::read(pdf_path)?;
    let (mut structure, data_slots) = PdfDeserializer::deserialize(&pdf_bytes)?;

    let mut struct_buf = Vec::new();
    structure.document.save_to(&mut struct_buf)?;
    std::fs::write(struct_out_path, struct_buf)?;

    Ok(serde_json::to_string(&data_slots)?)
}

fn inject_internal(
    struct_path: &str,
    json_data: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_slots: Vec<DataSlot> = serde_json::from_str(json_data)?;
    
    let struct_bytes = std::fs::read(struct_path)?;
    let document = Document::load_mem(&struct_bytes)?;
    let structure = PdfStructure { document };
    
    let new_pdf_bytes = PdfSerializer::serialize(structure, data_slots)?;
    std::fs::write(output_path, new_pdf_bytes)?;
    
    Ok(())
}
