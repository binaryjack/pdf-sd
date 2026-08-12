use lopdf::Document;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSlot {
    pub object_id: (u32, u16), // The PDF object ID where this data lives
    pub operation_index: usize, // The index of the operation in the content stream
    pub content: String, // The extracted UTF-8 text
    #[serde(default)]
    pub is_tagged: bool, // True if the text is wrapped in a BDC CodernicSlot tag
}

pub struct PdfStructure {
    pub document: Document,
}
