use bevy::{ecs::system::NonSendMarker, winit::WINIT_WINDOWS};
use winit::window::Icon;

pub fn set_window_icon(_marker: NonSendMarker) {

    WINIT_WINDOWS.with_borrow_mut(|winit| {
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open("./assets/textures/icon_big.png")
                .expect("Failed to open icon path")
                .into_rgba8();

            let (width, height) = image.dimensions();
            let rgba = image.into_raw();

            (rgba, width, height)
        };

        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

        for window in winit.windows.values() {
            window.set_window_icon(Some(icon.clone()));
        }
    });

}