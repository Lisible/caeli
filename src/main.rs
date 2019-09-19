/*
* MIT License
*
* Copyright (c) 2019 Clément SIBILLE
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
use env_logger;

use tuber_window::{Window, WindowEvent};
use tuber_window_sdl2::{SDLContext, SDLWindow};

use tuber_graphics::GraphicsAPI;
use tuber_graphics_opengl::GLGraphicsAPI;

use tuber_scene::*;
use tuber_scene_renderer::*;

use nalgebra_glm as glm;
use tuber_scene::lights::PointLight;
use tuber_scene::shapes::Rectangle;

fn main() {
    env_logger::init();

    let context = SDLContext::new().unwrap();
    let mut window = SDLWindow::new(&context, "Cæli", 800, 600).unwrap();
    tuber_graphics_opengl::load_functions(|s| {
        context.video_subsystem().gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    let mut graphics = GLGraphicsAPI::new();
    graphics.set_clear_color((0.58, 0.73, 0.98));

    let mut scene = Scene::new();

    let mut camera = SceneNode::new(
        "camera",
        NodeValue::CameraNode(Camera::perspective(
            45f32.to_radians(),
            800f32 / 600f32,
            0.01f32,
            100f32,
        )),
    );
    let camera_transform = camera.transform_mut();
    camera_transform.set_translation(&glm::vec3(0.5, 0.3, 0.0));
    scene.set_active_camera("camera");

    let mut track = SceneNode::new("track", NodeValue::RectangleNode(Rectangle::new(1.0, 10.0)));
    track.transform_mut().set_rotation(&glm::vec3(80f32.to_radians(), 0.0, 0.0));
    track.transform_mut().set_translation(&glm::vec3(0.0, 0.0, 1.0));

    let mut light = SceneNode::new("light", NodeValue::PointLightNode(PointLight::default()));
    light.transform_mut().set_translation(&glm::vec3(0.0, 0.0, -5.0));

    scene.graph_mut().root_mut().add_child(light);
    scene.graph_mut().root_mut().add_child(camera);
    scene.graph_mut().root_mut().add_child(track);


    let mut scene_renderer = SceneRenderer::new(Box::new(graphics));

    'main_loop: loop {
        while let Some(event) = window.poll_event() {
            match event {
                WindowEvent::Close => break 'main_loop,
                _ => {}
            }
        }

        scene_renderer.render_scene(&scene);
        window.display();
    }
}
