/// A port of the Zenburn theme.
///
/// In actuality, this theme is not based off of [the original Zenburn][original], but rather [Pale
/// Fire][pale-fire], a VS Code theme that is in turn based upon [zenburn-emacs][emacs], which
/// finally is a direct port of the original.
///
/// [original]: https://github.com/jnurmine/Zenburn
/// [pale-fire]: https://github.com/matklad/pale-fire
/// [emacs]: https://github.com/bbatsov/zenburn-emacs
#[derive(Debug)]
pub struct Zenburn;

#[allow(dead_code)]
mod palette {
    pub(super) const FG_PLUS_1: crate::Rgb = crate::rgb!(0xFF, 0xFF, 0xEF);
    pub(super) const FG: crate::Rgb = crate::rgb!(0xDC, 0xDC, 0xCC);
    pub(super) const FG_MINUS_1: crate::Rgb = crate::rgb!(0x65, 0x65, 0x55);
    pub(super) const BG_MINUS_2: crate::Rgb = crate::rgb!(0x00, 0x00, 0x00);
    pub(super) const BG_MINUS_1: crate::Rgb = crate::rgb!(0x2B, 0x2B, 0x2B);
    pub(super) const BG_MINUS_05: crate::Rgb = crate::rgb!(0x38, 0x38, 0x38);
    pub(super) const BG: crate::Rgb = crate::rgb!(0x3F, 0x3F, 0x3F);
    pub(super) const BG_PLUS_05: crate::Rgb = crate::rgb!(0x49, 0x49, 0x49);
    pub(super) const BG_PLUS_1: crate::Rgb = crate::rgb!(0x4F, 0x4F, 0x4F);
    pub(super) const BG_PLUS_2: crate::Rgb = crate::rgb!(0x5F, 0x5F, 0x5F);
    pub(super) const BG_PLUS_3: crate::Rgb = crate::rgb!(0x6F, 0x6F, 0x6F);
    pub(super) const RED_PLUS_2: crate::Rgb = crate::rgb!(0xEC, 0xB3, 0xB3);
    pub(super) const RED_PLUS_1: crate::Rgb = crate::rgb!(0xDC, 0xA3, 0xA3);
    pub(super) const RED: crate::Rgb = crate::rgb!(0xCC, 0x93, 0x93);
    pub(super) const RED_MINUS_1: crate::Rgb = crate::rgb!(0xBC, 0x83, 0x83);
    pub(super) const RED_MINUS_2: crate::Rgb = crate::rgb!(0xAC, 0x73, 0x73);
    pub(super) const RED_MINUS_3: crate::Rgb = crate::rgb!(0x9C, 0x63, 0x63);
    pub(super) const RED_MINUS_4: crate::Rgb = crate::rgb!(0x8C, 0x53, 0x53);
    pub(super) const RED_MINUS_5: crate::Rgb = crate::rgb!(0x7C, 0x43, 0x43);
    pub(super) const RED_MINUS_6: crate::Rgb = crate::rgb!(0x6C, 0x33, 0x33);
    pub(super) const ORANGE: crate::Rgb = crate::rgb!(0xDF, 0xAF, 0x8F);
    pub(super) const YELLOW: crate::Rgb = crate::rgb!(0xF0, 0xDF, 0xAF);
    pub(super) const YELLOW_MINUS_1: crate::Rgb = crate::rgb!(0xE0, 0xCF, 0x9F);
    pub(super) const YELLOW_MINUS_2: crate::Rgb = crate::rgb!(0xD0, 0xBF, 0x8F);
    pub(super) const GREEN_MINUS_5: crate::Rgb = crate::rgb!(0x2F, 0x4F, 0x2F);
    pub(super) const GREEN_MINUS_4: crate::Rgb = crate::rgb!(0x3F, 0x5F, 0x3F);
    pub(super) const GREEN_MINUS_3: crate::Rgb = crate::rgb!(0x4F, 0x6F, 0x4F);
    pub(super) const GREEN_MINUS_2: crate::Rgb = crate::rgb!(0x5F, 0x7F, 0x5F);
    pub(super) const GREEN_MINUS_1: crate::Rgb = crate::rgb!(0x6F, 0x8F, 0x6F);
    pub(super) const GREEN: crate::Rgb = crate::rgb!(0x7F, 0x9F, 0x7F);
    pub(super) const GREEN_PLUS_1: crate::Rgb = crate::rgb!(0x8F, 0xB2, 0x8F);
    pub(super) const GREEN_PLUS_2: crate::Rgb = crate::rgb!(0x9F, 0xC5, 0x9F);
    pub(super) const GREEN_PLUS_3: crate::Rgb = crate::rgb!(0xAF, 0xD8, 0xAF);
    pub(super) const GREEN_PLUS_4: crate::Rgb = crate::rgb!(0xBF, 0xEB, 0xBF);
    pub(super) const CYAN: crate::Rgb = crate::rgb!(0x93, 0xE0, 0xE3);
    pub(super) const BLUE_PLUS_3: crate::Rgb = crate::rgb!(0xBD, 0xE0, 0xF3);
    pub(super) const BLUE_PLUS_2: crate::Rgb = crate::rgb!(0xAC, 0xE0, 0xE3);
    pub(super) const BLUE_PLUS_1: crate::Rgb = crate::rgb!(0x94, 0xBF, 0xF3);
    pub(super) const BLUE: crate::Rgb = crate::rgb!(0x8C, 0xD0, 0xD3);
    pub(super) const BLUE_MINUS_1: crate::Rgb = crate::rgb!(0x7C, 0xB8, 0xBB);
    pub(super) const BLUE_MINUS_2: crate::Rgb = crate::rgb!(0x6C, 0xA0, 0xA3);
    pub(super) const BLUE_MINUS_3: crate::Rgb = crate::rgb!(0x5C, 0x88, 0x8B);
    pub(super) const BLUE_MINUS_4: crate::Rgb = crate::rgb!(0x4C, 0x70, 0x73);
    pub(super) const BLUE_MINUS_5: crate::Rgb = crate::rgb!(0x36, 0x60, 0x60);
    pub(super) const MAGENTA: crate::Rgb = crate::rgb!(0xDC, 0x8C, 0xC3);
}

