/// This theme is an implementation of [Dracula](https://draculatheme.com), and follows [its
/// specification](https://spec.draculatheme.com) as closely as possible.
#[derive(Debug)]
pub struct Dracula;

const FADED: crate::Rgb = crate::rgb!(98, 114, 164);
const RED: crate::Rgb = crate::rgb!(255, 85, 85);
const ORANGE: crate::Rgb = crate::rgb!(255, 184, 108);
const YELLOW: crate::Rgb = crate::rgb!(241, 250, 140);
const GREEN: crate::Rgb = crate::rgb!(80, 250, 123);
const PURPLE: crate::Rgb = crate::rgb!(189, 147, 249);
const CYAN: crate::Rgb = crate::rgb!(139, 233, 253);
const PINK: crate::Rgb = crate::rgb!(255, 121, 198);

impl crate::Theme for Dracula {
    fn default_style(&self) -> crate::ResolvedStyle {
        crate::ResolvedStyle {
            fg_color: crate::rgb!(248, 248, 242),
            bg_color: crate::rgb!(40, 42, 54),
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
                    fg_color: Some(PINK),
                    bg_color: None,
                    is_bold: false,
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
            | crate::HighlightGroup::PrimitiveTy => crate::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Interfaces
            crate::HighlightGroup::InterfaceDef | crate::HighlightGroup::InterfaceUse => {
                crate::Style {
                    fg_color: Some(CYAN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: true,
                    is_underline: false,
                }
            }

            // Variables
            crate::HighlightGroup::VariableDef
            | crate::HighlightGroup::VariableUse
            | crate::HighlightGroup::MemberDef
            | crate::HighlightGroup::MemberUse => crate::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Function parameters
            crate::HighlightGroup::FunctionParam => crate::Style {
                fg_color: Some(ORANGE),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            // Constants
            crate::HighlightGroup::ConstantDef
            | crate::HighlightGroup::ConstantUse
            | crate::HighlightGroup::Number
            | crate::HighlightGroup::Boolean
            | crate::HighlightGroup::Character
            | crate::HighlightGroup::CharacterDelimiter => crate::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Modules
            crate::HighlightGroup::ModuleDef | crate::HighlightGroup::ModuleUse => crate::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Macros and other preprocessor-related highlight groups
            crate::HighlightGroup::MacroDef
            | crate::HighlightGroup::MacroUse
            | crate::HighlightGroup::PreProc => crate::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Attributes
            crate::HighlightGroup::Attribute => crate::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            // Strings
            crate::HighlightGroup::String | crate::HighlightGroup::StringDelimiter => {
                crate::Style {
                    fg_color: Some(YELLOW),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Special identifiers
            crate::HighlightGroup::SpecialIdentDef | crate::HighlightGroup::SpecialIdentUse => {
                crate::Style {
                    fg_color: Some(GREEN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Comments
            crate::HighlightGroup::Comment | crate::HighlightGroup::DocComment => crate::Style {
                fg_color: Some(FADED),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Some punctuation gets a colour
            crate::HighlightGroup::PointerOper
            | crate::HighlightGroup::AssignOper
            | crate::HighlightGroup::BinaryOper
            | crate::HighlightGroup::OtherOper
            | crate::HighlightGroup::Separator => crate::Style {
                fg_color: Some(PINK),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Other punctuation doesn’t
            crate::HighlightGroup::MemberOper
            | crate::HighlightGroup::Delimiter
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
                is_underline: false,
            },
        }
    }
}
