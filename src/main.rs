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

use std::time::Instant;

use nalgebra_glm as glm;

use caeli::Track;

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

    let camera_transform = camera.transform_mut();
    camera_transform.set_translation(&glm::vec3(1.5, 1.25, 1.25));
    camera_transform.set_rotation(&glm::vec3(40f32.to_radians(), std::f32::consts::PI, 0.0));
    scene.set_active_camera("camera");


    let mut track = Track::new("track", 8);
    for i in 0..1000 {
        track.add_note(i*16, 4, 1);
        track.add_note(i*8, 3, 1);
        track.add_note(i*4, 2, 1);
        track.add_note(i*2, 1, 1);
        track.add_note(i, 0, 1);
    }

    let light_node = create_light();

    scene.graph_mut().root_mut().add_child(light_node);
    scene.graph_mut().root_mut().add_child(camera);
    scene.graph_mut().root_mut().add_child(track.create_node());

    let mut scene_renderer = SceneRenderer::new(Box::new(graphics));

    let timestep_timer = Instant::now();
    let mut last_frame_time = timestep_timer.elapsed().as_secs_f32();

    'main_loop: loop {
        let current_time = timestep_timer.elapsed().as_secs_f32();
        let timestep = current_time - last_frame_time;
        last_frame_time = current_time;

        while let Some(event) = window.poll_event() {
            match event {
                WindowEvent::Close | WindowEvent::KeyDown(Key::Escape) => break 'main_loop,
                WindowEvent::KeyDown(Key::A) => track.activate_lane(0, &mut scene),
                WindowEvent::KeyDown(Key::Z) => track.activate_lane(1, &mut scene),
                WindowEvent::KeyDown(Key::E) => track.activate_lane(2, &mut scene),
                WindowEvent::KeyDown(Key::R) => track.activate_lane(3, &mut scene),
                WindowEvent::KeyDown(Key::T) => track.activate_lane(4, &mut scene),
                WindowEvent::KeyDown(Key::Y) => track.activate_lane(5, &mut scene),
                WindowEvent::KeyDown(Key::U) => track.activate_lane(6, &mut scene),
                WindowEvent::KeyDown(Key::I) => track.activate_lane(7, &mut scene),
                WindowEvent::KeyDown(Key::O) => track.activate_lane(8, &mut scene),
                WindowEvent::KeyDown(Key::P) => track.activate_lane(9, &mut scene),
                WindowEvent::KeyUp(Key::A) => track.deactivate_lane(0, &mut scene),
                WindowEvent::KeyUp(Key::Z) => track.deactivate_lane(1, &mut scene),
                WindowEvent::KeyUp(Key::E) => track.deactivate_lane(2, &mut scene),
                WindowEvent::KeyUp(Key::R) => track.deactivate_lane(3, &mut scene),
                WindowEvent::KeyUp(Key::T) => track.deactivate_lane(4, &mut scene),
                WindowEvent::KeyUp(Key::Y) => track.deactivate_lane(5, &mut scene),
                WindowEvent::KeyUp(Key::U) => track.deactivate_lane(6, &mut scene),
                WindowEvent::KeyUp(Key::I) => track.deactivate_lane(7, &mut scene),
                WindowEvent::KeyUp(Key::O) => track.deactivate_lane(8, &mut scene),
                WindowEvent::KeyUp(Key::P) => track.deactivate_lane(9, &mut scene),
                _ => {}
            }
        }


        track.update(0.016 as f32, &mut scene);

        scene_renderer.render_scene(&scene);
        window.display();
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
