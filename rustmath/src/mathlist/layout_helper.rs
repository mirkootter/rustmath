use crate::{common::font_params::ScriptParams, layout::Node};

pub fn calculate_script_shifts<G: crate::common::Glyph>(
    params: &ScriptParams,
    _nucleus: &Option<Node<G>>,
    subscript: &Option<Node<G>>,
    superscript: &Option<Node<G>>,
) -> (f32, f32) {
    let mut vshift_down = params.subscript.shift_down;
    if let Some(subscript) = subscript {
        vshift_down = vshift_down.max(subscript.height() - params.subscript.top_max);
        // TODO: Baseline drop?
    }

    let mut vshift_up = params.superscript.shift_up;
    if let Some(superscript) = superscript {
        vshift_up = vshift_up.max(superscript.depth() + params.superscript.bottom_min);
        // TODO: Baseline drop?
    }

    (-vshift_down, vshift_up)
}
