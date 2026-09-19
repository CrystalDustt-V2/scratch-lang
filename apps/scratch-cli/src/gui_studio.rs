use eframe::egui;
use scratch_blocks::{BlockCategory, BlockRegistry};
use scratch_ir::lower_ast_to_ir;
use scratch_language::{format_source, lint_source, parse, Diagnostic};
use scratch_project::ProjectConfig;
use scratch_runtime::{Runtime, RuntimeValue};
use scratch_scenes::SceneData;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::project_cmd;
use crate::stage_view::StageRenderer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioTab {
    Variables,
    Entities,
    Diagnostics,
}

pub struct StudioApp {
    pub code: String,
    pub source_path: PathBuf,
    pub proj_dir: PathBuf,
    pub config: ProjectConfig,
    pub runtime: Runtime,
    pub stage_renderer: StageRenderer,
    pub selected_category: BlockCategory,
    pub filter_query: String,
    pub paused: bool,
    pub last_tick: Instant,
    pub fps_timer: Instant,
    pub frame_count: u32,
    pub current_fps: f32,
    pub active_tab: StudioTab,
    pub status_message: String,
    pub status_time: Instant,
    pub diagnostics: Vec<Diagnostic>,
    pub available_scenes: Vec<String>,
    pub selected_scene: String,
    pub registry: BlockRegistry,
    pub is_dirty: bool,
}

