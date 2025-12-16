pub use flags::{
    generate_complete_bash,
    generate_complete_fish,
    generate_complete_powershell,
    generate_complete_zsh,
    generate_man_page,
};

#[cfg(feature = "cli")]
#[macro_use]
mod messages;

mod flags;

#[cfg(feature = "cli")]
mod haystack;
#[cfg(feature = "cli")]
mod logger;
#[cfg(feature = "cli")]
mod search;
