use super::CharClassification;

pub const CHAR_CLASSIFICATION: [(u32, CharClassification); 597] = [
    (0, CharClassification::Ignore),
    (' ' as u32, CharClassification::Space),
    ('!' as u32, CharClassification::Normal),
    ('!' as u32 + 1, CharClassification::Ignore),
    ('#' as u32, CharClassification::Normal),
    ('&' as u32 + 1, CharClassification::Ignore),
    ('(' as u32, CharClassification::Opening),
    (')' as u32, CharClassification::Closing),
    ('*' as u32, CharClassification::Normal),
    ('+' as u32, CharClassification::Vary),
    (',' as u32, CharClassification::Punct),
    ('-' as u32, CharClassification::Normal),
    ('.' as u32, CharClassification::Punct),
    ('/' as u32, CharClassification::Binary),
    ('0' as u32, CharClassification::Normal),
    (':' as u32, CharClassification::Punct),
    ('<' as u32, CharClassification::Relation),
    ('?' as u32, CharClassification::Punct),
    ('@' as u32, CharClassification::Normal),
    ('A' as u32, CharClassification::Alphabetic),
    ('[' as u32, CharClassification::Opening),
    ('\\' as u32, CharClassification::Binary),
    (']' as u32, CharClassification::Closing),
    ('^' as u32, CharClassification::Normal),
    ('`' as u32, CharClassification::Diacritic),
    ('a' as u32, CharClassification::Alphabetic),
    ('{' as u32, CharClassification::Opening),
    ('|' as u32, CharClassification::Fence),
    ('}' as u32, CharClassification::Closing),
    ('~' as u32, CharClassification::Normal),
    ('~' as u32 + 1, CharClassification::Ignore),
    ('\u{a0}' as u32, CharClassification::Space),
    ('¡' as u32, CharClassification::Punct),
    ('¢' as u32, CharClassification::Normal),
    ('¨' as u32, CharClassification::Diacritic),
    ('¨' as u32 + 1, CharClassification::Ignore),
    ('¬' as u32, CharClassification::Unary),
    ('¬' as u32 + 1, CharClassification::Ignore),
    ('¯' as u32, CharClassification::Diacritic),
    ('°' as u32, CharClassification::Normal),
    ('±' as u32, CharClassification::Vary),
    ('²' as u32, CharClassification::Normal),
    ('·' as u32, CharClassification::Binary),
    ('·' as u32 + 1, CharClassification::Ignore),
    ('¹' as u32, CharClassification::Normal),
    ('¹' as u32 + 1, CharClassification::Ignore),
    ('¼' as u32, CharClassification::Normal),
    ('¿' as u32, CharClassification::Punct),
    ('¿' as u32 + 1, CharClassification::Ignore),
    ('×' as u32, CharClassification::Binary),
    ('×' as u32 + 1, CharClassification::Ignore),
    ('÷' as u32, CharClassification::Binary),
    ('÷' as u32 + 1, CharClassification::Ignore),
    ('ı' as u32, CharClassification::Alphabetic),
    ('ı' as u32 + 1, CharClassification::Ignore),
    ('ȷ' as u32, CharClassification::Alphabetic),
    ('ȷ' as u32 + 1, CharClassification::Ignore),
    ('ˆ' as u32, CharClassification::Diacritic),
    ('ˇ' as u32 + 1, CharClassification::Ignore),
    ('˘' as u32, CharClassification::Diacritic),
    ('˚' as u32 + 1, CharClassification::Ignore),
    ('˜' as u32, CharClassification::Diacritic),
    ('˜' as u32 + 1, CharClassification::Ignore),
    ('\u{300}' as u32, CharClassification::Diacritic),
    ('\u{308}' as u32 + 1, CharClassification::Ignore),
    ('\u{30a}' as u32, CharClassification::Diacritic),
    ('\u{30a}' as u32 + 1, CharClassification::Ignore),
    ('\u{30c}' as u32, CharClassification::Diacritic),
    ('\u{30c}' as u32 + 1, CharClassification::Ignore),
    ('\u{311}' as u32, CharClassification::Diacritic),
    ('\u{311}' as u32 + 1, CharClassification::Ignore),
    ('\u{323}' as u32, CharClassification::Diacritic),
    ('\u{323}' as u32 + 1, CharClassification::Ignore),
    ('\u{32c}' as u32, CharClassification::Diacritic),
    ('\u{333}' as u32 + 1, CharClassification::Ignore),
    ('\u{338}' as u32, CharClassification::Diacritic),
    ('\u{338}' as u32 + 1, CharClassification::Ignore),
    ('\u{33a}' as u32, CharClassification::Diacritic),
    ('\u{33a}' as u32 + 1, CharClassification::Ignore),
    ('\u{33f}' as u32, CharClassification::Diacritic),
    ('\u{33f}' as u32 + 1, CharClassification::Ignore),
    ('\u{346}' as u32, CharClassification::Diacritic),
    ('\u{346}' as u32 + 1, CharClassification::Ignore),
    ('Α' as u32, CharClassification::Alphabetic),
    ('Ρ' as u32 + 1, CharClassification::Ignore),
    ('Σ' as u32, CharClassification::Alphabetic),
    ('Τ' as u32 + 1, CharClassification::Ignore),
    ('Φ' as u32, CharClassification::Alphabetic),
    ('Ω' as u32 + 1, CharClassification::Ignore),
    ('α' as u32, CharClassification::Alphabetic),
    ('ρ' as u32 + 1, CharClassification::Ignore),
    ('σ' as u32, CharClassification::Alphabetic),
    ('ω' as u32 + 1, CharClassification::Ignore),
    ('ϐ' as u32, CharClassification::Alphabetic),
    ('ϒ' as u32 + 1, CharClassification::Ignore),
    ('ϕ' as u32, CharClassification::Alphabetic),
    ('ϖ' as u32 + 1, CharClassification::Ignore),
    ('Ϙ' as u32, CharClassification::Normal),
    ('Ϛ' as u32, CharClassification::Alphabetic),
    ('ϝ' as u32 + 1, CharClassification::Ignore),
    ('Ϡ' as u32, CharClassification::Alphabetic),
    ('ϡ' as u32 + 1, CharClassification::Ignore),
    ('ϰ' as u32, CharClassification::Alphabetic),
    ('ϱ' as u32 + 1, CharClassification::Ignore),
    ('ϴ' as u32, CharClassification::Alphabetic),
    ('϶' as u32, CharClassification::Normal),
    ('϶' as u32 + 1, CharClassification::Ignore),
    ('Ш' as u32, CharClassification::Alphabetic),
    ('Ш' as u32 + 1, CharClassification::Ignore),
    ('؆' as u32, CharClassification::Large),
    ('؈' as u32, CharClassification::Alphabetic),
    ('؈' as u32 + 1, CharClassification::Ignore),
    ('\u{2000}' as u32, CharClassification::Space),
    ('\u{2007}' as u32 + 1, CharClassification::Ignore),
    ('\u{2009}' as u32, CharClassification::Space),
    ('\u{200b}' as u32 + 1, CharClassification::Ignore),
    ('‐' as u32, CharClassification::Punct),
    ('‐' as u32 + 1, CharClassification::Ignore),
    ('‒' as u32, CharClassification::Punct),
    ('—' as u32 + 1, CharClassification::Ignore),
    ('‖' as u32, CharClassification::Fence),
    ('‖' as u32 + 1, CharClassification::Ignore),
    ('†' as u32, CharClassification::Relation),
    ('•' as u32, CharClassification::Binary),
    ('•' as u32 + 1, CharClassification::Ignore),
    ('…' as u32, CharClassification::Normal),
    ('…' as u32 + 1, CharClassification::Ignore),
    ('′' as u32, CharClassification::Normal),
    ('‷' as u32 + 1, CharClassification::Ignore),
    ('※' as u32, CharClassification::Normal),
    ('‼' as u32 + 1, CharClassification::Ignore),
    ('⁀' as u32, CharClassification::Binary),
    ('⁀' as u32 + 1, CharClassification::Ignore),
    ('⁄' as u32, CharClassification::Binary),
    ('⁄' as u32 + 1, CharClassification::Ignore),
    ('⁎' as u32, CharClassification::Binary),
    ('⁏' as u32, CharClassification::Relation),
    ('⁑' as u32, CharClassification::Normal),
    ('⁒' as u32 + 1, CharClassification::Ignore),
    ('⁗' as u32, CharClassification::Normal),
    ('⁗' as u32 + 1, CharClassification::Ignore),
    ('\u{205f}' as u32, CharClassification::Space),
    ('\u{205f}' as u32 + 1, CharClassification::Ignore),
    ('\u{2061}' as u32, CharClassification::Binary),
    ('\u{2063}' as u32, CharClassification::Punct),
    ('\u{2064}' as u32, CharClassification::Special),
    ('\u{2064}' as u32 + 1, CharClassification::Ignore),
    ('⁺' as u32, CharClassification::Normal),
    ('⁾' as u32 + 1, CharClassification::Ignore),
    ('₊' as u32, CharClassification::Normal),
    ('₎' as u32 + 1, CharClassification::Ignore),
    ('\u{20d0}' as u32, CharClassification::Diacritic),
    ('\u{20d3}' as u32, CharClassification::Special),
    ('\u{20d4}' as u32, CharClassification::Diacritic),
    ('\u{20df}' as u32 + 1, CharClassification::Ignore),
    ('\u{20e1}' as u32, CharClassification::Diacritic),
    ('\u{20e1}' as u32 + 1, CharClassification::Ignore),
    ('\u{20e4}' as u32, CharClassification::Diacritic),
    ('\u{20ef}' as u32 + 1, CharClassification::Ignore),
    ('ℂ' as u32, CharClassification::Alphabetic),
    ('ℂ' as u32 + 1, CharClassification::Ignore),
    ('ℇ' as u32, CharClassification::Normal),
    ('ℇ' as u32 + 1, CharClassification::Ignore),
    ('ℊ' as u32, CharClassification::Alphabetic),
    ('ℎ' as u32, CharClassification::Normal),
    ('ℐ' as u32, CharClassification::Alphabetic),
    ('ℓ' as u32 + 1, CharClassification::Ignore),
    ('ℕ' as u32, CharClassification::Alphabetic),
    ('ℕ' as u32 + 1, CharClassification::Ignore),
    ('℘' as u32, CharClassification::Alphabetic),
    ('ℝ' as u32 + 1, CharClassification::Ignore),
    ('ℤ' as u32, CharClassification::Alphabetic),
    ('ℤ' as u32 + 1, CharClassification::Ignore),
    ('Ω' as u32, CharClassification::Normal),
    ('ℨ' as u32, CharClassification::Alphabetic),
    ('℩' as u32, CharClassification::Normal),
    ('℩' as u32 + 1, CharClassification::Ignore),
    ('Å' as u32, CharClassification::Alphabetic),
    ('ℭ' as u32 + 1, CharClassification::Ignore),
    ('ℯ' as u32, CharClassification::Alphabetic),
    ('Ⅎ' as u32, CharClassification::Normal),
    ('ℳ' as u32, CharClassification::Alphabetic),
    ('ℸ' as u32 + 1, CharClassification::Ignore),
    ('ℼ' as u32, CharClassification::Alphabetic),
    ('ℾ' as u32, CharClassification::Normal),
    ('ℿ' as u32, CharClassification::Alphabetic),
    ('⅀' as u32, CharClassification::Large),
    ('⅁' as u32, CharClassification::Normal),
    ('ⅉ' as u32 + 1, CharClassification::Ignore),
    ('⅋' as u32, CharClassification::Normal),
    ('⅋' as u32 + 1, CharClassification::Ignore),
    ('←' as u32, CharClassification::Relation),
    ('↳' as u32 + 1, CharClassification::Ignore),
    ('↶' as u32, CharClassification::Relation),
    ('↷' as u32 + 1, CharClassification::Ignore),
    ('↺' as u32, CharClassification::Relation),
    ('∀' as u32, CharClassification::Unary),
    ('∂' as u32, CharClassification::Normal),
    ('∃' as u32, CharClassification::Unary),
    ('∅' as u32, CharClassification::Normal),
    ('∆' as u32, CharClassification::Unary),
    ('∈' as u32, CharClassification::Relation),
    ('∎' as u32, CharClassification::Normal),
    ('∏' as u32, CharClassification::Large),
    ('−' as u32, CharClassification::Vary),
    ('∔' as u32, CharClassification::Binary),
    ('√' as u32, CharClassification::Large),
    ('∝' as u32, CharClassification::Relation),
    ('∞' as u32, CharClassification::Normal),
    ('∣' as u32, CharClassification::Relation),
    ('∧' as u32, CharClassification::Binary),
    ('∫' as u32, CharClassification::Large),
    ('∴' as u32, CharClassification::Relation),
    ('∸' as u32, CharClassification::Binary),
    ('∹' as u32, CharClassification::Relation),
    ('∾' as u32, CharClassification::Binary),
    ('∿' as u32, CharClassification::Normal),
    ('≀' as u32, CharClassification::Binary),
    ('≁' as u32, CharClassification::Relation),
    ('⊌' as u32, CharClassification::Binary),
    ('⊏' as u32, CharClassification::Relation),
    ('⊓' as u32, CharClassification::Binary),
    ('⊢' as u32, CharClassification::Relation),
    ('⊤' as u32, CharClassification::Normal),
    ('⊥' as u32, CharClassification::Relation),
    ('⊹' as u32, CharClassification::Binary),
    ('⊾' as u32, CharClassification::Normal),
    ('⋀' as u32, CharClassification::Large),
    ('⋄' as u32, CharClassification::Binary),
    ('⋈' as u32, CharClassification::Relation),
    ('⋉' as u32, CharClassification::Binary),
    ('⋍' as u32, CharClassification::Relation),
    ('⋎' as u32, CharClassification::Binary),
    ('⋐' as u32, CharClassification::Relation),
    ('⋒' as u32, CharClassification::Binary),
    ('⋔' as u32, CharClassification::Relation),
    ('⌀' as u32, CharClassification::Normal),
    ('⌀' as u32 + 1, CharClassification::Ignore),
    ('⌂' as u32, CharClassification::Normal),
    ('⌂' as u32 + 1, CharClassification::Ignore),
    ('⌅' as u32, CharClassification::Binary),
    ('⌆' as u32 + 1, CharClassification::Ignore),
    ('⌈' as u32, CharClassification::Opening),
    ('⌉' as u32, CharClassification::Closing),
    ('⌊' as u32, CharClassification::Opening),
    ('⌋' as u32, CharClassification::Closing),
    ('⌋' as u32 + 1, CharClassification::Ignore),
    ('⌐' as u32, CharClassification::Normal),
    ('⌑' as u32 + 1, CharClassification::Ignore),
    ('⌙' as u32, CharClassification::Normal),
    ('⌙' as u32 + 1, CharClassification::Ignore),
    ('⌜' as u32, CharClassification::Opening),
    ('⌝' as u32, CharClassification::Closing),
    ('⌞' as u32, CharClassification::Opening),
    ('⌟' as u32, CharClassification::Closing),
    ('⌠' as u32, CharClassification::GlyphPart),
    ('⌢' as u32, CharClassification::Relation),
    ('⌣' as u32 + 1, CharClassification::Ignore),
    ('⌶' as u32, CharClassification::Normal),
    ('⌶' as u32 + 1, CharClassification::Ignore),
    ('⌽' as u32, CharClassification::Binary),
    ('⌽' as u32 + 1, CharClassification::Ignore),
    ('⌿' as u32, CharClassification::Relation),
    ('⌿' as u32 + 1, CharClassification::Ignore),
    ('⍼' as u32, CharClassification::Relation),
    ('⍼' as u32 + 1, CharClassification::Ignore),
    ('⎔' as u32, CharClassification::Normal),
    ('⎔' as u32 + 1, CharClassification::Ignore),
    ('⎛' as u32, CharClassification::GlyphPart),
    ('⎰' as u32, CharClassification::Relation),
    ('⎲' as u32, CharClassification::GlyphPart),
    ('⎴' as u32, CharClassification::Normal),
    ('⎷' as u32, CharClassification::GlyphPart),
    ('⎷' as u32 + 1, CharClassification::Ignore),
    ('⏐' as u32, CharClassification::GlyphPart),
    ('⏐' as u32 + 1, CharClassification::Ignore),
    ('⏜' as u32, CharClassification::Normal),
    ('⏧' as u32 + 1, CharClassification::Ignore),
    ('Ⓢ' as u32, CharClassification::Normal),
    ('Ⓢ' as u32 + 1, CharClassification::Ignore),
    ('■' as u32, CharClassification::Normal),
    ('□' as u32 + 1, CharClassification::Ignore),
    ('▪' as u32, CharClassification::Normal),
    ('▫' as u32 + 1, CharClassification::Ignore),
    ('▭' as u32, CharClassification::Normal),
    ('▲' as u32, CharClassification::Binary),
    ('▹' as u32 + 1, CharClassification::Ignore),
    ('▼' as u32, CharClassification::Binary),
    ('◆' as u32, CharClassification::Normal),
    ('◊' as u32, CharClassification::Binary),
    ('○' as u32 + 1, CharClassification::Ignore),
    ('◎' as u32, CharClassification::Normal),
    ('◓' as u32 + 1, CharClassification::Ignore),
    ('◖' as u32, CharClassification::Normal),
    ('◗' as u32 + 1, CharClassification::Ignore),
    ('◢' as u32, CharClassification::Normal),
    ('◦' as u32, CharClassification::Binary),
    ('◧' as u32, CharClassification::Normal),
    ('◫' as u32, CharClassification::Binary),
    ('◬' as u32 + 1, CharClassification::Ignore),
    ('◯' as u32, CharClassification::Normal),
    ('◯' as u32 + 1, CharClassification::Ignore),
    ('◸' as u32, CharClassification::Binary),
    ('◿' as u32 + 1, CharClassification::Ignore),
    ('★' as u32, CharClassification::Binary),
    ('☆' as u32 + 1, CharClassification::Ignore),
    ('☉' as u32, CharClassification::Normal),
    ('☉' as u32 + 1, CharClassification::Ignore),
    ('☌' as u32, CharClassification::Normal),
    ('☌' as u32 + 1, CharClassification::Ignore),
    ('☽' as u32, CharClassification::Normal),
    ('♄' as u32 + 1, CharClassification::Ignore),
    ('♆' as u32, CharClassification::Normal),
    ('♉' as u32 + 1, CharClassification::Ignore),
    ('♠' as u32, CharClassification::Normal),
    ('♧' as u32 + 1, CharClassification::Ignore),
    ('♩' as u32, CharClassification::Normal),
    ('♩' as u32 + 1, CharClassification::Ignore),
    ('♭' as u32, CharClassification::Normal),
    ('♯' as u32 + 1, CharClassification::Ignore),
    ('⚀' as u32, CharClassification::Normal),
    ('⚉' as u32 + 1, CharClassification::Ignore),
    ('⚪' as u32, CharClassification::Normal),
    ('⚬' as u32 + 1, CharClassification::Ignore),
    ('⚲' as u32, CharClassification::Normal),
    ('⚲' as u32 + 1, CharClassification::Ignore),
    ('✓' as u32, CharClassification::Normal),
    ('✓' as u32 + 1, CharClassification::Ignore),
    ('✗' as u32, CharClassification::Normal),
    ('✗' as u32 + 1, CharClassification::Ignore),
    ('✠' as u32, CharClassification::Normal),
    ('✠' as u32 + 1, CharClassification::Ignore),
    ('✪' as u32, CharClassification::Normal),
    ('✪' as u32 + 1, CharClassification::Ignore),
    ('✶' as u32, CharClassification::Normal),
    ('✶' as u32 + 1, CharClassification::Ignore),
    ('❲' as u32, CharClassification::Opening),
    ('❳' as u32, CharClassification::Closing),
    ('❳' as u32 + 1, CharClassification::Ignore),
    ('⟀' as u32, CharClassification::Normal),
    ('⟂' as u32, CharClassification::Relation),
    ('⟌' as u32, CharClassification::Large),
    ('⟍' as u32, CharClassification::Relation),
    ('⟎' as u32, CharClassification::Binary),
    ('⟐' as u32, CharClassification::Normal),
    ('⟑' as u32, CharClassification::Binary),
    ('⟒' as u32, CharClassification::Relation),
    ('⟕' as u32, CharClassification::Large),
    ('⟚' as u32, CharClassification::Relation),
    ('⟠' as u32, CharClassification::Binary),
    ('⟦' as u32, CharClassification::Opening),
    ('⟧' as u32, CharClassification::Closing),
    ('⟨' as u32, CharClassification::Opening),
    ('⟩' as u32, CharClassification::Closing),
    ('⟪' as u32, CharClassification::Opening),
    ('⟫' as u32, CharClassification::Closing),
    ('⟬' as u32, CharClassification::Opening),
    ('⟭' as u32, CharClassification::Closing),
    ('⟮' as u32, CharClassification::Opening),
    ('⟯' as u32, CharClassification::Closing),
    ('⟰' as u32, CharClassification::Relation),
    ('⟿' as u32 + 1, CharClassification::Ignore),
    ('⤀' as u32, CharClassification::Relation),
    ('⦀' as u32, CharClassification::Fence),
    ('⦁' as u32, CharClassification::Normal),
    ('⦂' as u32, CharClassification::Fence),
    ('⦃' as u32, CharClassification::Opening),
    ('⦄' as u32, CharClassification::Closing),
    ('⦅' as u32, CharClassification::Opening),
    ('⦆' as u32, CharClassification::Closing),
    ('⦇' as u32, CharClassification::Opening),
    ('⦈' as u32, CharClassification::Closing),
    ('⦉' as u32, CharClassification::Opening),
    ('⦊' as u32, CharClassification::Closing),
    ('⦋' as u32, CharClassification::Opening),
    ('⦌' as u32, CharClassification::Closing),
    ('⦍' as u32, CharClassification::Opening),
    ('⦎' as u32, CharClassification::Closing),
    ('⦏' as u32, CharClassification::Opening),
    ('⦐' as u32, CharClassification::Closing),
    ('⦑' as u32, CharClassification::Opening),
    ('⦒' as u32, CharClassification::Closing),
    ('⦓' as u32, CharClassification::Opening),
    ('⦔' as u32, CharClassification::Closing),
    ('⦕' as u32, CharClassification::Opening),
    ('⦖' as u32, CharClassification::Closing),
    ('⦗' as u32, CharClassification::Opening),
    ('⦘' as u32, CharClassification::Closing),
    ('⦙' as u32, CharClassification::Fence),
    ('⦛' as u32, CharClassification::Normal),
    ('⦶' as u32, CharClassification::Binary),
    ('⦺' as u32, CharClassification::Normal),
    ('⧀' as u32, CharClassification::Binary),
    ('⧂' as u32, CharClassification::Normal),
    ('⧄' as u32, CharClassification::Binary),
    ('⧉' as u32, CharClassification::Normal),
    ('⧎' as u32, CharClassification::Relation),
    ('⧖' as u32, CharClassification::Binary),
    ('⧘' as u32, CharClassification::Opening),
    ('⧙' as u32, CharClassification::Closing),
    ('⧚' as u32, CharClassification::Opening),
    ('⧛' as u32, CharClassification::Closing),
    ('⧜' as u32, CharClassification::Normal),
    ('⧟' as u32, CharClassification::Relation),
    ('⧠' as u32, CharClassification::Normal),
    ('⧡' as u32, CharClassification::Relation),
    ('⧢' as u32, CharClassification::Binary),
    ('⧣' as u32, CharClassification::Relation),
    ('⧧' as u32, CharClassification::Normal),
    ('⧫' as u32, CharClassification::Binary),
    ('⧬' as u32, CharClassification::Normal),
    ('⧴' as u32, CharClassification::Relation),
    ('⧵' as u32, CharClassification::Binary),
    ('⧸' as u32, CharClassification::Large),
    ('⧺' as u32, CharClassification::Binary),
    ('⧼' as u32, CharClassification::Opening),
    ('⧽' as u32, CharClassification::Closing),
    ('⧾' as u32, CharClassification::Binary),
    ('⨀' as u32, CharClassification::Large),
    ('⨢' as u32, CharClassification::Binary),
    ('⩙' as u32, CharClassification::Relation),
    ('⩚' as u32, CharClassification::Binary),
    ('⩦' as u32, CharClassification::Relation),
    ('⩱' as u32, CharClassification::Binary),
    ('⩳' as u32, CharClassification::Relation),
    ('⫡' as u32, CharClassification::Normal),
    ('⫢' as u32, CharClassification::Relation),
    ('⫱' as u32, CharClassification::Normal),
    ('⫲' as u32, CharClassification::Relation),
    ('⫴' as u32, CharClassification::Binary),
    ('⫷' as u32, CharClassification::Relation),
    ('⫻' as u32, CharClassification::Binary),
    ('⫼' as u32, CharClassification::Large),
    ('⫽' as u32, CharClassification::Binary),
    ('⫿' as u32, CharClassification::Large),
    ('⬀' as u32, CharClassification::Relation),
    ('⬒' as u32, CharClassification::Normal),
    ('⬙' as u32 + 1, CharClassification::Ignore),
    ('⬛' as u32, CharClassification::Normal),
    ('⬰' as u32, CharClassification::Relation),
    ('⭌' as u32 + 1, CharClassification::Ignore),
    ('⭐' as u32, CharClassification::Normal),
    ('⭔' as u32 + 1, CharClassification::Ignore),
    ('⮕' as u32, CharClassification::Relation),
    ('⮕' as u32 + 1, CharClassification::Ignore),
    ('⯂' as u32, CharClassification::Normal),
    ('⯈' as u32 + 1, CharClassification::Ignore),
    ('⯊' as u32, CharClassification::Normal),
    ('⯋' as u32 + 1, CharClassification::Ignore),
    ('〈' as u32, CharClassification::Special),
    ('〉' as u32 + 1, CharClassification::Ignore),
    ('〚' as u32, CharClassification::Special),
    ('〛' as u32 + 1, CharClassification::Ignore),
    ('の' as u32, CharClassification::Normal),
    ('の' as u32 + 1, CharClassification::Ignore),
    ('﬩' as u32, CharClassification::Special),
    ('﬩' as u32 + 1, CharClassification::Ignore),
    ('\u{fe00}' as u32, CharClassification::Diacritic),
    ('\u{fe00}' as u32 + 1, CharClassification::Ignore),
    ('﹡' as u32, CharClassification::Special),
    ('﹦' as u32 + 1, CharClassification::Ignore),
    ('﹨' as u32, CharClassification::Special),
    ('﹨' as u32 + 1, CharClassification::Ignore),
    ('＋' as u32, CharClassification::Special),
    ('＋' as u32 + 1, CharClassification::Ignore),
    ('＜' as u32, CharClassification::Special),
    ('＞' as u32 + 1, CharClassification::Ignore),
    ('＼' as u32, CharClassification::Special),
    ('＼' as u32 + 1, CharClassification::Ignore),
    ('＾' as u32, CharClassification::Special),
    ('＾' as u32 + 1, CharClassification::Ignore),
    ('｜' as u32, CharClassification::Special),
    ('｜' as u32 + 1, CharClassification::Ignore),
    ('～' as u32, CharClassification::Special),
    ('～' as u32 + 1, CharClassification::Ignore),
    ('￢' as u32, CharClassification::Special),
    ('￢' as u32 + 1, CharClassification::Ignore),
    ('￩' as u32, CharClassification::Special),
    ('￬' as u32 + 1, CharClassification::Ignore),
    ('𝐀' as u32, CharClassification::Alphabetic),
    ('𝑔' as u32 + 1, CharClassification::Ignore),
    ('𝑖' as u32, CharClassification::Alphabetic),
    ('𝒜' as u32 + 1, CharClassification::Ignore),
    ('𝒞' as u32, CharClassification::Alphabetic),
    ('𝒟' as u32 + 1, CharClassification::Ignore),
    ('𝒢' as u32, CharClassification::Alphabetic),
    ('𝒢' as u32 + 1, CharClassification::Ignore),
    ('𝒥' as u32, CharClassification::Alphabetic),
    ('𝒦' as u32 + 1, CharClassification::Ignore),
    ('𝒩' as u32, CharClassification::Alphabetic),
    ('𝒬' as u32 + 1, CharClassification::Ignore),
    ('𝒮' as u32, CharClassification::Alphabetic),
    ('𝒹' as u32 + 1, CharClassification::Ignore),
    ('𝒻' as u32, CharClassification::Alphabetic),
    ('𝒻' as u32 + 1, CharClassification::Ignore),
    ('𝒽' as u32, CharClassification::Alphabetic),
    ('𝓃' as u32 + 1, CharClassification::Ignore),
    ('𝓅' as u32, CharClassification::Alphabetic),
    ('𝔅' as u32 + 1, CharClassification::Ignore),
    ('𝔇' as u32, CharClassification::Alphabetic),
    ('𝔊' as u32 + 1, CharClassification::Ignore),
    ('𝔍' as u32, CharClassification::Alphabetic),
    ('𝔔' as u32 + 1, CharClassification::Ignore),
    ('𝔖' as u32, CharClassification::Alphabetic),
    ('𝔜' as u32 + 1, CharClassification::Ignore),
    ('𝔞' as u32, CharClassification::Alphabetic),
    ('𝔹' as u32 + 1, CharClassification::Ignore),
    ('𝔻' as u32, CharClassification::Alphabetic),
    ('𝔾' as u32 + 1, CharClassification::Ignore),
    ('𝕀' as u32, CharClassification::Alphabetic),
    ('𝕄' as u32 + 1, CharClassification::Ignore),
    ('𝕆' as u32, CharClassification::Alphabetic),
    ('𝕆' as u32 + 1, CharClassification::Ignore),
    ('𝕊' as u32, CharClassification::Alphabetic),
    ('𝕐' as u32 + 1, CharClassification::Ignore),
    ('𝕒' as u32, CharClassification::Alphabetic),
    ('𝚥' as u32 + 1, CharClassification::Ignore),
    ('𝚨' as u32, CharClassification::Alphabetic),
    ('𝟋' as u32 + 1, CharClassification::Ignore),
    ('𝟎' as u32, CharClassification::Normal),
    ('𝟿' as u32 + 1, CharClassification::Ignore),
    ('𞸀' as u32, CharClassification::Alphabetic),
    ('𞸃' as u32 + 1, CharClassification::Ignore),
    ('𞸅' as u32, CharClassification::Alphabetic),
    ('𞸟' as u32 + 1, CharClassification::Ignore),
    ('𞸡' as u32, CharClassification::Alphabetic),
    ('𞸢' as u32 + 1, CharClassification::Ignore),
    ('𞸤' as u32, CharClassification::Alphabetic),
    ('𞸤' as u32 + 1, CharClassification::Ignore),
    ('𞸧' as u32, CharClassification::Alphabetic),
    ('𞸧' as u32 + 1, CharClassification::Ignore),
    ('𞸩' as u32, CharClassification::Alphabetic),
    ('𞸲' as u32 + 1, CharClassification::Ignore),
    ('𞸴' as u32, CharClassification::Alphabetic),
    ('𞸷' as u32 + 1, CharClassification::Ignore),
    ('𞸹' as u32, CharClassification::Alphabetic),
    ('𞸹' as u32 + 1, CharClassification::Ignore),
    ('𞸻' as u32, CharClassification::Alphabetic),
    ('𞸻' as u32 + 1, CharClassification::Ignore),
    ('𞹂' as u32, CharClassification::Alphabetic),
    ('𞹂' as u32 + 1, CharClassification::Ignore),
    ('𞹇' as u32, CharClassification::Alphabetic),
    ('𞹇' as u32 + 1, CharClassification::Ignore),
    ('𞹉' as u32, CharClassification::Alphabetic),
    ('𞹉' as u32 + 1, CharClassification::Ignore),
    ('𞹋' as u32, CharClassification::Alphabetic),
    ('𞹋' as u32 + 1, CharClassification::Ignore),
    ('𞹍' as u32, CharClassification::Alphabetic),
    ('𞹏' as u32 + 1, CharClassification::Ignore),
    ('𞹑' as u32, CharClassification::Alphabetic),
    ('𞹒' as u32 + 1, CharClassification::Ignore),
    ('𞹔' as u32, CharClassification::Alphabetic),
    ('𞹔' as u32 + 1, CharClassification::Ignore),
    ('𞹗' as u32, CharClassification::Alphabetic),
    ('𞹗' as u32 + 1, CharClassification::Ignore),
    ('𞹙' as u32, CharClassification::Alphabetic),
    ('𞹙' as u32 + 1, CharClassification::Ignore),
    ('𞹛' as u32, CharClassification::Alphabetic),
    ('𞹛' as u32 + 1, CharClassification::Ignore),
    ('𞹝' as u32, CharClassification::Alphabetic),
    ('𞹝' as u32 + 1, CharClassification::Ignore),
    ('𞹟' as u32, CharClassification::Alphabetic),
    ('𞹟' as u32 + 1, CharClassification::Ignore),
    ('𞹡' as u32, CharClassification::Alphabetic),
    ('𞹢' as u32 + 1, CharClassification::Ignore),
    ('𞹤' as u32, CharClassification::Alphabetic),
    ('𞹤' as u32 + 1, CharClassification::Ignore),
    ('𞹧' as u32, CharClassification::Alphabetic),
    ('𞹪' as u32 + 1, CharClassification::Ignore),
    ('𞹬' as u32, CharClassification::Alphabetic),
    ('𞹲' as u32 + 1, CharClassification::Ignore),
    ('𞹴' as u32, CharClassification::Alphabetic),
    ('𞹷' as u32 + 1, CharClassification::Ignore),
    ('𞹹' as u32, CharClassification::Alphabetic),
    ('𞹼' as u32 + 1, CharClassification::Ignore),
    ('𞹾' as u32, CharClassification::Alphabetic),
    ('𞹾' as u32 + 1, CharClassification::Ignore),
    ('𞺀' as u32, CharClassification::Alphabetic),
    ('𞺉' as u32 + 1, CharClassification::Ignore),
    ('𞺋' as u32, CharClassification::Alphabetic),
    ('𞺛' as u32 + 1, CharClassification::Ignore),
    ('𞺡' as u32, CharClassification::Alphabetic),
    ('𞺣' as u32 + 1, CharClassification::Ignore),
    ('𞺥' as u32, CharClassification::Alphabetic),
    ('𞺩' as u32 + 1, CharClassification::Ignore),
    ('𞺫' as u32, CharClassification::Alphabetic),
    ('𞺻' as u32 + 1, CharClassification::Ignore),
    ('𞻰' as u32, CharClassification::Large),
    ('𞻱' as u32 + 1, CharClassification::Ignore),
    ('🞄' as u32, CharClassification::Normal),
    ('🞄' as u32 + 1, CharClassification::Ignore),
    ('🞌' as u32, CharClassification::Normal),
    ('🞍' as u32 + 1, CharClassification::Ignore),
    ('🞗' as u32, CharClassification::Normal),
    ('🞙' as u32 + 1, CharClassification::Ignore),
    ('🞝' as u32, CharClassification::Normal),
    ('🞟' as u32 + 1, CharClassification::Ignore),
];
