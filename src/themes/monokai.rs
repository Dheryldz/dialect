/// Replicates as closely as possible [the original Monokai
/// theme](https://web.archive.org/web/20071128121331/http://www.monokai.nl/blog/2006/07/15/textmate-color-theme/).
#[derive(Debug)]
pub struct Monokai;

const FADED: crate::Rgb = crate::rgb!(117, 113, 94);

const CYAN: crate::Rgb = crate::rgb!(102, 217, 239);
const GREEN: crate::Rgb = crate::rgb!(166, 226, 46);
const ORANGE: crate::Rgb = crate::rgb!(253, 151, 31);
const PINK: crate::Rgb = crate::rgb!(249, 38, 114);
const PURPLE: crate::Rgb = crate::rgb!(174, 129, 255);
const YELLOW: crate::Rgb = crate::rgb!(230, 219, 116);

impl crate::Theme for Monokai {
    fn default_style(&self) -> crate::ResolvedStyle {
        crate::ResolvedStyle {
            fg_color: crate::rgb!(248, 248, 242),
            bg_color: crate::rgb!(39, 40, 34),
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: crate::HighlightGroup) -> crate::Style {
        match group {
            // Control flow and operators
            crate::HighlightGroup::CtrlFlowKeyword
            | crate::HighlightGroup::PointerOper
            | crate::HighlightGroup::AssignOper
            | crate::HighlightGroup::BinaryOper
            | crate::HighlightGroup::OtherOper => crate::Style {
                fg_color: Some(PINK),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::OtherKeyword => crate::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            crate::HighlightGroup::FunctionDef
            | crate::HighlightGroup::TyDef
            | crate::HighlightGroup::InterfaceDef
            | crate::HighlightGroup::MacroDef => crate::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::FunctionCall
            | crate::HighlightGroup::MacroUse
            | crate::HighlightGroup::PreProc => crate::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::TyUse
            | crate::HighlightGroup::InterfaceUse
            | crate::HighlightGroup::PrimitiveTy => crate::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            // Variables, members and modules don’t get any highlighting
            crate::HighlightGroup::VariableDef
            | crate::HighlightGroup::VariableUse
            | crate::HighlightGroup::MemberDef
            | crate::HighlightGroup::MemberUse
            | crate::HighlightGroup::ModuleDef
            | crate::HighlightGroup::ModuleUse => crate::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::SpecialIdentDef | crate::HighlightGroup::SpecialIdentUse => {
                crate::Style {
                    fg_color: Some(PURPLE),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::FunctionParam => crate::Style {
                fg_color: Some(ORANGE),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            // Constants and literals (apart from strings)
            crate::HighlightGroup::ConstantDef
            | crate::HighlightGroup::ConstantUse
            | crate::HighlightGroup::Number
            | crate::HighlightGroup::Character
            | crate::HighlightGroup::CharacterDelimiter
            | crate::HighlightGroup::Boolean => crate::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::String | crate::HighlightGroup::StringDelimiter => {
                crate::Style {
                    fg_color: Some(YELLOW),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::Comment | crate::HighlightGroup::DocComment => crate::Style {
                fg_color: Some(FADED),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::Attribute => crate::Style {
                fg_color: Some(FADED),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::Error => crate::Style {
                fg_color: Some(PINK),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: true,
            },

            // Miscellaneous punctuation that doesn’t get highlighted
            crate::HighlightGroup::MemberOper
            | crate::HighlightGroup::Delimiter
            | crate::HighlightGroup::Separator
            | crate::HighlightGroup::Terminator => crate::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },
        }
    }
}
