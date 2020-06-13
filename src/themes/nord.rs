// Some of the colours in the Nord palette aren’t used yet, but might be in the future.
#![allow(unused)]

/// An implementation of the dark version of [Nord](https://www.nordtheme.com). It follows [the
/// specification](https://www.nordtheme.com/docs/colors-and-palettes) as closely as possible.
#[derive(Debug)]
pub struct Nord;

// Polar Night
const NORD0: crate::Rgb = crate::rgb!(46, 52, 64);
const NORD1: crate::Rgb = crate::rgb!(59, 66, 82);
const NORD2: crate::Rgb = crate::rgb!(67, 76, 94);
const NORD3: crate::Rgb = crate::rgb!(76, 86, 106);

// Snow Storm
const NORD4: crate::Rgb = crate::rgb!(216, 222, 233);
const NORD5: crate::Rgb = crate::rgb!(229, 233, 240);
const NORD6: crate::Rgb = crate::rgb!(236, 239, 244);

// Frost
const NORD7: crate::Rgb = crate::rgb!(143, 188, 187);
const NORD8: crate::Rgb = crate::rgb!(136, 192, 208);
const NORD9: crate::Rgb = crate::rgb!(129, 161, 193);
const NORD10: crate::Rgb = crate::rgb!(94, 129, 172);

// Aurora
const NORD11: crate::Rgb = crate::rgb!(191, 97, 106);
const NORD12: crate::Rgb = crate::rgb!(208, 135, 112);
const NORD13: crate::Rgb = crate::rgb!(235, 203, 139);
const NORD14: crate::Rgb = crate::rgb!(163, 190, 140);
const NORD15: crate::Rgb = crate::rgb!(180, 142, 173);

impl crate::Theme for Nord {
    fn default_style(&self) -> crate::ResolvedStyle {
        crate::ResolvedStyle {
            fg_color: NORD6,
            bg_color: NORD0,
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: crate::HighlightGroup) -> crate::Style {
        match group {
            crate::HighlightGroup::CtrlFlowKeyword | crate::HighlightGroup::OtherKeyword => {
                crate::Style {
                    fg_color: Some(NORD9),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::FunctionDef | crate::HighlightGroup::FunctionCall => {
                crate::Style {
                    fg_color: Some(NORD8),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::TyDef
            | crate::HighlightGroup::TyUse
            | crate::HighlightGroup::InterfaceDef
            | crate::HighlightGroup::InterfaceUse
            | crate::HighlightGroup::PrimitiveTy => crate::Style {
                fg_color: Some(NORD7),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::VariableDef
            | crate::HighlightGroup::VariableUse
            | crate::HighlightGroup::MemberDef
            | crate::HighlightGroup::MemberUse
            | crate::HighlightGroup::ConstantDef
            | crate::HighlightGroup::ConstantUse
            | crate::HighlightGroup::FunctionParam => crate::Style {
                fg_color: Some(NORD4),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::ModuleDef | crate::HighlightGroup::ModuleUse => crate::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::MacroDef
            | crate::HighlightGroup::MacroUse
            | crate::HighlightGroup::PreProc => crate::Style {
                fg_color: Some(NORD10),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Unclear what highlighting this should get, as it is not specified by the Nord
            // Specification.
            crate::HighlightGroup::SpecialIdentDef | crate::HighlightGroup::SpecialIdentUse => {
                crate::Style {
                    fg_color: Some(NORD7),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::Number => crate::Style {
                fg_color: Some(NORD15),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::String | crate::HighlightGroup::StringDelimiter => {
                crate::Style {
                    fg_color: Some(NORD14),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::Character | crate::HighlightGroup::CharacterDelimiter => {
                crate::Style {
                    fg_color: Some(NORD13),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::Boolean => crate::Style {
                fg_color: Some(NORD9),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::Attribute => crate::Style {
                fg_color: Some(NORD12),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::Comment | crate::HighlightGroup::DocComment => crate::Style {
                fg_color: Some(NORD3),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::MemberOper
            | crate::HighlightGroup::PointerOper
            | crate::HighlightGroup::AssignOper
            | crate::HighlightGroup::BinaryOper
            | crate::HighlightGroup::OtherOper
            | crate::HighlightGroup::Separator
            | crate::HighlightGroup::Terminator => crate::Style {
                fg_color: Some(NORD9),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::Delimiter => crate::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::Error => crate::Style {
                fg_color: Some(NORD11),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },
        }
    }
}
