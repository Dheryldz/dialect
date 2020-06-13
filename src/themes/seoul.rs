/// An attempted implementation of [seoul256](https://github.com/junegunn/seoul256.vim). The
/// original theme could not be replicated exactly because it differentiates between highlight
/// groups that do not exist in Dialect.
#[derive(Debug)]
pub struct Seoul;

const BLUE: crate::Rgb = crate::rgb!(152, 190, 222);
const BROWN: crate::Rgb = crate::rgb!(190, 152, 115);
const CREAM: crate::Rgb = crate::rgb!(223, 222, 189);
const CYAN: crate::Rgb = crate::rgb!(111, 188, 189);
const DARK_GREEN: crate::Rgb = crate::rgb!(113, 152, 114);
const GREEN: crate::Rgb = crate::rgb!(152, 188, 153);
const KHAKI: crate::Rgb = crate::rgb!(189, 187, 114);
const LEMON: crate::Rgb = crate::rgb!(222, 221, 153);
const LIGHT_BLUE: crate::Rgb = crate::rgb!(152, 188, 189);
const LIGHT_YELLOW: crate::Rgb = crate::rgb!(255, 222, 153);
const PURPLE: crate::Rgb = crate::rgb!(225, 120, 153);
const SALMON: crate::Rgb = crate::rgb!(255, 191, 189);
const VIOLET: crate::Rgb = crate::rgb!(153, 154, 189);
const YELLOW: crate::Rgb = crate::rgb!(223, 188, 114);

impl crate::Theme for Seoul {
    fn default_style(&self) -> crate::ResolvedStyle {
        crate::ResolvedStyle {
            fg_color: crate::rgb!(217, 217, 217),
            bg_color: crate::rgb!(75, 75, 75),
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: crate::HighlightGroup) -> crate::Style {
        match group {
            crate::HighlightGroup::CtrlFlowKeyword => crate::Style {
                fg_color: Some(BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::OtherKeyword => crate::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::FunctionDef | crate::HighlightGroup::FunctionCall => {
                crate::Style {
                    fg_color: Some(CREAM),
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
                fg_color: Some(YELLOW),
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
                fg_color: Some(SALMON),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::ConstantDef | crate::HighlightGroup::ConstantUse => {
                crate::Style {
                    fg_color: Some(SALMON),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

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
                fg_color: Some(KHAKI),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::SpecialIdentDef | crate::HighlightGroup::SpecialIdentUse => {
                crate::Style {
                    fg_color: Some(CYAN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::Number => crate::Style {
                fg_color: Some(LIGHT_YELLOW),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::String | crate::HighlightGroup::Character => crate::Style {
                fg_color: Some(LIGHT_BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::StringDelimiter | crate::HighlightGroup::CharacterDelimiter => {
                crate::Style {
                    fg_color: Some(BROWN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::Boolean => crate::Style {
                fg_color: Some(VIOLET),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::Attribute => crate::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::Comment | crate::HighlightGroup::DocComment => crate::Style {
                fg_color: Some(DARK_GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::MemberOper
            | crate::HighlightGroup::PointerOper
            | crate::HighlightGroup::AssignOper
            | crate::HighlightGroup::BinaryOper
            | crate::HighlightGroup::OtherOper => crate::Style {
                fg_color: Some(LEMON),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::Separator
            | crate::HighlightGroup::Delimiter
            | crate::HighlightGroup::Terminator => crate::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::Error => crate::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: true,
            },
        }
    }
}
