use crate::{common::font_params::ScriptParams, layout::Node};

pub fn calculate_script_shifts<G: crate::common::Glyph>(
    params: &ScriptParams,
    nucleus: &Option<Node<G>>,
    subscript: &Option<Node<G>>,
    superscript: &Option<Node<G>>,
) -> (f32, f32) {
    let mut vshift_down = params.subscript.shift_down;
    if let Some(subscript) = subscript {
        vshift_down = vshift_down.max(subscript.height() - params.subscript.top_max);

        // TODO: The following should only be applied on subformulars, operators, ...
        if let Some(base) = nucleus {
            vshift_down = vshift_down.max(base.depth() + params.subscript.baseline_drop_min);
        }
    }

    let mut vshift_up = params.superscript.shift_up;
    if let Some(superscript) = superscript {
        vshift_up = vshift_up.max(superscript.depth() + params.superscript.bottom_min);

        // TODO: The following should only be applied on subformulars, operators, ...
        if let Some(base) = nucleus {
            vshift_up = vshift_up.max(base.height() - params.superscript.baseline_drop_max);
        }
    }

    if let (Some(subscript), Some(superscript)) = (subscript, superscript) {
        let current_superscript_bottom = vshift_up - superscript.depth();
        let current_gap = current_superscript_bottom - (subscript.height() - vshift_down);
        let gap_diff = (params.sub_super_gap_min - current_gap).max(0.0);

        let max_shift_up = (params.super_bottom_max_with_subscript - current_superscript_bottom).max(vshift_up);
        let additional_shift_up = gap_diff.clamp(0.0, max_shift_up - vshift_up);

        vshift_up += additional_shift_up;
        vshift_down += gap_diff - additional_shift_up;
    }

    (-vshift_down, vshift_up)
}
