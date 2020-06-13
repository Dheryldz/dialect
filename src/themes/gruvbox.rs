/// This theme is based on the [Gruvbox](https://github.com/morhetz/gruvbox) Dark Hard theme, with
/// the small change of not highlighting functions in bold, while highlighting keywords in bold.
#[derive(Debug)]
pub struct Gruvbox;

const GRAY: crate::Rgb = crate::rgb!(189, 174, 147);
const RED: crate::Rgb = crate::rgb!(251, 73, 52);
const GREEN: crate::Rgb = crate::rgb!(184, 187, 38);
const YELLOW: crate::Rgb = crate::rgb!(250, 189, 47);
const BLUE: crate::Rgb = crate::rgb!(131, 165, 152);
const PURPLE: crate::Rgb = crate::rgb!(211, 134, 155);
const AQUA: crate::Rgb = crate::rgb!(142, 192, 124);
const ORANGE: crate::Rgb = crate::rgb!(254, 128, 25);

impl crate::Theme for Gruvbox {
    fn default_style(&self) -> crate::ResolvedStyle {
        crate::ResolvedStyle {
            fg_color: crate::rgb!(235, 219, 178),
            bg_color: crate::rgb!(29, 32, 33),
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: crate::HighlightGroup) -> crate::Style {
        match group {
            // Keywords
            crate::HighlightGroup::CtrlFlowKeyword | crate::HighlightGroup::OtherKeyword => {
                crate::Style {
                    fg_color: Some(RED),
                    bg_color: None,
                    is_bold: true,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Functions
            crate::HighlightGroup::FunctionDef | crate::HighlightGroup::FunctionCall => {
                crate::Style {
                    fg_color: Some(GREEN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Types
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

            // Variables
            crate::HighlightGroup::VariableDef
            | crate::HighlightGroup::VariableUse
            | crate::HighlightGroup::MemberDef
            | crate::HighlightGroup::MemberUse
            | crate::HighlightGroup::FunctionParam => crate::Style {
                fg_color: Some(BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Constants
            crate::HighlightGroup::ConstantDef
            | crate::HighlightGroup::ConstantUse
            | crate::HighlightGroup::Number
            | crate::HighlightGroup::Boolean => crate::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Modules
            crate::HighlightGroup::ModuleDef | crate::HighlightGroup::ModuleUse => crate::Style {
                fg_color: Some(BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Preprocessor-related
            crate::HighlightGroup::MacroDef
            | crate::HighlightGroup::MacroUse
            | crate::HighlightGroup::PreProc
            | crate::HighlightGroup::Attribute => crate::Style {
                fg_color: Some(AQUA),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // String and character literals
            crate::HighlightGroup::String
            | crate::HighlightGroup::StringDelimiter
            | crate::HighlightGroup::Character
            | crate::HighlightGroup::CharacterDelimiter => crate::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Special identifiers
            crate::HighlightGroup::SpecialIdentDef | crate::HighlightGroup::SpecialIdentUse => {
                crate::Style {
                    fg_color: Some(ORANGE),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Comments
            crate::HighlightGroup::Comment | crate::HighlightGroup::DocComment => crate::Style {
                fg_color: Some(GRAY),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Punctuation
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

            // Errors
            crate::HighlightGroup::Error => crate::Style {
                fg_color: Some(RED),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: true,
            },
        }
    }
}
