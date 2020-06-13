/// This theme tries to emulate the one in VS Code, but includes some of the extra colours used in
/// Visual Studio.
#[derive(Debug)]
pub struct DarkPlus;

const DARK_BLUE: crate::Rgb = crate::rgb!(86, 156, 214);
const DARK_GREEN: crate::Rgb = crate::rgb!(107, 153, 85);
const DULL_GREEN_DARKER: crate::Rgb = crate::rgb!(181, 206, 168);
const FADED: crate::Rgb = crate::rgb!(178, 178, 178);
const GREEN: crate::Rgb = crate::rgb!(134, 198, 145);
const DULL_GREEN: crate::Rgb = crate::rgb!(184, 215, 163);
const LIGHT_BLUE: crate::Rgb = crate::rgb!(156, 220, 254);
const ORANGE: crate::Rgb = crate::rgb!(206, 144, 120);
const PURPLE: crate::Rgb = crate::rgb!(197, 134, 192);
const RED: crate::Rgb = crate::rgb!(244, 71, 71);
const TEAL: crate::Rgb = crate::rgb!(78, 201, 176);
const YELLOW: crate::Rgb = crate::rgb!(220, 220, 170);

impl crate::Theme for DarkPlus {
    fn default_style(&self) -> crate::ResolvedStyle {
        crate::ResolvedStyle {
            fg_color: crate::rgb!(212, 212, 212),
            bg_color: crate::rgb!(30, 30, 30),
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: crate::HighlightGroup) -> crate::Style {
        match group {
            crate::HighlightGroup::CtrlFlowKeyword => crate::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Keywords and things that are often treated as such
            crate::HighlightGroup::OtherKeyword
            | crate::HighlightGroup::PrimitiveTy
            | crate::HighlightGroup::Boolean => crate::Style {
                fg_color: Some(DARK_BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Call-able things
            crate::HighlightGroup::FunctionDef
            | crate::HighlightGroup::FunctionCall
            | crate::HighlightGroup::MacroDef
            | crate::HighlightGroup::MacroUse => crate::Style {
                fg_color: Some(YELLOW),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::TyDef | crate::HighlightGroup::TyUse => crate::Style {
                fg_color: Some(TEAL),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::InterfaceDef | crate::HighlightGroup::InterfaceUse => {
                crate::Style {
                    fg_color: Some(DULL_GREEN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            crate::HighlightGroup::VariableDef
            | crate::HighlightGroup::VariableUse
            | crate::HighlightGroup::MemberDef
            | crate::HighlightGroup::MemberUse
            | crate::HighlightGroup::ConstantDef
            | crate::HighlightGroup::ConstantUse
            | crate::HighlightGroup::FunctionParam => crate::Style {
                fg_color: Some(LIGHT_BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::SpecialIdentDef | crate::HighlightGroup::SpecialIdentUse => {
                crate::Style {
                    // This colour is actually used for structs, but the distinction between
                    // structs and other types is only possible through semantic highlighting -- it
                    // is expected that all highlighters will either be simple lexers or parsers.
                    //
                    // Since ‘special identifiers’ are unique in the languages that they occur in
                    // (e.g.  lifetimes in Rust, symbols in Ruby), it makes sense to give them a
                    // special colour. This colour was left over, so I decided to use it.
                    fg_color: Some(GREEN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Modules aren’t highlighted
            crate::HighlightGroup::ModuleDef | crate::HighlightGroup::ModuleUse => crate::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::Number => crate::Style {
                fg_color: Some(DULL_GREEN_DARKER),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::String
            | crate::HighlightGroup::StringDelimiter
            | crate::HighlightGroup::Character
            | crate::HighlightGroup::CharacterDelimiter => crate::Style {
                fg_color: Some(ORANGE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            crate::HighlightGroup::PreProc => crate::Style {
                fg_color: Some(DARK_BLUE),
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

            crate::HighlightGroup::Comment | crate::HighlightGroup::DocComment => crate::Style {
                fg_color: Some(DARK_GREEN),
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
                fg_color: Some(FADED),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

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
