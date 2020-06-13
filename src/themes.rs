//! Some common themes, pre-implemented for convenience.

mod dark_plus;
mod dracula;
mod gruvbox;
mod monokai;
mod nord;
mod seoul;
mod solarized;

pub use {
    dark_plus::DarkPlus,
    dracula::Dracula,
    gruvbox::Gruvbox,
    monokai::Monokai,
    nord::Nord,
    seoul::Seoul,
    solarized::{SolarizedDark, SolarizedLight},
};