#[allow(unused_imports)]
use palette::*;

impl crate::Theme for Zenburn {
    fn default_style(&self) -> crate::ResolvedStyle {
        crate::ResolvedStyle {
            fg_color: FG,
            bg_color: BG,
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: crate::HighlightGroup) -> crate::Style {
        match group {
            crate::HighlightGroup::CtrlFlowKeyword
            | crate::HighlightGroup::OtherKeyword
            | crate::HighlightGroup::Boolean => crate::Style {
                fg_color: Some(YELLOW),
                bg_color: None,
                is_bold: true,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::FunctionDef | crate::HighlightGroup::FunctionCall => {
                crate::Style {
                    fg_color: Some(CYAN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::TyDef | crate::HighlightGroup::TyUse => crate::Style {
                fg_color: Some(BLUE_MINUS_1),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::InterfaceDef | crate::HighlightGroup::InterfaceUse => {
                crate::Style {
                    fg_color: Some(BLUE),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::PrimitiveTy => crate::Style {
                fg_color: Some(BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::VariableDef
            | crate::HighlightGroup::VariableUse
            | crate::HighlightGroup::FunctionParam => crate::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::MemberDef | crate::HighlightGroup::MemberUse => crate::Style {
                fg_color: Some(ORANGE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::ConstantDef | crate::HighlightGroup::ConstantUse => {
                crate::Style {
                    fg_color: Some(BLUE_PLUS_3),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::ModuleDef | crate::HighlightGroup::ModuleUse => crate::Style {
                fg_color: Some(GREEN_PLUS_4),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::MacroDef
            | crate::HighlightGroup::MacroUse
            | crate::HighlightGroup::PreProc
            | crate::HighlightGroup::Attribute => crate::Style {
                fg_color: Some(BLUE_PLUS_1),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::SpecialIdentDef | crate::HighlightGroup::SpecialIdentUse => {
                crate::Style {
                    fg_color: Some(ORANGE),
                    bg_color: None,
                    is_bold: false,
                    is_italic: true,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::Number => crate::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::String | crate::HighlightGroup::StringDelimiter => {
                crate::Style {
                    fg_color: Some(RED),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::Character | crate::HighlightGroup::CharacterDelimiter => {
                crate::Style {
                    fg_color: None,
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::Comment => crate::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::DocComment => crate::Style {
                fg_color: Some(GREEN_PLUS_2),
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
            | crate::HighlightGroup::Delimiter
            | crate::HighlightGroup::Separator
            | crate::HighlightGroup::Terminator => crate::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::Error => crate::Style {
                fg_color: Some(RED_MINUS_3),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: true,
            },
        }
    }
}
