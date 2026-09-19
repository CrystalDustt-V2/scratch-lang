use eframe::egui;
use scratch_blocks::BlockRegistry;
use scratch_ir::lower_ast_to_ir;
use scratch_language::parse;
use scratch_project::{AspectRatioPreset, ProjectConfig};
use scratch_runtime::{Runtime, RuntimeValue};
use scratch_scenes::SceneData;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::project_cmd;
use crate::stage_view::StageRenderer;

pub struct PreviewApp {
    pub runtime: Runtime,
    pub config: ProjectConfig,
    pub selected_ratio: AspectRatioPreset,
    pub stage_renderer: StageRenderer,
    pub paused: bool,
    pub last_tick: Instant,
    pub fps_timer: Instant,
    pub frame_count: u32,
    pub current_fps: f32,
    pub source_path: PathBuf,
    pub original_code: String,
    pub scene_name: String,
}

impl PreviewApp {
    pub fn new(source_path: PathBuf, code: String, config: ProjectConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let registry = BlockRegistry::core();
        let ast = parse(&code).map_err(|e| format!("Syntax error: {}", e))?;
        let ir = lower_ast_to_ir(&ast, &registry).map_err(|e| format!("IR error: {}", e))?;
        let mut runtime = Runtime::new(ir, registry);

        // Load scene if available in project
        let proj_dir = if source_path.is_file() {
            source_path.parent().and_then(|p| if p.ends_with("src") { p.parent() } else { Some(p) }).unwrap_or(Path::new("."))
        } else {
            Path::new(".")
        };

        let mut scene_name = "Main".to_string();
        let main_scene_path = if proj_dir.join("scenes/main.schscene").exists() {
            proj_dir.join("scenes/main.schscene")
        } else {
            proj_dir.join("scenes/main.scene")
        };
        if main_scene_path.exists() {
            if let Ok(scene) = SceneData::load_from_file(&main_scene_path) {
                scene_name = scene.name.clone();
                runtime.world.background = scene.background.clone();
                if scene.objects.iter().any(|obj| obj.x > 640.0 || obj.y > 360.0) {
                    runtime.world.is_corner_origin = true;
                }
                for obj in scene.objects {
                    if let Some(ent) = runtime.world.get_entity_by_name_mut(&obj.name) {
                        ent.transform.x = obj.x;
                        ent.transform.y = obj.y;
                        ent.size = obj.size;
                        ent.color = obj.color;
                    } else {
                        let id = runtime.world.spawn_entity(&obj.name);
                        if let Some(ent) = runtime.world.get_entity_mut(id) {
                            ent.transform.x = obj.x;
                            ent.transform.y = obj.y;
                            ent.size = obj.size;
                            ent.color = obj.color;
                        }
                    }
                }
            }
        }

        runtime.start();

        let selected_ratio = AspectRatioPreset::parse_str(&config.ratio)
            .unwrap_or(AspectRatioPreset::Ratio16x9);

        Ok(Self {
            runtime,
            config,
            selected_ratio,
            stage_renderer: StageRenderer::new(),
            paused: false,
            last_tick: Instant::now(),
            fps_timer: Instant::now(),
            frame_count: 0,
            current_fps: 60.0,
            source_path,
            original_code: code,
            scene_name,
        })
    }

    pub fn restart(&mut self) {
        if let Ok(mut new_app) = Self::new(self.source_path.clone(), self.original_code.clone(), self.config.clone()) {
            new_app.selected_ratio = self.selected_ratio;
            self.runtime = new_app.runtime;
            self.paused = false;
            self.last_tick = Instant::now();
        }
    }
}

