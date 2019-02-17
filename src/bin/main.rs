/*
* MIT License
*
* Copyright (c) 2018 Clément SIBILLE
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

use std::rc::Rc;
use std::cell::RefCell;

use tuber::window::{Window, WindowEvent};
use tuber::input::keyboard::Key;

use tuber_window_sdl2::SDLWindow;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let sdl_video = sdl_context.video()?;
    let sdl_event_pump = Rc::new(RefCell::new(sdl_context.event_pump()?));

    let mut window = SDLWindow::new(&sdl_video, sdl_event_pump.clone());

    'main_loop: loop {
        while let Some(event) = window.poll_event() {
            match event {
                WindowEvent::Close |
                WindowEvent::KeyDown(Key::Escape) => break 'main_loop,
                _ => {}
            }
        }
    }

    Ok(())
}