impl StudioApp {
    pub fn new(source_path: PathBuf, config: ProjectConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let code = if source_path.exists() {
            std::fs::read_to_string(&source_path)?
        } else {
            "when start:\n    score = 0\n".to_string()
        };

        let proj_dir = if source_path.is_file() {
            source_path
                .parent()
                .and_then(|p| if p.ends_with("src") { p.parent() } else { Some(p) })
                .unwrap_or(Path::new("."))
                .to_path_buf()
        } else {
            PathBuf::from(".")
        };

        // Discover scenes in scenes/ folder
        let mut available_scenes = Vec::new();
        let scenes_folder = proj_dir.join("scenes");
        if scenes_folder.exists() {
            if let Ok(entries) = std::fs::read_dir(&scenes_folder) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                        if ext == "schscene" || ext == "scene" {
                            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                                if !available_scenes.contains(&stem.to_string()) {
                                    available_scenes.push(stem.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        if available_scenes.is_empty() {
            available_scenes.push("main".to_string());
        }
        let selected_scene = available_scenes[0].clone();

        let registry = BlockRegistry::core();
        let diagnostics = lint_source(&code, &registry).unwrap_or_default();

        let mut runtime = create_runtime(&code, &proj_dir, &selected_scene, &registry);
        runtime.start();

        Ok(Self {
            code,
            source_path,
            proj_dir,
            config,
            runtime,
            stage_renderer: StageRenderer::new(),
            selected_category: BlockCategory::Movement,
            filter_query: String::new(),
            paused: false,
            last_tick: Instant::now(),
            fps_timer: Instant::now(),
            frame_count: 0,
            current_fps: 60.0,
            active_tab: StudioTab::Variables,
            status_message: "Ready. Live preview running.".to_string(),
            status_time: Instant::now(),
            diagnostics,
            available_scenes,
            selected_scene,
            registry,
            is_dirty: false,
        })
    }

    pub fn reload_runtime(&mut self) {
        let mut new_rt = create_runtime(&self.code, &self.proj_dir, &self.selected_scene, &self.registry);
        new_rt.start();
        self.runtime = new_rt;
        self.diagnostics = lint_source(&self.code, &self.registry).unwrap_or_default();
        self.set_status("Reloaded game runtime.");
    }

    pub fn save_file(&mut self) {
        if let Err(e) = std::fs::write(&self.source_path, &self.code) {
            self.set_status(&format!("Error saving: {}", e));
        } else {
            self.is_dirty = false;
            self.set_status(&format!("Saved to {}", self.source_path.display()));
            self.reload_runtime();
        }
    }

    pub fn format_code(&mut self) {
        match format_source(&self.code) {
            Ok(formatted) => {
                self.code = formatted;
                self.is_dirty = true;
                self.set_status("Code formatted successfully.");
                self.reload_runtime();
            }
            Err(e) => {
                self.set_status(&format!("Format error: {}", e));
            }
        }
    }

    pub fn set_status(&mut self, msg: &str) {
        self.status_message = msg.to_string();
        self.status_time = Instant::now();
    }
}

fn create_runtime(code: &str, proj_dir: &Path, scene_name: &str, registry: &BlockRegistry) -> Runtime {
    let ast = parse(code).unwrap_or_else(|_| scratch_language::Program {
        events: Vec::new(),
        functions: Vec::new(),
        top_level: Vec::new(),
    });
    let ir = lower_ast_to_ir(&ast, registry).unwrap_or_else(|_| scratch_ir::IrProgram {
        events: Vec::new(),
        functions: Vec::new(),
    });
    let mut runtime = Runtime::new(ir, registry.clone());

    // Check scene file (.schscene or .scene)
    let candidate = proj_dir.join("scenes").join(format!("{}.schscene", scene_name));
    let scene_path = if candidate.exists() {
        candidate
    } else {
        proj_dir.join("scenes").join(format!("{}.scene", scene_name))
    };
    if scene_path.exists() {
        if let Ok(scene) = SceneData::load_from_file(&scene_path) {
            runtime.world.background = scene.background.clone();
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

    runtime
}

impl eframe::App for StudioApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Continuous repaint for 60 FPS live stage
        ui.ctx().request_repaint();

        // Keyboard shortcuts
        ui.input(|i| {
            if i.modifiers.command && i.key_pressed(egui::Key::S) {
                self.save_file();
            }
        });

        // Advance simulation tick
        if !self.paused {
            let dt = self.last_tick.elapsed().as_secs_f32().min(0.05);
            self.last_tick = Instant::now();
            self.runtime.tick(dt);

            let cur_t = self.runtime.world.get_var_or_nil("__runtime_timer").as_number().unwrap_or(0.0);
            self.runtime.world.set_var("__runtime_timer", RuntimeValue::Number(cur_t + dt as f64));
        } else {
            self.last_tick = Instant::now();
        }

        // FPS tracking
        self.frame_count += 1;
        let elapsed = self.fps_timer.elapsed().as_secs_f32();
        if elapsed >= 0.5 {
            self.current_fps = (self.frame_count as f32 / elapsed).round();
            self.frame_count = 0;
            self.fps_timer = Instant::now();
        }

        // 1. TOP TOOLBAR
        egui::Panel::top("studio_top_bar").show(ui, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(" Scratch Studio").heading().strong().color(egui::Color32::from_rgb(255, 171, 25)));
                ui.separator();

                // Green flag (Run/Restart)
                let run_btn = egui::Button::new(egui::RichText::new("🚩 Run").color(egui::Color32::from_rgb(34, 197, 94)).strong())
                    .fill(egui::Color32::from_rgb(20, 50, 30));
                if ui.add(run_btn).on_hover_text("Run / Restart live game simulation").clicked() {
                    self.reload_runtime();
                }

                // Stop sign
                let stop_btn = egui::Button::new(egui::RichText::new("🛑 Stop").color(egui::Color32::from_rgb(239, 68, 68)).strong())
                    .fill(egui::Color32::from_rgb(50, 20, 20));
                if ui.add(stop_btn).on_hover_text("Pause simulation").clicked() {
                    self.paused = true;
                }

                // Resume/Pause toggle
                if self.paused {
                    if ui.button(egui::RichText::new("▶ Resume").color(egui::Color32::from_rgb(56, 189, 248))).clicked() {
                        self.paused = false;
                    }
                }

                ui.separator();

                // Save button
                let save_text = if self.is_dirty { "💾 Save *" } else { "💾 Save" };
                let save_btn = egui::Button::new(egui::RichText::new(save_text).color(if self.is_dirty { egui::Color32::from_rgb(251, 191, 36) } else { egui::Color32::WHITE }));
                if ui.add(save_btn).on_hover_text("Save code to disk (Ctrl+S)").clicked() {
                    self.save_file();
                }

                // Format button
                if ui.button("🧹 Format").on_hover_text("Auto-format source code").clicked() {
                    self.format_code();
                }

                // Check button
                if ui.button("🔍 Check").on_hover_text("Check syntax and linter rules").clicked() {
                    self.diagnostics = lint_source(&self.code, &self.registry).unwrap_or_default();
                    if self.diagnostics.is_empty() {
                        self.set_status("✓ All checks passed! No issues found.");
                    } else {
                        self.set_status(&format!("Found {} diagnostic issue(s).", self.diagnostics.len()));
                    }
                }

                ui.separator();

                // Scene switcher
                ui.label("Scene:");
                let prev_scene = self.selected_scene.clone();
                egui::ComboBox::from_id_salt("scene_selector")
                    .selected_text(&self.selected_scene)
                    .show_ui(ui, |ui| {
                        for sc in &self.available_scenes {
                            ui.selectable_value(&mut self.selected_scene, sc.clone(), sc);
                        }
                    });
                if self.selected_scene != prev_scene {
                    self.reload_runtime();
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(format!("⚡ {} FPS", self.current_fps as u32)).monospace().color(egui::Color32::from_rgb(74, 222, 128)));
                    ui.separator();
                    if self.diagnostics.is_empty() {
                        ui.label(egui::RichText::new("● 0 Errors").color(egui::Color32::from_rgb(74, 222, 128)));
                    } else {
                        ui.label(egui::RichText::new(format!("⚠ {} Issues", self.diagnostics.len())).color(egui::Color32::from_rgb(251, 191, 36)));
                    }
                });
            });
            ui.add_space(4.0);
        });

        // 2. BOTTOM PANEL (Inspections, Variables, Diagnostics)
        egui::Panel::bottom("studio_bottom_panel")
            .resizable(true)
            .default_size(160.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.active_tab, StudioTab::Variables, "📊 Variables & Lists");
                    ui.selectable_value(&mut self.active_tab, StudioTab::Entities, "👾 Entities & Scene");
                    ui.selectable_value(&mut self.active_tab, StudioTab::Diagnostics, format!("📋 Diagnostics ({})", self.diagnostics.len()));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.status_time.elapsed().as_secs_f32() < 4.0 {
                            ui.label(egui::RichText::new(&self.status_message).color(egui::Color32::from_rgb(56, 189, 248)).italics());
                        }
                    });
                });
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    match self.active_tab {
                        StudioTab::Variables => {
                            ui.columns(2, |cols| {
                                cols[0].label(egui::RichText::new("Variables").strong().color(egui::Color32::from_rgb(255, 140, 0)));
                                egui::Grid::new("vars_grid").striped(true).show(&mut cols[0], |ui| {
                                    ui.label("Variable");
                                    ui.label("Value");
                                    ui.end_row();
                                    for var_name in ["score", "health", "time", "coins", "level", "points"] {
                                        if let Some(val) = self.runtime.world.get_var(var_name) {
                                            ui.label(egui::RichText::new(var_name).monospace());
                                            ui.label(egui::RichText::new(val.to_string()).strong().monospace());
                                            ui.end_row();
                                        }
                                    }
                                });

                                cols[1].label(egui::RichText::new("Lists").strong().color(egui::Color32::from_rgb(255, 102, 128)));
                                for list_name in ["inventory", "items", "highscores"] {
                                    if let Some(items) = self.runtime.world.get_list(list_name) {
                                        cols[1].label(egui::RichText::new(format!("{}: [{}]", list_name, items.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "))).monospace());
                                    }
                                }
                            });
                        }
                        StudioTab::Entities => {
                            egui::Grid::new("entities_grid").striped(true).show(ui, |ui| {
                                ui.label("Entity");
                                ui.label("Position");
                                ui.label("Size");
                                ui.label("Visible");
                                ui.end_row();
                                for ent in self.runtime.world.iter_entities() {
                                    ui.label(egui::RichText::new(&ent.name).strong());
                                    ui.label(format!("({:.1}, {:.1})", ent.transform.x, ent.transform.y));
                                    ui.label(format!("{:.0}x{:.0}", ent.size[0], ent.size[1]));
                                    ui.label(if ent.visible { "✓ Yes" } else { "✗ No" });
                                    ui.end_row();
                                }
                            });
                        }
                        StudioTab::Diagnostics => {
                            if self.diagnostics.is_empty() {
                                ui.label(egui::RichText::new("✓ No compiler or linter errors found. Code is healthy!").color(egui::Color32::from_rgb(74, 222, 128)));
                            } else {
                                for diag in &self.diagnostics {
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(format!("[{}] Line {}:{}", diag.code, diag.span.line, diag.span.col)).color(egui::Color32::from_rgb(251, 191, 36)).monospace());
                                        ui.label(&diag.message);
                                        if let Some(hint) = &diag.suggestion {
                                            ui.label(egui::RichText::new(format!("💡 {}", hint)).color(egui::Color32::from_rgb(56, 189, 248)));
                                        }
                                    });
                                }
                            }
                        }
                    }
                });
            });

        // 3. LEFT DOCK: SCRATCH BLOCK PALETTE (width: 250px)
        egui::Panel::left("studio_blocks_palette")
            .resizable(true)
            .default_size(260.0)
            .show(ui, |ui| {
                ui.add_space(4.0);
                ui.label(egui::RichText::new("Block Palette").strong());
                ui.add_space(4.0);

                // Category Buttons Grid
                egui::Grid::new("cat_grid").spacing(egui::Vec2::new(4.0, 4.0)).show(ui, |ui| {
                    let cats = [
                        (BlockCategory::Movement, "Motion", egui::Color32::from_rgb(76, 151, 255)),
                        (BlockCategory::Looks, "Looks", egui::Color32::from_rgb(153, 102, 255)),
                        (BlockCategory::Audio, "Sound", egui::Color32::from_rgb(214, 92, 214)),
                        (BlockCategory::Events, "Events", egui::Color32::from_rgb(255, 191, 0)),
                        (BlockCategory::Control, "Control", egui::Color32::from_rgb(255, 171, 25)),
                        (BlockCategory::Sensing, "Sensing", egui::Color32::from_rgb(76, 191, 230)),
                        (BlockCategory::Operators, "Operators", egui::Color32::from_rgb(89, 192, 89)),
                        (BlockCategory::Variables, "Variables", egui::Color32::from_rgb(255, 140, 26)),
                        (BlockCategory::Custom, "Lists", egui::Color32::from_rgb(255, 102, 128)),
                    ];

                    for (idx, (cat, name, col)) in cats.iter().enumerate() {
                        let is_selected = self.selected_category == *cat;
                        let text = egui::RichText::new(*name).color(if is_selected { egui::Color32::WHITE } else { egui::Color32::from_rgb(220, 225, 235) }).size(11.0);
                        let btn = egui::Button::new(text).fill(if is_selected { *col } else { egui::Color32::from_rgb(30, 41, 59) });
                        if ui.add(btn).clicked() {
                            self.selected_category = cat.clone();
                        }
                        if (idx + 1) % 3 == 0 {
                            ui.end_row();
                        }
                    }
                });

                ui.separator();
                ui.add(egui::TextEdit::singleline(&mut self.filter_query).hint_text("🔍 Filter blocks..."));
                ui.add_space(4.0);

                // Blocks list for selected category
                let blocks = get_category_snippets(self.selected_category.clone());
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (snippet, desc) in blocks {
                        if !self.filter_query.is_empty() && !snippet.to_lowercase().contains(&self.filter_query.to_lowercase()) {
                            continue;
                        }

                        let chip_col = get_category_color(self.selected_category.clone());
                        let chip_btn = egui::Button::new(egui::RichText::new(&snippet).monospace().size(11.5).color(egui::Color32::WHITE))
                            .fill(chip_col)
                            .corner_radius(6.0);

                        if ui.add(chip_btn).on_hover_text(format!("Click to insert:\n{}", desc)).clicked() {
                            // Insert snippet into code
                            if !self.code.ends_with('\n') && !self.code.is_empty() {
                                self.code.push('\n');
                            }
                            self.code.push_str(&snippet);
                            self.code.push('\n');
                            self.is_dirty = true;
                            self.set_status(&format!("Inserted '{}' into code.", snippet));
                            self.reload_runtime();
                        }
                        ui.add_space(3.0);
                    }
                });
            });

        // 4. RIGHT DOCK: LIVE GAME STAGE PREVIEW (width: 450px)
        egui::Panel::right("studio_stage_preview")
            .resizable(true)
            .default_size(450.0)
            .show(ui, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Live Stage").strong());
                    ui.separator();
                    if ui.button(if self.paused { "▶ Resume" } else { "⏸ Pause" }).clicked() {
                        self.paused = !self.paused;
                    }
                    if ui.button("🔄 Reset").clicked() {
                        self.reload_runtime();
                    }
                });
                ui.separator();

                // Interactive stage
                let avail = ui.available_size();
                self.stage_renderer.render(ui, &mut self.runtime, &self.config, avail, true);
            });

        // 5. CENTER DOCK: CODE EDITOR
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                let title = format!("📄 {} {}", self.source_path.file_name().and_then(|s| s.to_str()).unwrap_or("main.sch"), if self.is_dirty { "● (unsaved)" } else { "" });
                ui.label(egui::RichText::new(title).strong().color(if self.is_dirty { egui::Color32::from_rgb(251, 191, 36) } else { egui::Color32::WHITE }));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(format!("Lines: {}", self.code.lines().count())).size(11.0).color(egui::Color32::from_rgb(148, 163, 184)));
                });
            });
            ui.separator();

            // Code Editor
            egui::ScrollArea::both().show(ui, |ui| {
                let prev_code = self.code.clone();
                let editor = egui::TextEdit::multiline(&mut self.code)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .lock_focus(true);
                ui.add(editor);

                if self.code != prev_code {
                    self.is_dirty = true;
                    self.diagnostics = lint_source(&self.code, &self.registry).unwrap_or_default();
                }
            });
        });
    }
}

