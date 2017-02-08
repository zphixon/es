
use std::collections::BTreeMap;

use buffer::*;
use EsError;

#[derive(Debug)]
pub struct EsEditor {
    buffers: EsBufferTree,
    current_buffer: u64
}

impl EsEditor {
    pub fn new() -> EsEditor {
        EsEditor {
            buffers: EsBufferTree::new(),
            current_buffer: 0
        }
    }

}

