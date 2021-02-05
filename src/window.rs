use bevy::{prelude::*, window::WindowMode};
// pub struct WindowDescriptor {
//     pub width: f32,
//     pub height: f32,
//     pub scale_factor_override: Option<f64>,
//     pub title: String,
//     pub vsync: bool,
//     pub resizable: bool,
//     pub decorations: bool,
//     pub cursor_visible: bool,
//     pub cursor_locked: bool,
//     pub mode: WindowMode,
//     #[cfg(target_arch = "wasm32")]
//     pub canvas: Option<String>,
// }

// pub enum WindowMode {
//     Windowed,
//     BorderlessFullscreen,
//     Fullscreen { use_size: bool },
// }

/// This system will then change the title during execution
fn change_title(time: Res<Time>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title(format!(
        "Seconds since startup: {}",
        time.seconds_since_startup().round()
    ));
}

/// This system toggles the cursor's visibility when the space bar is pressed
fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if input.just_pressed(KeyCode::Space) {
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
    }
}

#[derive(Default)]
pub struct PrevWindow(WindowDescriptor);

pub fn track_window(mut prev: ResMut<PrevWindow>, curr: Res<WindowDescriptor>) {
    prev.0 = curr.clone();
}

fn compare_f32(a: f32, b: f32) -> bool {
    (a - b).abs() < std::f32::EPSILON
}

pub fn compare_window(
    prev: Res<PrevWindow>,
    curr: Res<WindowDescriptor>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    let prev = &prev.0;
    // if prev.% != curr.% {
    //     window.set_%(curr.%)
    // }
    if compare_f32(prev.width, curr.width) || compare_f32(prev.height, curr.height) {
        window.set_resolution(curr.width, curr.height);
    }
    if prev.scale_factor_override != curr.scale_factor_override {
        window.set_scale_factor_override(curr.scale_factor_override);
    }
    if prev.title != curr.title {
        window.set_title(curr.title.clone());
    }
    if prev.vsync != curr.vsync {
        window.set_vsync(curr.vsync);
    }
    if prev.resizable != curr.resizable {
        window.set_resizable(curr.resizable);
    }
    if prev.decorations != curr.decorations {
        window.set_decorations(curr.decorations);
    }
    if prev.cursor_visible != curr.cursor_visible {
        window.set_cursor_visibility(curr.cursor_visible);
    }
    if prev.cursor_locked != curr.cursor_locked {
        window.set_cursor_lock_mode(curr.cursor_locked);
    }
    // if !matches!(prev.mode, curr.mode) {
    //     window.set_mode(curr.mode);
    // }
    match (prev.mode, curr.mode) {
        (WindowMode::Windowed, WindowMode::Windowed)
        | (WindowMode::BorderlessFullscreen, WindowMode::BorderlessFullscreen) => {}
        (WindowMode::Fullscreen { use_size: p }, WindowMode::Fullscreen { use_size: c }) => {
            if p != c {
                window.set_mode(curr.mode);
            }
        }
        _ => {
            window.set_mode(curr.mode);
        }
    }
    // there is no set_canvas method
    // #[cfg(target_arch = "wasm32")]
    // if prev.canvas != curr.canvas {
    //     window.set_canvas(curr.canvas);
    // }
}
