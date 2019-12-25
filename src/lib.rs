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

use nalgebra_glm as glm;
use tuber::scene::{Scene, SceneNode, NodeValue};
use tuber::graphics::{Material, Mesh};
use tuber::audio::AudioAPI;

const DETECTION_BAR_Y: f32 = 1.0;

pub struct Track {
    lanes: Vec<Lane>,
    notes: Notes,
    node_identifier: String
}

impl Track {
    pub fn new(node_identifier: &str, lane_count: usize) -> Track {
        let mut lanes = vec!();
        for i in 0..lane_count {
            lanes.push(Lane::new(&format!("lane_{}", i)));
        }

        Track {
            lanes,
            notes: Notes::new(),
            node_identifier: String::from(node_identifier)
        }
    }

    pub fn add_note(&mut self, milli_second: usize, lane: usize, size: usize) {
        self.notes.add_note(milli_second, lane, size);
    }

    pub fn activate_lane(&mut self, lane_number: usize, scene: &mut Scene, audio_api: &mut dyn AudioAPI, milli:usize) {
        if let Some(lane) = self.lanes.get_mut(lane_number) {
            println!("input time : {}", milli);
            lane.activate(scene, audio_api, self.notes.get_note_lane_milli(milli, lane_number));
        }
    }

    pub fn deactivate_lane(&mut self, lane_number: usize, scene: &mut Scene) {
        if let Some(lane) = self.lanes.get_mut(lane_number) {
            lane.deactivate(scene);
        }
    }

    pub fn update(&mut self, delta_time: f32, scene: &mut Scene) {
        self.notes.update(delta_time, scene);
    }

    pub fn create_node(&self) -> SceneNode {
        let mut track_node = SceneNode::new(&self.node_identifier, NodeValue::AbstractNode);
        let lane_horizontal_scale = 3.0 / self.lanes.len() as f32;

        for (i, lane) in self.lanes.iter().enumerate() {
            let mut lane_node = lane.create_node(lane_horizontal_scale);
            lane_node.transform_mut().translate(&glm::vec3(i as f32 * lane_horizontal_scale, 0.0, 0.0));
            track_node.add_child(lane_node);
        }

        track_node.add_child(self.notes.create_node(lane_horizontal_scale));


        const DETECTION_BAR_MATERIAL: Material = Material {
            diffuse: (0.8, 0.8, 0.0),
            specular: (0.8, 0.8, 0.0),
            shininess: 32.0,
            texture: None,
        };

        const DETECTION_BAR_SCALE_Y : f32 = 0.015;

        let mut detection_bar_node = SceneNode::new("detection_bar", NodeValue::MeshNode(Mesh::new_plane_mesh(DETECTION_BAR_MATERIAL)));
        detection_bar_node.transform_mut().set_scale(&glm::vec3(lane_horizontal_scale * self.lanes.len() as f32, DETECTION_BAR_SCALE_Y, 1.0));
        detection_bar_node.transform_mut().set_translation(&glm::vec3(0.0, DETECTION_BAR_Y, 0.003));
        track_node.add_child(detection_bar_node);

        track_node
    }
}

const ACTIVE_LANE_MATERIAL: Material = Material {
    diffuse: (0.8, 0.8, 0.8),
    specular: (0.8, 0.8, 0.8),
    shininess: 32.0,
    texture: None,
};

const FOUND_NOTE_LANE_MATERIAL: Material = Material {
    diffuse: (0.5, 1.0, 0.5),
    specular: (0.5, 1.0, 0.5),
    shininess: 32.0,
    texture: None,
};

const LANE_MATERIAL: Material = Material {
    diffuse: (0.0, 0.0, 0.0),
    specular: (0.8, 0.8, 0.8),
    shininess: 32.0,
    texture: None,
};

struct Lane {
    node_identifier: String,
    active: bool
}
impl Lane {
    pub fn new(node_identifier: &str) -> Lane {
        Lane {
            node_identifier: String::from(node_identifier),
            active: false
        }
    }

