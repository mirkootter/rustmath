#[derive(Clone)]
pub struct SubScriptParams {
    pub shift_down: f32,
    pub top_max: f32,
    pub baseline_drop_min: f32,
}

#[derive(Clone)]
pub struct SuperScriptParams {
    pub shift_up: f32,
    pub bottom_min: f32,
    pub baseline_drop_max: f32,
}

#[derive(Clone)]
pub struct ScriptParams {
    pub subscript: SubScriptParams,
    pub superscript: SuperScriptParams,
    pub sub_super_gap_min: f32,
    pub super_bottom_max_with_subscript: f32,
}
