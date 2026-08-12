use anyhow::Result;
use lopdf::{Object, content::Content};
use crate::pipeline::{DataSlot, PdfStructure};

pub struct PdfSerializer;

impl PdfSerializer {
    pub fn serialize(mut structure: PdfStructure, data_slots: Vec<DataSlot>) -> Result<Vec<u8>> {
        // Group data slots by object ID
        let mut slots_by_object: std::collections::HashMap<(u32, u16), Vec<DataSlot>> = std::collections::HashMap::new();
        for slot in data_slots {
            slots_by_object.entry(slot.object_id).or_default().push(slot);
        }

        // Apply changes to the structure
        for (object_id, mut slots) in slots_by_object {
            if let Ok(Object::Stream(stream)) = structure.document.get_object_mut(object_id) {
                let content_bytes = match stream.decompressed_content() {
                    Ok(bytes) => bytes,
                    Err(_) => stream.content.clone(),
                };
                
                if let Ok(mut content) = Content::decode(&content_bytes) {
                    // Sort slots in reverse operation_index order to prevent index shifting when inserting operations
                    slots.sort_by(|a, b| b.operation_index.cmp(&a.operation_index));

                    // Apply all slot modifications to this content object
                    for slot in slots {
                        let mut is_modified = false;
                        if let Some(op) = content.operations.get_mut(slot.operation_index) {
                            if op.operator == "Tj" {
                                if let Some(operand) = op.operands.first_mut() {
                                    if let Object::String(ref old_bytes, _) = operand {
                                        let old_str = String::from_utf8_lossy(old_bytes);
                                        if old_str != slot.content {
                                            is_modified = true;
                                            *operand = Object::String(slot.content.into_bytes(), lopdf::StringFormat::Literal);
                                        }
                                    }
                                }
                            } else if op.operator == "TJ" {
                                if let Some(Object::Array(arr)) = op.operands.first_mut() {
                                    // Verify if modified (simplified check)
                                    is_modified = true; // Assume modified for TJ since we flatten it anyway
                                    // For TJ, we simplify by replacing the whole array with a single string
                                    // The original kerning adjustments are lost, but the text is preserved
                                    arr.clear();
                                    arr.push(Object::String(slot.content.into_bytes(), lopdf::StringFormat::Literal));
                                }
                            }
                        }

                        if is_modified {
                            // Inject EMC after Tj
                            content.operations.insert(slot.operation_index + 1, lopdf::content::Operation::new("EMC", vec![]));
                            
                            // Inject BDC before Tj with /CodernicSlot true
                            let mut dict = lopdf::Dictionary::new();
                            dict.set("CodernicSlot", Object::Boolean(true));
                            content.operations.insert(slot.operation_index, lopdf::content::Operation::new("BDC", vec![
                                Object::Name(b"Span".to_vec()),
                                Object::Dictionary(dict)
                            ]));
                        }
                    }
                    // Re-encode content back to the stream
                    let new_bytes = content.encode()?;
                    stream.set_content(new_bytes);
                    // Force re-compression when saving
                    let _ = stream.compress();
                }
            }
        }
        
        let mut buffer = Vec::new();
        structure.document.save_to(&mut buffer)?;
        Ok(buffer)
    }
}
