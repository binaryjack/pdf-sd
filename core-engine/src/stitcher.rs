use crate::pipeline::DataSlot;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StitchedGroup {
    pub group_id: usize,
    pub full_text: String,
    pub slots: Vec<DataSlot>,
}

pub trait StitchingStrategy {
    /// Takes a list of raw DataSlots and groups them logically into StitchedGroups.
    fn stitch(&self, slots: Vec<DataSlot>) -> Vec<StitchedGroup>;
    
    /// Takes modified StitchedGroups (with `full_text` replaced) and redistributes
    /// the new text proportionally across the original child `DataSlot`s.
    fn unstitch(&self, groups: Vec<StitchedGroup>) -> Vec<DataSlot>;
}
