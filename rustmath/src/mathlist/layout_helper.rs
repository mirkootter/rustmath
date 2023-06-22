use crate::{common::font_params::ScriptParams, layout::Node};

pub fn calculate_script_shifts<G: crate::common::Glyph>(
    params: &ScriptParams,
    _nucleus: &Option<Node<G>>,
    _subscript: &Option<Node<G>>,
    _superscript: &Option<Node<G>>,
) -> (f32, f32) {
    (-params.subscript.shift_down, params.superscript.shift_up)
}