    pub fn activate(&mut self, scene: &mut Scene, audio_api: &mut dyn AudioAPI, note : bool) {
        let lane_node = scene.graph_mut().root_mut().find_mut(&self.node_identifier).unwrap();
        if let NodeValue::MeshNode(mesh) = lane_node.value_mut() {
            if note {
                mesh.material = FOUND_NOTE_LANE_MATERIAL.clone();
                audio_api.play_sound("tap");
            } else {
                mesh.material = ACTIVE_LANE_MATERIAL.clone();
            }
        }

        self.active = true;
    }

    pub fn deactivate(&mut self, scene: &mut Scene) {
        let lane_node = scene.graph_mut().root_mut().find_mut(&self.node_identifier).unwrap();
        if let NodeValue::MeshNode(mesh) = lane_node.value_mut() {
            mesh.material = LANE_MATERIAL.clone();
        }
        self.active = false;
        println!("Lane '{}' deactivated", self.node_identifier);
    }

    pub fn create_node(&self, horizontal_scale: f32) -> SceneNode {
        let mut lane_node = SceneNode::new(
            &self.node_identifier,
            NodeValue::MeshNode(Mesh::new_plane_mesh(LANE_MATERIAL.clone())),
        );
        lane_node.transform_mut().set_scale(&glm::vec3(horizontal_scale, 100.0, 1.0));

        lane_node
    }
}

struct Notes {
    notes: Vec<TapNote>,
    current_time: f32
}

impl Notes {
    pub fn new() -> Notes {
        Notes {
            notes: vec!(),
            current_time: 0.0
        }
    }

    pub fn get_note_lane_milli (&self, milli_second:usize, lane:usize) -> bool {
        const TOLERANCE_MILLI:usize  = 100;
        return self.notes.iter().any(|note| note.milli_second/TOLERANCE_MILLI == milli_second/TOLERANCE_MILLI && note.lane == lane);
    }

    pub fn add_note(&mut self, milli_second: usize, size: usize, lane: usize) {
        self.notes.push(TapNote::new(milli_second, size, lane));
    }

    pub fn update(&mut self, delta_time: f32, scene: &mut Scene) {
        self.current_time += delta_time;
        scene.graph_mut().root_mut().find_mut("notes").unwrap().transform_mut().set_translation(&glm::vec3(0.0, -self.current_time, 0.005));
    }

    pub fn create_node(&self, lane_scale: f32) -> SceneNode {
        let mut notes_node = SceneNode::new("notes", NodeValue::AbstractNode);
        for (i, note) in self.notes.iter().enumerate() {
            notes_node.add_child(note.create_node(&format!("note_{}", i), lane_scale));
        }

        notes_node
    }
}

struct TapNote {
    milli_second: usize,
    lane: usize,
    size: usize,
}

impl TapNote {
    pub fn new(milli_second: usize, lane: usize, size: usize) -> TapNote {
        TapNote {
            milli_second,
            size,
            lane
        }
    }

    pub fn create_node(&self, node_identifier: &str, lane_scale: f32) -> SceneNode {
        const NOTE_VERTICAL_SCALE: f32 = 0.15;
        let note_material: Material = Material {
            diffuse: (1.0, 0.0, 0.0),
            specular: (1.0, 0.0, 0.0),
            shininess: 256.0,
            texture: Some("note_texture".to_owned()),
        };


        let mut note_node = SceneNode::new(node_identifier,
            NodeValue::MeshNode(Mesh::new_plane_mesh(note_material)));
        note_node.transform_mut().set_scale(&glm::vec3(self.size as f32 * lane_scale, NOTE_VERTICAL_SCALE, 1.0));
        note_node.transform_mut().set_translation(&glm::vec3(self.lane as f32 * lane_scale, (self.milli_second as f32)/1000.0, 0.0));
        note_node
    }
}
