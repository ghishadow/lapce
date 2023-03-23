use std::path::PathBuf;

use floem::peniko::kurbo::Vec2;
use lapce_core::buffer::Buffer;
use lsp_types::Position;

#[derive(Clone, Debug, PartialEq)]
pub struct EditorLocation {
    pub path: PathBuf,
    pub position: Option<EditorPosition>,
    pub scroll_offset: Option<Vec2>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditorPosition {
    Line(usize),
    Position(Position),
    Offset(usize),
}

impl EditorPosition {
    pub fn to_offset(&self, buffer: &Buffer) -> usize {
        match self {
            EditorPosition::Line(n) => {
                buffer.first_non_blank_character_on_line(n.saturating_sub(1))
            }
            EditorPosition::Position(position) => {
                buffer.offset_of_position(position)
            }
            EditorPosition::Offset(offset) => *offset,
        }
    }
}
