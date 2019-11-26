/*
* MIT License
*
* Copyright (c) 2019 Caeli team
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

use tuber::window::{Window, WindowEvent, input::keyboard::Key};
use tuber::platform::sdl2::{SDLWindow, SDLContext};

use gl;

use caeli::Track;
use caeli::graphics::Renderer;

fn main() {
    let context = SDLContext::new().unwrap();
    let mut window = SDLWindow::new(&context, "Cæli", 800, 600).unwrap();
    gl::load_with(|s| {
        context.video_subsystem().gl_get_proc_address(s) as *const std::ffi::c_void 
    });
    
    let mut track = Track::new(9);
    let mut renderer = Renderer::new();
    renderer.set_clear_color(1.0, 0.0, 0.0);

    'main_loop: loop {
        while let Some(event) = window.poll_event() {
            match event {
                WindowEvent::Close => break 'main_loop,
                WindowEvent::KeyDown(Key::A) => track.activate_section(0),
                WindowEvent::KeyDown(Key::Z) => track.activate_section(1),
                WindowEvent::KeyDown(Key::E) => track.activate_section(2),
                WindowEvent::KeyDown(Key::R) => track.activate_section(3),
                WindowEvent::KeyDown(Key::T) => track.activate_section(4),
                WindowEvent::KeyDown(Key::Y) => track.activate_section(5),
                WindowEvent::KeyDown(Key::U) => track.activate_section(6),
                WindowEvent::KeyDown(Key::I) => track.activate_section(7),
                WindowEvent::KeyDown(Key::O) => track.activate_section(8),
                WindowEvent::KeyDown(Key::P) => track.activate_section(9),
                WindowEvent::KeyUp(Key::A) => track.deactivate_section(0),
                WindowEvent::KeyUp(Key::Z) => track.deactivate_section(1),
                WindowEvent::KeyUp(Key::E) => track.deactivate_section(2),
                WindowEvent::KeyUp(Key::R) => track.deactivate_section(3),
                WindowEvent::KeyUp(Key::T) => track.deactivate_section(4),
                WindowEvent::KeyUp(Key::Y) => track.deactivate_section(5),
                WindowEvent::KeyUp(Key::U) => track.deactivate_section(6),
                WindowEvent::KeyUp(Key::I) => track.deactivate_section(7),
                WindowEvent::KeyUp(Key::O) => track.deactivate_section(8),
                WindowEvent::KeyUp(Key::P) => track.deactivate_section(9),
                _ => {}
            }
        }

        renderer.clear();
        render_track(&track, &mut renderer);
        renderer.render();
        window.display();
    }
}

const SCREEN_WIDTH : f32 = 2.0;
const SCREEN_HEIGHT : f32 = 2.0;
const SCREEN_ORIGIN_X : f32 = -1.0;
const SCREEN_ORIGIN_Y : f32 = -1.0;

fn render_track(track: &Track, renderer: &mut Renderer) {
    let x_offset = SCREEN_WIDTH / track.section_count() as f32;

    for i in 0..track.section_count() {
        let color = if track.is_activated(i) {(0.5, 0.5, 0.5)} else {(0.2, 0.2, 0.2)};
        renderer.draw_rectangle(SCREEN_ORIGIN_X + x_offset*i as f32, SCREEN_ORIGIN_Y, x_offset, SCREEN_HEIGHT, color);
        renderer.draw_line(
            (SCREEN_ORIGIN_X + x_offset*i as f32, SCREEN_ORIGIN_Y),
            (SCREEN_ORIGIN_X + x_offset*i as f32, SCREEN_ORIGIN_Y + SCREEN_HEIGHT),
            (1.0, 1.0, 1.0)
        );
    }
}