impl eframe::App for PreviewApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Continuous 60 FPS repainting
        ui.ctx().request_repaint();

        // Advance simulation tick if not paused
        if !self.paused {
            let dt = self.last_tick.elapsed().as_secs_f32().min(0.05);
            self.last_tick = Instant::now();
            self.runtime.tick(dt);

            let cur_t = self.runtime.world.get_var_or_nil("__runtime_timer").as_number().unwrap_or(0.0);
            self.runtime.world.set_var("__runtime_timer", RuntimeValue::Number(cur_t + dt as f64));
        } else {
            self.last_tick = Instant::now();
        }

        // FPS calculation
        self.frame_count += 1;
        let elapsed = self.fps_timer.elapsed().as_secs_f32();
        if elapsed >= 0.5 {
            self.current_fps = (self.frame_count as f32 / elapsed).round();
            self.frame_count = 0;
            self.fps_timer = Instant::now();
        }

        // 1. Top Control Bar
        egui::Panel::top("preview_top_bar").show(ui, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                // Restart button (iconic Scratch Green Flag)
                let restart_btn = egui::Button::new(egui::RichText::new("🚩 Restart").color(egui::Color32::from_rgb(34, 197, 94)).strong())
                    .fill(egui::Color32::from_rgb(20, 50, 30));
                if ui.add(restart_btn).on_hover_text("Restart game simulation from the beginning").clicked() {
                    self.restart();
                }

                // Pause / Resume button
                let (pause_label, pause_color) = if self.paused {
                    ("▶ Resume", egui::Color32::from_rgb(56, 189, 248))
                } else {
                    ("⏸ Pause", egui::Color32::from_rgb(251, 191, 36))
                };
                let pause_btn = egui::Button::new(egui::RichText::new(pause_label).color(pause_color).strong())
                    .fill(egui::Color32::from_rgb(30, 41, 59));
                if ui.add(pause_btn).clicked() {
                    self.paused = !self.paused;
                }

                ui.separator();

                // Timer display
                let timer_val = self.runtime.world.get_var_or_nil("__runtime_timer").as_number().unwrap_or(0.0);
                ui.label(egui::RichText::new(format!("⏱ {:.1}s", timer_val)).monospace().color(egui::Color32::from_rgb(226, 232, 240)));

                // FPS monitor
                let fps_color = if self.current_fps >= 50.0 {
                    egui::Color32::from_rgb(74, 222, 128)
                } else if self.current_fps >= 30.0 {
                    egui::Color32::from_rgb(250, 204, 21)
                } else {
                    egui::Color32::from_rgb(248, 113, 113)
                };
                ui.label(egui::RichText::new(format!("⚡ {} FPS", self.current_fps as u32)).monospace().color(fps_color));

                ui.separator();

                // Scene badge
                ui.label(egui::RichText::new(format!("🎬 Scene: {}", self.scene_name)).color(egui::Color32::from_rgb(192, 132, 252)));

                ui.separator();

                // Live Aspect Ratio Selector (16:9, 4:3, Device)
                let mut changed_ratio = None;
                egui::ComboBox::from_id_salt("preview_ratio_picker")
                    .selected_text(format!("📐 {}", self.selected_ratio.short_label()))
                    .show_ui(ui, |ui| {
                        for preset in [
                            AspectRatioPreset::Ratio16x9,
                            AspectRatioPreset::Ratio4x3,
                            AspectRatioPreset::Device,
                        ] {
                            let is_selected = self.selected_ratio == preset;
                            if ui.selectable_label(is_selected, preset.label()).clicked() {
                                changed_ratio = Some(preset);
                            }
                        }
                    });

                if let Some(preset) = changed_ratio {
                    self.selected_ratio = preset;
                    self.config.apply_ratio_preset(preset);
                    let (w, h) = preset.to_resolution();
                    self.runtime.world.stage_width = w as f32;
                    self.runtime.world.stage_height = h as f32;
                    let entity_names: Vec<String> = self.runtime.world.iter_entities().map(|e| e.name.clone()).collect();
                    for name in entity_names {
                        self.runtime.world.clamp_entity_to_stage(&name);
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("⌨ [W/A/S/D/Arrows] Move  [Space] Jump  [Mouse] Aim & Click")
                            .size(11.0)
                            .color(egui::Color32::from_rgb(148, 163, 184)),
                    );
                });
            });
            ui.add_space(4.0);
        });

        // 2. Bottom Status Bar
        egui::Panel::bottom("preview_bottom_bar").show(ui, |ui| {
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("🎮 {}", self.config.name)).strong().color(egui::Color32::WHITE));
                ui.separator();
                ui.label(egui::RichText::new(format!("Ratio: {}", self.config.ratio)).size(11.0).color(egui::Color32::from_rgb(56, 189, 248)));
                ui.separator();
                ui.label(egui::RichText::new(format!("Entities: {}", self.runtime.world.iter_entities().count())).size(11.0).color(egui::Color32::from_rgb(203, 213, 225)));
                ui.separator();
                ui.label(egui::RichText::new(format!("Source: {}", self.source_path.display())).size(11.0).color(egui::Color32::from_rgb(148, 163, 184)));

                // If ask prompt is active, render inline text field
                if let Some(prompt_q) = self.runtime.world.active_prompt.clone() {
                    ui.separator();
                    ui.label(egui::RichText::new(format!("💬 {}", prompt_q)).color(egui::Color32::from_rgb(56, 189, 248)).strong());
                    let text_edit = egui::TextEdit::singleline(&mut self.stage_renderer.ask_buffer)
                        .hint_text("Type answer and press Enter...")
                        .desired_width(180.0);
                    let resp = ui.add(text_edit);
                    if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        let ans = self.stage_renderer.ask_buffer.trim().to_string();
                        self.runtime.world.submit_answer(ans);
                        self.runtime.world.broadcast("answered");
                        self.stage_renderer.ask_buffer.clear();
                    }
                    if ui.button("OK").clicked() {
                        let ans = self.stage_renderer.ask_buffer.trim().to_string();
                        self.runtime.world.submit_answer(ans);
                        self.runtime.world.broadcast("answered");
                        self.stage_renderer.ask_buffer.clear();
                    }
                }
            });
            ui.add_space(2.0);
        });

        // 3. Central Stage Viewport
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(15, 23, 42)))
            .show(ui, |ui| {
                let avail = ui.available_size();
                self.stage_renderer.render(ui, &mut self.runtime, &self.config, avail, true);
            });
    }
}

