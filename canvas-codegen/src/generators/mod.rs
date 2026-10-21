//! One module per output target.

pub mod css;
pub mod html;
pub mod json;
pub mod json_schema;
pub mod leptos;
pub mod markdown;
pub mod react;
pub mod svelte;
pub mod tailwind;
pub mod typescript;
pub mod vue;

pub use css::*;
pub use html::*;
pub use json::*;
pub use json_schema::*;
pub use leptos::*;
pub use markdown::*;
pub use react::*;
pub use svelte::*;
pub use tailwind::*;
pub use typescript::*;
pub use vue::*;
