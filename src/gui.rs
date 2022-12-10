use glow::HasContext;
use imgui::Context;
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;

use sdl2::{
    event::Event,
    video::{GLProfile, Window},
};

// Create a new glow context.
fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

fn startup_gui() {
    let sdl = sdl2::init().unwrap();
    let video_sub_sysm = sdl.video().upwrap();
}


struct window()

struct render()
