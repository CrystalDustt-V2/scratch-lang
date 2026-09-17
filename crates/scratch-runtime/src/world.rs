use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::{Entity, EntityId, RuntimeValue};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PenStroke {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub color: [f32; 4],
    pub size: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PenStamp {
    pub target: String,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub color: [f32; 4],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MusicEvent {
    pub kind: String,
    pub value: f32,
    pub beats: f32,
    pub instrument: i32,
}

#[derive(Debug, Clone)]
pub struct World {
    entities: HashMap<EntityId, Entity>,
    name_index: HashMap<String, EntityId>,
    next_entity_id: u64,
    variables: HashMap<String, RuntimeValue>,
    pub input_actions_down: HashSet<String>,
    pub input_actions_pressed: HashSet<String>,
    pub input_actions_up: HashSet<String>,
    pub mouse_pos: (f32, f32),
    pub mouse_down: bool,
    pub background: String,
    pub backdrops: Vec<String>,
    pub active_backdrop_index: usize,
    pub camera_follow_target: Option<String>,
    pub camera_pos: (f32, f32),
    pub camera_zoom: f32,
    pub pending_messages: Vec<String>,
    pub active_tweens: Vec<crate::movement::GlideTween>,
    pub answer: String,
    pub active_prompt: Option<String>,
    pub sound_effects: HashMap<String, f32>,
    pub pen_active_sprites: HashSet<String>,
    pub pen_color: [f32; 4],
    pub pen_size: f32,
    pub pen_param_hue: f32,
    pub pen_param_saturation: f32,
    pub pen_param_brightness: f32,
    pub pen_param_transparency: f32,
    pub pen_strokes: Vec<PenStroke>,
    pub pen_stamps: Vec<PenStamp>,
    pub music_tempo: f32,
    pub music_instrument: i32,
    pub music_events: Vec<MusicEvent>,
    pub tts_voice: String,
    pub tts_language: String,
    pub tts_speech_queue: Vec<String>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            name_index: HashMap::new(),
            next_entity_id: 1,
            variables: HashMap::new(),
            input_actions_down: HashSet::new(),
            input_actions_pressed: HashSet::new(),
            input_actions_up: HashSet::new(),
            mouse_pos: (0.0, 0.0),
            mouse_down: false,
            background: "white".to_string(),
            backdrops: vec!["backdrop1".to_string()],
            active_backdrop_index: 0,
            camera_follow_target: None,
            camera_pos: (0.0, 0.0),
            camera_zoom: 1.0,
            pending_messages: Vec::new(),
            active_tweens: Vec::new(),
            answer: String::new(),
            active_prompt: None,
            sound_effects: HashMap::new(),
            pen_active_sprites: HashSet::new(),
            pen_color: [0.0, 0.0, 1.0, 1.0], // default pen blue
            pen_size: 1.0,
            pen_param_hue: 66.0,
            pen_param_saturation: 100.0,
            pen_param_brightness: 100.0,
            pen_param_transparency: 0.0,
            pen_strokes: Vec::new(),
            pen_stamps: Vec::new(),
            music_tempo: 60.0,
            music_instrument: 1,
            music_events: Vec::new(),
            tts_voice: "alto".to_string(),
            tts_language: "en".to_string(),
            tts_speech_queue: Vec::new(),
        }
    }

    pub fn ask(&mut self, question: impl Into<String>) {
        let q = question.into();
        self.active_prompt = Some(q.clone());
        self.set_var("__active_prompt", RuntimeValue::String(q));
    }

    pub fn submit_answer(&mut self, ans: impl Into<String>) {
        let a = ans.into();
        self.answer = a.clone();
        self.set_var("__answer", RuntimeValue::String(a));
        self.active_prompt = None;
        self.set_var("__active_prompt", RuntimeValue::Nil);
    }

    pub fn get_answer(&self) -> &str {
        &self.answer
    }

    pub fn show_variable(&mut self, name: &str) {
        self.set_var(format!("__var_visible_{}", name), RuntimeValue::Bool(true));
    }

    pub fn hide_variable(&mut self, name: &str) {
        self.set_var(format!("__var_visible_{}", name), RuntimeValue::Bool(false));
    }

    pub fn is_variable_visible(&self, name: &str) -> bool {
        self.get_var(&format!("__var_visible_{}", name))
            .map(|v| v.as_bool())
            .unwrap_or(false)
    }

    pub fn broadcast(&mut self, message: impl Into<String>) {
        self.pending_messages.push(message.into());
    }

    pub fn take_messages(&mut self) -> Vec<String> {
        std::mem::take(&mut self.pending_messages)
    }

    pub fn spawn_entity(&mut self, name: impl Into<String>) -> EntityId {
        let name_str = name.into();
        let id = EntityId(self.next_entity_id);
        self.next_entity_id += 1;

        let entity = Entity::new(id, name_str.clone());
        self.entities.insert(id, entity);
        self.name_index.insert(name_str, id);
        id
    }

    pub fn get_entity_by_name(&self, name: &str) -> Option<&Entity> {
        self.name_index.get(name).and_then(|id| self.entities.get(id))
    }

    pub fn get_entity_by_name_mut(&mut self, name: &str) -> Option<&mut Entity> {
        if let Some(&id) = self.name_index.get(name) {
            self.entities.get_mut(&id)
        } else {
            None
        }
    }

    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn iter_entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    pub fn set_var(&mut self, name: impl Into<String>, value: RuntimeValue) {
        self.variables.insert(name.into(), value);
    }

    pub fn get_var(&self, name: &str) -> Option<&RuntimeValue> {
        self.variables.get(name)
    }

    pub fn get_var_or_nil(&self, name: &str) -> RuntimeValue {
        self.variables.get(name).cloned().unwrap_or(RuntimeValue::Nil)
    }

    pub fn get_list(&self, name: &str) -> Option<&[RuntimeValue]> {
        self.variables.get(name).and_then(|v| v.as_list())
    }

    pub fn get_list_mut(&mut self, name: &str) -> &mut Vec<RuntimeValue> {
        let entry = self.variables.entry(name.to_string()).or_insert_with(|| RuntimeValue::List(Vec::new()));
        if !matches!(entry, RuntimeValue::List(_)) {
            *entry = RuntimeValue::List(Vec::new());
        }
        match entry {
            RuntimeValue::List(l) => l,
            _ => unreachable!(),
        }
    }

    pub fn is_action_down(&self, action: &str) -> bool {
        self.input_actions_down.contains(action)
    }

    pub fn is_action_pressed(&self, action: &str) -> bool {
        self.input_actions_pressed.contains(action)
    }

    pub fn is_action_up(&self, action: &str) -> bool {
        self.input_actions_up.contains(action)
    }

    pub fn set_action_down(&mut self, action: &str, down: bool) {
        if down {
            if !self.input_actions_down.contains(action) {
                self.input_actions_pressed.insert(action.to_string());
            }
            self.input_actions_down.insert(action.to_string());
        } else {
            if self.input_actions_down.contains(action) {
                self.input_actions_up.insert(action.to_string());
            }
            self.input_actions_down.remove(action);
        }
    }

    pub fn clear_transient_inputs(&mut self) {
        self.input_actions_pressed.clear();
        self.input_actions_up.clear();
    }

    // Backdrop management
    pub fn switch_backdrop(&mut self, name: impl Into<String>) {
        let name_str = name.into();
        self.background = name_str.clone();
        if let Some(pos) = self.backdrops.iter().position(|b| b == &name_str) {
            self.active_backdrop_index = pos;
        } else {
            self.backdrops.push(name_str);
            self.active_backdrop_index = self.backdrops.len() - 1;
        }
    }

    pub fn next_backdrop(&mut self) {
        if !self.backdrops.is_empty() {
            self.active_backdrop_index = (self.active_backdrop_index + 1) % self.backdrops.len();
            self.background = self.backdrops[self.active_backdrop_index].clone();
        }
    }

    pub fn get_backdrop_number(&self) -> usize {
        self.active_backdrop_index + 1
    }

    pub fn get_backdrop_name(&self) -> &str {
        if let Some(b) = self.backdrops.get(self.active_backdrop_index) {
            b.as_str()
        } else {
            &self.background
        }
    }

    // Pen Subsystem
    pub fn pen_down(&mut self, target: &str) {
        self.pen_active_sprites.insert(target.to_string());
    }

    pub fn pen_up(&mut self, target: &str) {
        self.pen_active_sprites.remove(target);
    }

    pub fn pen_clear(&mut self) {
        self.pen_strokes.clear();
        self.pen_stamps.clear();
    }

    pub fn pen_stamp(&mut self, target: &str) {
        if let Some(ent) = self.get_entity_by_name(target) {
            self.pen_stamps.push(PenStamp {
                target: target.to_string(),
                x: ent.transform.x,
                y: ent.transform.y,
                rotation: ent.transform.rotation,
                scale_x: ent.transform.scale_x,
                scale_y: ent.transform.scale_y,
                color: ent.color,
            });
        }
    }

    pub fn pen_set_color(&mut self, color: [f32; 4]) {
        self.pen_color = color;
    }

    pub fn pen_set_size(&mut self, size: f32) {
        self.pen_size = size.max(1.0);
    }

    pub fn pen_change_size(&mut self, delta: f32) {
        self.pen_size = (self.pen_size + delta).max(1.0);
    }

    pub fn pen_set_param(&mut self, param: &str, val: f32) {
        match param.to_lowercase().as_str() {
            "color" | "hue" => self.pen_param_hue = val % 100.0,
            "saturation" => self.pen_param_saturation = val.clamp(0.0, 100.0),
            "brightness" => self.pen_param_brightness = val.clamp(0.0, 100.0),
            "transparency" => self.pen_param_transparency = val.clamp(0.0, 100.0),
            _ => {}
        }
    }

    pub fn pen_change_param(&mut self, param: &str, delta: f32) {
        match param.to_lowercase().as_str() {
            "color" | "hue" => self.pen_param_hue = (self.pen_param_hue + delta) % 100.0,
            "saturation" => self.pen_param_saturation = (self.pen_param_saturation + delta).clamp(0.0, 100.0),
            "brightness" => self.pen_param_brightness = (self.pen_param_brightness + delta).clamp(0.0, 100.0),
            "transparency" => self.pen_param_transparency = (self.pen_param_transparency + delta).clamp(0.0, 100.0),
            _ => {}
        }
    }

    pub fn add_pen_stroke(&mut self, stroke: PenStroke) {
        self.pen_strokes.push(stroke);
    }

    // Music Subsystem
    pub fn music_play_note(&mut self, note: f32, beats: f32) {
        self.music_events.push(MusicEvent {
            kind: "note".to_string(),
            value: note,
            beats,
            instrument: self.music_instrument,
        });
    }

    pub fn music_play_drum(&mut self, drum: f32, beats: f32) {
        self.music_events.push(MusicEvent {
            kind: "drum".to_string(),
            value: drum,
            beats,
            instrument: self.music_instrument,
        });
    }

    pub fn music_set_tempo(&mut self, bpm: f32) {
        self.music_tempo = bpm.max(20.0);
        self.set_var("__music_tempo", RuntimeValue::Number(self.music_tempo as f64));
    }

    pub fn music_change_tempo(&mut self, delta: f32) {
        self.music_tempo = (self.music_tempo + delta).max(20.0);
        self.set_var("__music_tempo", RuntimeValue::Number(self.music_tempo as f64));
    }

    pub fn music_set_instrument(&mut self, inst: i32) {
        self.music_instrument = inst;
    }

    // Sound DSP
    pub fn sound_set_effect(&mut self, effect: &str, val: f32) {
        self.sound_effects.insert(effect.to_lowercase(), val);
    }

    pub fn sound_change_effect(&mut self, effect: &str, delta: f32) {
        let cur = self.sound_effects.entry(effect.to_lowercase()).or_insert(0.0);
        *cur += delta;
    }

    pub fn sound_clear_effects(&mut self) {
        self.sound_effects.clear();
    }

    // Text to Speech
    pub fn tts_speak(&mut self, text: impl Into<String>) {
        self.tts_speech_queue.push(text.into());
    }

    pub fn tts_set_voice(&mut self, voice: impl Into<String>) {
        self.tts_voice = voice.into();
    }

    pub fn tts_set_language(&mut self, lang: impl Into<String>) {
        self.tts_language = lang.into();
    }
}
