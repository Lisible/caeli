/*
* MIT License
*
* Copyright (c) 2019 Tuber team
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

use tuber::platform::sdl2::{SDLContext, SDLWindow};
use tuber::window::{input::keyboard::Key, Window, WindowEvent};

use tuber::graphics::{GraphicsAPI, Material, Mesh, PointLight};
use tuber::platform::opengl::GLGraphicsAPI;

use tuber::graphics::scene_renderer::*;
use tuber::scene::*;

use tuber::audio::AudioAPI;
use tuber::platform::sdl2::audio::SDLAudioAPI;

use std::time::Instant;

use nalgebra_glm as glm;

use caeli::Track;

struct Game {
    track: Track,
    score: usize,
    // song ?
}

impl Game {
    pub fn new() -> Game {
        let mut new_track = Track::new("track", 3);
        for i in 0..50 {
            new_track.add_note(i * 1000, 0, 1);
            //new_track.add_note(i*5, i%3, 1);
        }

        Game {
            track: new_track,
            score: 0,
        }
    }

    pub fn handle_input(&mut self, event: WindowEvent, scene : &mut Scene, time:f32){
            match event {
                //FIXME(quentin) : ugly stuff, may use some kind of global timer instead of passing this to the function
                WindowEvent::KeyDown(Key::A) => self.track.activate_lane(0, scene, (time * 1000.0) as usize),
                WindowEvent::KeyDown(Key::Z) => self.track.activate_lane(1, scene, (time * 1000.0) as usize),
                WindowEvent::KeyDown(Key::E) => self.track.activate_lane(2, scene, (time * 1000.0) as usize),
                WindowEvent::KeyDown(Key::R) => self.track.activate_lane(3, scene, (time * 1000.0) as usize),
                WindowEvent::KeyDown(Key::T) => self.track.activate_lane(4, scene, (time * 1000.0) as usize),
                WindowEvent::KeyDown(Key::Y) => self.track.activate_lane(5, scene, (time * 1000.0) as usize),
                WindowEvent::KeyDown(Key::U) => self.track.activate_lane(6, scene, (time * 1000.0) as usize),
                WindowEvent::KeyDown(Key::I) => self.track.activate_lane(7, scene, (time * 1000.0) as usize),
                WindowEvent::KeyDown(Key::O) => self.track.activate_lane(8, scene, (time * 1000.0) as usize),
                WindowEvent::KeyDown(Key::P) => self.track.activate_lane(9, scene, (time * 1000.0) as usize),
                WindowEvent::KeyUp(Key::A) => self.track.deactivate_lane(0, scene),
                WindowEvent::KeyUp(Key::Z) => self.track.deactivate_lane(1, scene),
                WindowEvent::KeyUp(Key::E) => self.track.deactivate_lane(2, scene),
                WindowEvent::KeyUp(Key::R) => self.track.deactivate_lane(3, scene),
                WindowEvent::KeyUp(Key::T) => self.track.deactivate_lane(4, scene),
                WindowEvent::KeyUp(Key::Y) => self.track.deactivate_lane(5, scene),
                WindowEvent::KeyUp(Key::U) => self.track.deactivate_lane(6, scene),
                WindowEvent::KeyUp(Key::I) => self.track.deactivate_lane(7, scene),
                WindowEvent::KeyUp(Key::O) => self.track.deactivate_lane(8, scene),
                WindowEvent::KeyUp(Key::P) => self.track.deactivate_lane(9, scene),
                _ => {}
            }
    }

    pub fn update(&mut self, delta_time: f32, scene: &mut Scene) {
        self.track.update(delta_time, scene);
    }

    pub fn create_node(&self) -> SceneNode {
        self.track.create_node()
    }
}

fn main() {
    env_logger::init();

    let context = SDLContext::new().unwrap();
    let mut window = SDLWindow::new(&context, "Bonjour", 800, 600).unwrap();
    tuber::platform::opengl::load_functions(|s| {
        context.video_subsystem().gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    let mut graphics = GLGraphicsAPI::new();
    graphics.set_bounding_box_visibility(false);
    graphics.set_clear_color((0.58, 0.73, 0.98));

    let mut scene = Scene::new();
    let mut camera = SceneNode::new(
        "camera",
        NodeValue::CameraNode(Camera::perspective(
            120f32.to_radians(),
            800.0 / 600.0,
            0.1,
            100.0,
        )),
    );

    let mut audio = SDLAudioAPI::new();
    audio.play_music("music");
    audio.seek_music(std::time::Duration::from_secs_f32(30.0));

    let camera_transform = camera.transform_mut();
    camera_transform.set_translation(&glm::vec3(1.5, 1.25, 1.25));
    camera_transform.set_rotation(&glm::vec3(40f32.to_radians(), std::f32::consts::PI, 0.0));
    scene.set_active_camera("camera");

    let mut game = Game::new();

    let light_node = create_light();

    scene.graph_mut().root_mut().add_child(light_node);
    scene.graph_mut().root_mut().add_child(camera);
    scene.graph_mut().root_mut().add_child(game.create_node());

    let mut scene_renderer = SceneRenderer::new(Box::new(graphics));

    const FRAMERATE: u32 = 60;
    const TICKS_PER_FRAME: u32 = 1000 / FRAMERATE;

    let timestep_timer = Instant::now();
    let mut last_frame_time = timestep_timer.elapsed().as_secs_f32();

    'main_loop: loop {
        let cap_timer = Instant::now();

        let current_time = timestep_timer.elapsed().as_secs_f32();
        let timestep = current_time - last_frame_time;
        last_frame_time = current_time;

        while let Some(event) = window.poll_event() {
            match event {
                WindowEvent::Close | WindowEvent::KeyDown(Key::Escape) => break 'main_loop,
                WindowEvent::KeyDown(Key::Q) => audio.play_sound("sound"),
                WindowEvent::KeyDown(Key::S) => {
                    audio.stop_music();
                },
                _ => game.handle_input(event, &mut scene, current_time)
           }
        }

        game.update(timestep, &mut scene);

        scene_renderer.render_scene(&scene);
        window.display();

        let frame_ticks = cap_timer.elapsed().as_millis();
        if frame_ticks < TICKS_PER_FRAME as u128 {
            std::thread::sleep(std::time::Duration::from_millis(TICKS_PER_FRAME as u64 - frame_ticks as u64));
        }
    }
}

pub fn create_light() -> SceneNode {
    let light = PointLight {
        ambient: (0.5, 0.5, 0.5),
        diffuse: (0.7, 0.7, 0.7),
        specular: (0.7, 0.7, 0.7),
        constant: 1.0,
        linear: 0.00014,
        quadratic: 0.00007,
    };

    let mut light_node = SceneNode::new("light", NodeValue::PointLightNode(light));
    light_node
        .transform_mut()
        .set_translation(&glm::vec3(0.0, 9.0, 2.0));
    light_node
}
