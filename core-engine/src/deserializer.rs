use anyhow::Result;
use lopdf::{Document, Object, content::Content};
use crate::pipeline::{DataSlot, PdfStructure};

pub struct PdfDeserializer;

impl PdfDeserializer {
    pub fn deserialize(bytes: &[u8]) -> Result<(PdfStructure, Vec<DataSlot>)> {
        let doc = Document::load_mem(bytes)?;
        let mut data_slots = Vec::new();
        
        for (object_id, object) in doc.objects.iter() {
            if let Ok(stream) = object.as_stream() {
                let content_bytes = match stream.decompressed_content() {
                    Ok(bytes) => bytes,
                    Err(_) => stream.content.clone(),
                };
                
                if let Ok(content) = Content::decode(&content_bytes) {
                    let mut is_in_codernic_slot = false;
                    for (idx, op) in content.operations.iter().enumerate() {
                        if op.operator == "BDC" {
                            if let Some(Object::Dictionary(dict)) = op.operands.get(1) {
                                if dict.has(b"CodernicSlot") {
                                    is_in_codernic_slot = true;
                                }
                            }
                        } else if op.operator == "EMC" {
                            is_in_codernic_slot = false;
                        } else if op.operator == "Tj" {
                            if let Some(Object::String(bytes, _)) = op.operands.first() {
                                let text = String::from_utf8_lossy(bytes).into_owned();
                                data_slots.push(DataSlot {
                                    object_id: *object_id,
                                    operation_index: idx,
                                    content: text,
                                    is_tagged: is_in_codernic_slot,
                                });
                            }
                        } else if op.operator == "TJ" {
                            if let Some(Object::Array(arr)) = op.operands.first() {
                                let mut combined_text = String::new();
                                for item in arr {
                                    if let Object::String(bytes, _) = item {
                                        combined_text.push_str(&String::from_utf8_lossy(bytes));
                                    }
                                }
                                data_slots.push(DataSlot {
                                    object_id: *object_id,
                                    operation_index: idx,
                                    content: combined_text,
                                    is_tagged: is_in_codernic_slot,
                                });
                            }
                        }
                    }
                }
            }
        }
        
        Ok((PdfStructure { document: doc }, data_slots))
    }
}
