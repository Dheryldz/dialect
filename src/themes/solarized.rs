const BASE03: crate::Rgb = crate::rgb!(0, 43, 54);
const BASE01: crate::Rgb = crate::rgb!(88, 110, 117);
const BASE00: crate::Rgb = crate::rgb!(101, 123, 131);
const BASE0: crate::Rgb = crate::rgb!(131, 148, 150);
const BASE1: crate::Rgb = crate::rgb!(147, 161, 161);
const BASE3: crate::Rgb = crate::rgb!(253, 246, 227);
const YELLOW: crate::Rgb = crate::rgb!(181, 137, 0);
const ORANGE: crate::Rgb = crate::rgb!(203, 75, 22);
const RED: crate::Rgb = crate::rgb!(220, 50, 47);
const BLUE: crate::Rgb = crate::rgb!(38, 139, 210);
const CYAN: crate::Rgb = crate::rgb!(42, 161, 152);
const GREEN: crate::Rgb = crate::rgb!(133, 153, 0);

macro_rules! create_solarized_theme {
    ($theme_name: ident, $fg: expr, $bg: expr, $deemphasized: expr) => {
        /// An implementation of [Solarized](https://ethanschoonover.com/solarized/). Delimiters
        /// are not highlighted as red anymore. This is because Vim syntax configurations very
        /// rarely highlight delimiters, leaving most delimiters unhighlighted in the original
        /// Solarized theme. It turned out that highlighting all delimiters in red is very
        /// distracting (and in practice different from the orginal), which is why it was decided
        /// not to highlight them.
        #[derive(Debug)]
        pub struct $theme_name;

        impl crate::Theme for $theme_name {
            fn default_style(&self) -> crate::ResolvedStyle {
                crate::ResolvedStyle {
                    fg_color: $fg,
                    bg_color: $bg,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            fn style(&self, group: crate::HighlightGroup) -> crate::Style {
                match group {
                    crate::HighlightGroup::CtrlFlowKeyword
                    | crate::HighlightGroup::OtherKeyword => crate::Style {
                        fg_color: Some(GREEN),
                        bg_color: None,
                        is_bold: false,
                        is_italic: false,
                        is_underline: false,
                    },

                    // ‘Identifiers’ (functions, variables, modules)
                    crate::HighlightGroup::FunctionDef
                    | crate::HighlightGroup::FunctionCall
                    | crate::HighlightGroup::VariableDef
                    | crate::HighlightGroup::VariableUse
                    | crate::HighlightGroup::MemberDef
                    | crate::HighlightGroup::MemberUse
                    | crate::HighlightGroup::SpecialIdentDef
                    | crate::HighlightGroup::SpecialIdentUse
                    | crate::HighlightGroup::FunctionParam
                    | crate::HighlightGroup::ModuleDef
                    | crate::HighlightGroup::ModuleUse => crate::Style {
                        fg_color: Some(BLUE),
                        bg_color: None,
                        is_bold: false,
                        is_italic: false,
                        is_underline: false,
                    },

                    // Constants of any kind
                    crate::HighlightGroup::ConstantDef
                    | crate::HighlightGroup::ConstantUse
                    | crate::HighlightGroup::Number
                    | crate::HighlightGroup::String
                    | crate::HighlightGroup::StringDelimiter
                    | crate::HighlightGroup::Character
                    | crate::HighlightGroup::CharacterDelimiter
                    | crate::HighlightGroup::Boolean => crate::Style {
                        fg_color: Some(CYAN),
                        bg_color: None,
                        is_bold: false,
                        is_italic: false,
                        is_underline: false,
                    },

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

                    crate::HighlightGroup::PreProc
                    | crate::HighlightGroup::MacroDef
                    | crate::HighlightGroup::MacroUse => crate::Style {
                        fg_color: Some(ORANGE),
                        bg_color: None,
                        is_bold: false,
                        is_italic: false,
                        is_underline: false,
                    },

                    // Punctuation gets no highlighting
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

                    crate::HighlightGroup::Comment | crate::HighlightGroup::DocComment => {
                        crate::Style {
                            fg_color: Some($deemphasized),
                            bg_color: None,
                            is_bold: false,
                            is_italic: true,
                            is_underline: false,
                        }
                    }

                    crate::HighlightGroup::Attribute => crate::Style {
                        fg_color: Some(GREEN),
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
    };
}

create_solarized_theme!(SolarizedLight, BASE00, BASE3, BASE1);
create_solarized_theme!(SolarizedDark, BASE0, BASE03, BASE01);
