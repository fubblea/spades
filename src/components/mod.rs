//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.
mod big_button;
pub use big_button::BigButton;

mod typing;
pub use typing::Title;

mod incrementer;
pub use incrementer::Incrementer;

mod title_bar;
pub use title_bar::TitleBar;

mod team;
pub use team::TeamComponent;
