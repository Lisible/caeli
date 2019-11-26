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

pub mod graphics;

pub struct Track {
    section_count: usize,
    sections_states: Vec<bool>
}

impl Track {
    pub fn new(section_count: usize) -> Track {
        Track {
            section_count,
            sections_states: vec![false; section_count]
        }
    }

    pub fn section_count(&self) -> usize {
       self.section_count 
    }

    pub fn activate_section(&mut self, section: usize) {
        if section >= self.section_count { 
            return;
        }

        self.sections_states[section] = true; 
    }

    pub fn deactivate_section(&mut self, section: usize) {
        if section >= self.section_count {
            return;
        }

        self.sections_states[section] = false;
    }

    pub fn is_activated(&self, section: usize) -> bool {
        self.sections_states[section]
    }
}



