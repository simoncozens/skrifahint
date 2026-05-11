use skrifa::outline::autohint::{SCRIPT_CLASSES, STYLE_CLASSES};

use crate::AutohintError;

pub(crate) const STYLE_COUNT: usize = STYLE_CLASSES.len();
pub(crate) const STYLE_INDEX_UNASSIGNED: u16 = u16::MAX;

/// A Skrifa-native style index (0..STYLE_COUNT).
///
/// Used as the key when mapping from style to per-style data.  Distinct from
/// a *slot index* (the compact running number 0..num_used_styles assigned to
/// styles that have usable CVT metrics).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StyleIndex(usize);

impl StyleIndex {
    pub fn new(index: usize) -> Result<Self, AutohintError> {
        if index < STYLE_COUNT {
            Ok(StyleIndex(index))
        } else {
            Err(AutohintError::InvalidArgument(format!(
                "style index {index} is out of bounds (max {})",
                STYLE_COUNT - 1
            )))
        }
    }

    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Check if a script is oriented top-to-bottom (e.g., Mongolian, Gothic).
    /// Returns false for out-of-bounds style indices.
    pub(crate) fn script_hints_top_to_bottom(self) -> bool {
        if let Some(style) = STYLE_CLASSES.get(self.0) {
            style.script.hint_top_to_bottom
        } else {
            false
        }
    }
}

impl From<usize> for StyleIndex {
    fn from(v: usize) -> Self {
        StyleIndex(v)
    }
}

impl From<u16> for StyleIndex {
    fn from(v: u16) -> Self {
        StyleIndex(v as usize)
    }
}

/// Return the default Skrifa style for a script index.
///
/// Returns `None` if no default style exists for the given script index.
pub(crate) fn default_style_for_script(script_index: usize) -> Option<usize> {
    let script = SCRIPT_CLASSES.get(script_index)?;

    for (style_index, style) in STYLE_CLASSES.iter().enumerate() {
        if style.feature.is_some() {
            continue;
        }

        if style.script.tag != script.tag {
            continue;
        }

        return Some(style_index);
    }

    None
}

pub(crate) fn none_default_style() -> Option<usize> {
    STYLE_CLASSES
        .iter()
        .enumerate()
        .find(|(_, style)| style.name == "NONE_DFLT")
        .map(|(idx, _)| idx)
}
