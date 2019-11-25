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

use tuber::window::{Window, WindowEvent};
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

    
    let track = Track::new(4);
    let renderer = Renderer{};
    renderer.set_clear_color(1.0, 0.0, 0.0);
    
    'main_loop: loop {
        while let Some(event) = window.poll_event() {
            match event {
                WindowEvent::Close => break 'main_loop,
                _ => {}
            }
        }

        renderer.clear();
        renderer.draw_rectangle(0.0, 0.0, 800.0, 600.0);
        renderer.render();
        window.display();
    }
}