fn get_category_color(cat: BlockCategory) -> egui::Color32 {
    match cat {
        BlockCategory::Movement => egui::Color32::from_rgb(76, 151, 255),
        BlockCategory::Looks => egui::Color32::from_rgb(153, 102, 255),
        BlockCategory::Audio => egui::Color32::from_rgb(214, 92, 214),
        BlockCategory::Events => egui::Color32::from_rgb(255, 191, 0),
        BlockCategory::Control => egui::Color32::from_rgb(255, 171, 25),
        BlockCategory::Sensing => egui::Color32::from_rgb(76, 191, 230),
        BlockCategory::Operators => egui::Color32::from_rgb(89, 192, 89),
        BlockCategory::Variables => egui::Color32::from_rgb(255, 140, 26),
        _ => egui::Color32::from_rgb(255, 102, 128),
    }
}

fn get_category_snippets(cat: BlockCategory) -> Vec<(String, &'static str)> {
    match cat {
        BlockCategory::Movement => vec![
            ("move(Player, 10)".into(), "Move player horizontally by delta pixels"),
            ("change_y(Player, 10)".into(), "Move player vertically by delta pixels"),
            ("teleport(Player, 100, 200)".into(), "Set absolute position (x, y)"),
            ("glide(Player, 2, 300, 200)".into(), "Smoothly glide to position over duration"),
            ("turn_right(Player, 15)".into(), "Rotate clockwise by degrees"),
            ("turn_left(Player, 15)".into(), "Rotate counter-clockwise by degrees"),
            ("point_towards(Player, \"mouse\")".into(), "Face towards mouse pointer or sprite"),
            ("bounce_on_edge(Player)".into(), "Bounce off stage boundaries"),
            ("set_rotation_style(Player, \"left-right\")".into(), "Set rotation constraint"),
        ],
        BlockCategory::Looks => vec![
            ("say_for(Player, \"Hello!\", 2)".into(), "Show speech bubble for duration"),
            ("say(Player, \"Hello!\")".into(), "Show speech bubble indefinitely"),
            ("think_for(Player, \"Hmm...\", 2)".into(), "Show thought bubble for duration"),
            ("show(Player)".into(), "Make sprite visible"),
            ("hide(Player)".into(), "Hide sprite from stage"),
            ("set_size(Player, 100)".into(), "Set scale percentage"),
            ("change_size(Player, 10)".into(), "Increase or decrease scale"),
            ("switch_costume(Player, \"costume2\")".into(), "Switch sprite costume"),
            ("background.set(\"forest\")".into(), "Set stage background color/theme"),
        ],
        BlockCategory::Audio => vec![
            ("sound.play(\"coin\")".into(), "Play sound effect"),
            ("sound.play_until_done(\"victory\")".into(), "Play sound and wait until finished"),
            ("sound.stop_all()".into(), "Stop all sound channels"),
            ("sound.set_volume(100)".into(), "Set master volume percentage"),
            ("music.play_note(60, 0.5)".into(), "Synthesize MIDI musical note"),
        ],
        BlockCategory::Events => vec![
            ("when start:".into(), "Runs once when game boots up"),
            ("when action.down(\"right\"):".into(), "Runs every frame key/action is held"),
            ("when action.press(\"jump\"):".into(), "Runs once when key is pressed down"),
            ("when Player touches Coin:".into(), "Fires on collision overlap"),
            ("when message(\"game_over\"):".into(), "Fires when broadcast is received"),
            ("broadcast(\"start_level\")".into(), "Broadcast custom event message"),
            ("every 3 seconds:".into(), "Recurring timer trigger"),
        ],
        BlockCategory::Control => vec![
            ("repeat 10:\n    move(Player, 5)".into(), "Loop statement executing body N times"),
            ("if score >= 100:\n    broadcast(\"win\")".into(), "Conditional branching statement"),
            ("wait(1.5)".into(), "Delay execution by seconds"),
            ("stop_all()".into(), "Halt game execution"),
        ],
        BlockCategory::Sensing => vec![
            ("ask(\"What is your name?\")".into(), "Display prompt input box to player"),
            ("get_answer()".into(), "Returns player's input string"),
            ("touching(Player, Coin)".into(), "Check if two sprites are touching"),
            ("key_pressed(\"space\")".into(), "Check if specific key is held"),
            ("mouse_down()".into(), "Check if left mouse button is pressed"),
            ("distance_to(Player, Coin)".into(), "Euclidean distance between sprites"),
        ],
        BlockCategory::Operators => vec![
            ("random(1, 10)".into(), "Uniform random number between min and max"),
            ("text.join(\"Hello, \", get_answer())".into(), "Concatenate two strings"),
            ("math.round(n)".into(), "Round number to nearest whole integer"),
            ("math.sqrt(n)".into(), "Square root of number"),
            ("math.abs(n)".into(), "Absolute value"),
        ],
        BlockCategory::Variables => vec![
            ("score = 0".into(), "Set variable value"),
            ("score += 1".into(), "Increment variable by value"),
            ("health -= 1".into(), "Decrement variable by value"),
            ("variable.show(\"score\")".into(), "Display variable HUD badge on stage"),
            ("variable.hide(\"score\")".into(), "Hide variable HUD badge from stage"),
        ],
        _ => vec![
            ("list.add(\"inventory\", \"sword\")".into(), "Append item to dynamic list"),
            ("list.delete(\"inventory\", 1)".into(), "Remove item at index"),
            ("list.item(\"inventory\", 1)".into(), "Query item at index"),
            ("list.length(\"inventory\")".into(), "Get count of items in list"),
            ("list.show(\"inventory\")".into(), "Display list monitor on stage"),
        ],
    }
}

pub fn run_studio_native(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, config) = project_cmd::resolve_entry_file(path)?;
    println!("Loading Scratch Studio Desktop IDE for '{}'...", config.name);
    println!("Source: {}", entry_path.display());

    let app = StudioApp::new(entry_path, config.clone())?;

    let window_title = format!("Scratch Studio - {}", config.name);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 820.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title(window_title),
        ..Default::default()
    };

    println!("============================================================");
    println!(" Scratch Studio Desktop IDE Running!");
    println!(" Project:  {}", config.name);
    println!(" Window:   Native Rust Desktop Window (1280x820)");
    println!(" Panels:   Block Palette | Code Editor | Live Stage | Inspector");
    println!(" Close the window or press Alt+F4 to exit.");
    println!("============================================================");

    eframe::run_native(
        "scratch_studio",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )?;

    Ok(())
}
