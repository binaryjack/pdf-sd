use crate::pipeline::DataSlot;
use crate::stitcher::{StitchedGroup, StitchingStrategy};

pub struct HeuristicStitcher;

impl StitchingStrategy for HeuristicStitcher {
    fn stitch(&self, slots: Vec<DataSlot>) -> Vec<StitchedGroup> {
        let mut groups = Vec::new();
        let mut current_group: Option<StitchedGroup> = None;
        let mut group_id_counter = 0;

        for slot in slots {
            match current_group.take() {
                Some(mut group) => {
                    if group.slots.last().map(|s| s.object_id) == Some(slot.object_id) {
                        // Same object, append to current group
                        group.full_text.push_str(&slot.content);
                        group.slots.push(slot);
                        current_group = Some(group);
                    } else {
                        // Different object, finish current and start new
                        groups.push(group);
                        group_id_counter += 1;
                        current_group = Some(StitchedGroup {
                            group_id: group_id_counter,
                            full_text: slot.content.clone(),
                            slots: vec![slot],
                        });
                    }
                }
                None => {
                    // First slot
                    current_group = Some(StitchedGroup {
                        group_id: group_id_counter,
                        full_text: slot.content.clone(),
                        slots: vec![slot],
                    });
                }
            }
        }

        if let Some(group) = current_group {
            groups.push(group);
        }

        groups
    }

    fn unstitch(&self, groups: Vec<StitchedGroup>) -> Vec<DataSlot> {
        let mut final_slots = Vec::new();

        for group in groups {
            let mut char_iter = group.full_text.chars();

            for mut slot in group.slots {
                let slot_len = slot.content.chars().count();
                let mut new_slot_content = String::new();

                for _ in 0..slot_len {
                    if let Some(c) = char_iter.next() {
                        new_slot_content.push(c);
                    } else {
                        break;
                    }
                }

                // If the user replaced it with something shorter, we pad it with spaces,
                // though ISO replacement guarantees exact length.
                while new_slot_content.chars().count() < slot_len {
                    new_slot_content.push(' ');
                }

                slot.content = new_slot_content;
                final_slots.push(slot);
            }
        }

        final_slots
    }
}
