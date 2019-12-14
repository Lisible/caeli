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

use tuber::window::{Window, WindowEvent, input::keyboard::Key};
use tuber::platform::sdl2::{SDLContext, SDLWindow};

use tuber::graphics::{GraphicsAPI, Mesh, Material};
use tuber::platform::opengl::GLGraphicsAPI;

use tuber::scene::*;
use tuber::graphics::scene_renderer::*;

use nalgebra_glm as glm;

fn main() {
    env_logger::init();

    let context = SDLContext::new().unwrap();
    let mut window = SDLWindow::new(&context, "Bonjour", 800, 600).unwrap();
    tuber::platform::opengl::load_functions(|s| {
        context.video_subsystem().gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    let mut graphics = GLGraphicsAPI::new();
    graphics.set_clear_color((0.58, 0.73, 0.98));

    let mut scene = Scene::new();

    let mut camera = SceneNode::new(
        "camera",
        NodeValue::CameraNode(Camera::perspective(
                22f32.to_radians(),
                800.0 / 600.0,
                0.1,
                100.0
        ))
    );
    let camera_transform = camera.transform_mut();
    camera_transform.set_translation(&glm::vec3(0.0, -2.0, 2.0));
    camera_transform.set_rotation(&glm::vec3(75f32.to_radians(), std::f32::consts::PI, 0.0));
    scene.set_active_camera("camera");

    let mut track_node = SceneNode::new("track", NodeValue::AbstractNode);
    const LANE_MATERIAL: Material = Material {
        diffuse: (0.0, 0.0, 0.0),
        specular: (0.0, 0.0, 0.0),
        shininess: 32.0,
        texture: None
    };

    let lane_count = 6;
    let horizontal_scale = 2.0 / lane_count as f32;
    for i in 0..lane_count {
        let lane_node_name = format!("lane_{}", i.to_string());
        let mut lane_node = SceneNode::new(lane_node_name.as_str(), NodeValue::MeshNode(Mesh::new_plane_mesh(LANE_MATERIAL.clone())));
        let lane_node_transform = lane_node.transform_mut();

        lane_node_transform.set_scale(&glm::vec3(horizontal_scale, 100.0, 1.0));
        lane_node_transform.set_translation(&glm::vec3(i as f32 * horizontal_scale, 0.0, 0.0));
        track_node.add_child(lane_node);
    }

    let note_material: Material = Material {
        diffuse: (1.0, 1.0, 1.0),
        specular: (1.0, 1.0, 1.0),
        shininess: 2048.0,
        texture: Some("note_texture".to_owned())
    };

    let mut notes_node = SceneNode::new("notes", NodeValue::AbstractNode);

    for i in 0..20 {
        let mut note_node = SceneNode::new(format!("note_{}", i).as_str(), NodeValue::MeshNode(Mesh::new_plane_mesh(note_material.clone())));
        note_node.transform_mut().set_translation(&glm::vec3((i % lane_count) as f32 * horizontal_scale, i as f32, 0.001));
        note_node.transform_mut().set_scale(&glm::vec3(horizontal_scale, 0.3, 1.0));
        notes_node.add_child(note_node);
    }
    track_node.add_child(notes_node);


    const DETECTION_BAR_MATERIAL: Material = Material {
        diffuse: (1.0, 1.0, 0.0),
        specular: (1.0, 1.0, 0.0),
        shininess: 32.0,
        texture: None
    };
    let mut detection_bar = SceneNode::new("detection_bar", NodeValue::MeshNode(Mesh::new_plane_mesh(DETECTION_BAR_MATERIAL.clone())));
    detection_bar.transform_mut().set_translation(&glm::vec3(0.0, 2.0, 0.002));
    detection_bar.transform_mut().set_scale(&glm::vec3(2.0, 0.25, 1.0));
    track_node.add_child(detection_bar);


    let track_transform = track_node.transform_mut();
    track_transform.set_translation(&glm::vec3(-horizontal_scale*(lane_count as f32 / 2.0), 0.0, 0.0));
    track_transform.set_rotation(&glm::vec3(0.0, 0.0, 0.0));
    track_transform.set_scale(&glm::vec3(1.0, 1.0, 1.0));

    let mut light = tuber::scene::lights::PointLight::default();
    light.set_ambient_color((0.5, 0.5, 0.5));
    light.set_diffuse_color((0.7, 0.7, 0.7));
    light.set_specular_color((0.7, 0.7, 0.7));
    light.set_attenuation((1.0, 0.0014, 0.000007));

    let mut light_node = SceneNode::new("light", NodeValue::PointLightNode(light));
    light_node.transform_mut().set_translation(&glm::vec3(0.0, 9.0, 2.0));

    scene.graph_mut().root_mut().add_child(light_node);
    scene.graph_mut().root_mut().add_child(camera);
    scene.graph_mut().root_mut().add_child(track_node);


    let mut scene_renderer = SceneRenderer::new(Box::new(graphics));

    'main_loop: loop {
        while let Some(event) = window.poll_event() {
            match event {
                WindowEvent::Close | WindowEvent::KeyDown(Key::Escape) => break 'main_loop,
                WindowEvent::KeyDown(Key::A) => activate_lane("lane_0", &mut scene),
                WindowEvent::KeyDown(Key::Z) => activate_lane("lane_1", &mut scene),
                WindowEvent::KeyDown(Key::E) => activate_lane("lane_2", &mut scene),
                WindowEvent::KeyDown(Key::R) => activate_lane("lane_3", &mut scene),
                WindowEvent::KeyDown(Key::T) => activate_lane("lane_4", &mut scene),
                WindowEvent::KeyDown(Key::Y) => activate_lane("lane_5", &mut scene),
                WindowEvent::KeyDown(Key::U) => activate_lane("lane_6", &mut scene),
                WindowEvent::KeyDown(Key::I) => activate_lane("lane_7", &mut scene),
                WindowEvent::KeyDown(Key::O) => activate_lane("lane_8", &mut scene),
                WindowEvent::KeyDown(Key::P) => activate_lane("lane_9", &mut scene),
                WindowEvent::KeyUp(Key::A) => deactivate_lane("lane_0", &mut scene),
                WindowEvent::KeyUp(Key::Z) => deactivate_lane("lane_1", &mut scene),
                WindowEvent::KeyUp(Key::E) => deactivate_lane("lane_2", &mut scene),
                WindowEvent::KeyUp(Key::R) => deactivate_lane("lane_3", &mut scene),
                WindowEvent::KeyUp(Key::T) => deactivate_lane("lane_4", &mut scene),
                WindowEvent::KeyUp(Key::Y) => deactivate_lane("lane_5", &mut scene),
                WindowEvent::KeyUp(Key::U) => deactivate_lane("lane_6", &mut scene),
                WindowEvent::KeyUp(Key::I) => deactivate_lane("lane_7", &mut scene),
                WindowEvent::KeyUp(Key::O) => deactivate_lane("lane_8", &mut scene),
                WindowEvent::KeyUp(Key::P) => deactivate_lane("lane_9", &mut scene),
                _ => {}
            }
        }

        let notes_node = scene.graph_mut().root_mut().find_mut("notes").unwrap();
        notes_node.transform_mut().translate(&glm::vec3(0.0, -0.05, 0.0));

        let mut notes_to_remove = vec!();
        let y = notes_node.transform().translation().y;
        for (index, note) in notes_node.children_mut().iter().enumerate() {
            if y + note.transform().translation().y < 0.0 {
                notes_to_remove.push(index);
            }
        }

        for index in notes_to_remove {
            notes_node.remove_child(index);
        }



        scene_renderer.render_scene(&scene);
        window.display();
    }
}

pub fn activate_lane(lane_id: &str, scene: &mut Scene) {
   let lane_node = scene.graph_mut().root_mut().find_mut(lane_id).unwrap();
   if let NodeValue::MeshNode(mesh) = lane_node.value_mut() {
       mesh.material.diffuse = (0.3, 0.3, 0.3);
   }

   let notes_node = scene.graph_mut().root_mut().find_mut("notes").unwrap();
   let current_y = notes_node.transform().translation().y;
   for (_, node) in notes_node.children_mut().iter().enumerate() {
       let lane = (node.transform().translation().x / (2.0 / 6.0)).round() as i32;
       let activated_lane = lane_id.split("_").nth(1).unwrap().parse::<i32>().unwrap();
       let position_y = current_y + node.transform().translation().y;
       if position_y > 1.75 && position_y < 2.25 && activated_lane == lane {
            println!("+1 point");
       }
   }
}

pub fn deactivate_lane(lane_id: &str, scene: &mut Scene) {
   let lane_node = scene.graph_mut().root_mut().find_mut(lane_id).unwrap();
   if let NodeValue::MeshNode(mesh) = lane_node.value_mut() {
       mesh.material.diffuse = (0.0, 0.0, 0.0);
   }
}
