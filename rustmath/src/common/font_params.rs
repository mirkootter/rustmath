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

#[derive(Clone)]
pub struct LimitParams {
    pub upper_gap_min: f32,
    pub upper_baseline_rise_min: f32,
    pub lower_gap_min: f32,
    pub lower_baseline_drop_min: f32,
}

#[derive(Clone)]
pub struct GeneralParams {
    pub axis_height: f32,
}

#[derive(Clone)]
pub struct FractionPartParams {
    pub shift: f32,
    pub gap_min: f32,
}

#[derive(Clone)]
pub struct FractionParams {
    pub numerator: FractionPartParams,
    pub denominator: FractionPartParams,
    pub rule_thickness: f32,
}