pub fn run_preview_native(path: &Path, ratio_override: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, mut config) = project_cmd::resolve_entry_file(path)?;
    if let Some(r) = ratio_override {
        if let Some(preset) = AspectRatioPreset::parse_str(r) {
            config.apply_ratio_preset(preset);
        } else {
            eprintln!("Warning: Unrecognized ratio override '{}'. Using project default.", r);
        }
    }

    println!("Loading scratch-lang native popup app preview for '{}'...", config.name);
    println!("Source: {}", entry_path.display());
    println!("Ratio:  {} ({}x{})", config.ratio, config.resolution.width, config.resolution.height);

    let source = std::fs::read_to_string(&entry_path)?;
    let app = PreviewApp::new(entry_path.clone(), source, config.clone())?;

    let window_title = format!("Scratch Live Preview - {}", config.name);
    let win_w = (config.resolution.width as f32).min(1920.0).max(800.0);
    let win_h = (config.resolution.height as f32 + 64.0).min(1080.0).max(500.0);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([win_w, win_h])
            .with_min_inner_size([640.0, 400.0])
            .with_title(window_title),
        ..Default::default()
    };

    println!("============================================================");
    println!(" Scratch Live Native Popup Preview Running!");
    println!(" Project:  {}", config.name);
    println!(" Window:   Native Rust Desktop Window ({}x{}, {})", win_w as u32, win_h as u32, config.ratio);
    println!(" Controls: Arrow Keys / WASD (Move 4-Way), Space (Jump), Mouse Click");
    println!(" Close the window or press Alt+F4 to exit.");
    println!("============================================================");

    eframe::run_native(
        "scratch_preview",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )?;

    Ok(())
}
