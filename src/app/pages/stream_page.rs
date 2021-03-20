use cursive::views::{TextView, LinearLayout, NamedView};
use crate::app::components::container::aeronitium_container;


pub fn build_stream() -> NamedView<LinearLayout> {
    let widget = aeronitium_container(
        TextView::new("Halaman Simulasi Tekanan"),
        |_| {}
    );

    widget
}