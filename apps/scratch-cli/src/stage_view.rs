use eframe::egui;
use scratch_project::ProjectConfig;
use scratch_runtime::Runtime;
use std::time::Instant;

pub struct StageRenderer {
    pub ask_buffer: String,
    pub notifications: Vec<(String, Instant)>,
}

impl Default for StageRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl StageRenderer {
    pub fn new() -> Self {
        Self {
            ask_buffer: String::new(),
            notifications: Vec::new(),
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        runtime: &mut Runtime,
        config: &ProjectConfig,
        available_size: egui::Vec2,
        interactive: bool,
    ) {
        // Drain pending messages from runtime for visual notifications
        let pending = runtime.world.take_messages();
        for msg in pending {
            if let Some(snd) = msg.strip_prefix("__sound:") {
                self.notifications.push((format!("🔊 Sound: {}", snd), Instant::now()));
            } else if let Some(scene) = msg.strip_prefix("__scene_switch:") {
                self.notifications.push((format!("🎬 Scene: {}", scene), Instant::now()));
                // If scene switches, update world background
                if scene == "forest" || scene == "level2" {
                    runtime.world.background = "forest".to_string();
                }
            } else {
                self.notifications.push((format!("📢 Broadcast: {}", msg), Instant::now()));
            }
        }
        while let Some(txt) = runtime.world.tts_speech_queue.pop() {
            self.notifications.push((format!("🗣️ TTS: \"{}\"", txt), Instant::now()));
        }
        while let Some(ev) = runtime.world.music_events.pop() {
            if ev.kind == "note" {
                self.notifications.push((format!("🎵 Note: MIDI {:.0} ({:.2} beats)", ev.value, ev.beats), Instant::now()));
            } else {
                self.notifications.push((format!("🥁 Drum: #{:.0} ({:.2} beats)", ev.value, ev.beats), Instant::now()));
            }
        }
        // Retain notifications from the last 2.5 seconds
        self.notifications.retain(|(_, time)| time.elapsed().as_secs_f32() < 2.5);

        // Aspect ratio calculation from project resolution / active ratio preset
        let (target_w, target_h) = if config.resolution.width > 0 && config.resolution.height > 0 {
            (config.resolution.width as f32, config.resolution.height as f32)
        } else {
            (1280.0, 720.0)
        };
        let aspect = target_w / target_h.max(1.0);
        runtime.world.stage_width = target_w;
        runtime.world.stage_height = target_h;

        let avail_w = available_size.x.max(100.0);
        let avail_h = available_size.y.max(100.0);

        let mut stage_w = avail_w;
        let mut stage_h = stage_w / aspect;
        if stage_h > avail_h {
            stage_h = avail_h;
            stage_w = stage_h * aspect;
        }

        // Center stage within available viewport area
        let offset_x = (avail_w - stage_w) / 2.0;
        let offset_y = (avail_h - stage_h) / 2.0;

        let (response, mut painter) = ui.allocate_painter(egui::vec2(avail_w, avail_h), egui::Sense::click_and_drag());
        let stage_rect = egui::Rect::from_min_size(
            egui::pos2(response.rect.min.x + offset_x, response.rect.min.y + offset_y),
            egui::vec2(stage_w, stage_h),
        );

        // Background color
        let bg_color = parse_background_color(&runtime.world.background);
        painter.rect_filled(stage_rect, 6.0, bg_color);

        // Physical 16:9 Game Border Frame with corner accents
        let border_stroke = egui::Stroke::new(3.0, egui::Color32::from_rgb(59, 130, 246));
        painter.rect_stroke(stage_rect, 6.0, border_stroke, egui::StrokeKind::Outside);

        let accent_len = 18.0;
        let accent_stroke = egui::Stroke::new(2.5, egui::Color32::from_rgb(147, 197, 253));
        // Top-left
        painter.line_segment([stage_rect.min, stage_rect.min + egui::vec2(accent_len, 0.0)], accent_stroke);
        painter.line_segment([stage_rect.min, stage_rect.min + egui::vec2(0.0, accent_len)], accent_stroke);
        // Top-right
        let tr = egui::pos2(stage_rect.max.x, stage_rect.min.y);
        painter.line_segment([tr, tr - egui::vec2(accent_len, 0.0)], accent_stroke);
        painter.line_segment([tr, tr + egui::vec2(0.0, accent_len)], accent_stroke);
        // Bottom-left
        let bl = egui::pos2(stage_rect.min.x, stage_rect.max.y);
        painter.line_segment([bl, bl + egui::vec2(accent_len, 0.0)], accent_stroke);
        painter.line_segment([bl, bl - egui::vec2(0.0, accent_len)], accent_stroke);
        // Bottom-right
        painter.line_segment([stage_rect.max, stage_rect.max - egui::vec2(accent_len, 0.0)], accent_stroke);
        painter.line_segment([stage_rect.max, stage_rect.max - egui::vec2(0.0, accent_len)], accent_stroke);

        // Clip anything that exceeds the physical 16:9 stage boundary
        painter.set_clip_rect(stage_rect);

        // Grid dots or subtle star particles for night
        if runtime.world.background == "night" {
            draw_stars(&painter, stage_rect);
        }

        // Coordinate scaling
        let scale_x = stage_rect.width() / target_w;
        let scale_y = stage_rect.height() / target_h;
        let scale = scale_x.min(scale_y);

        // Determine if coordinates are centered (Scratch standard) or corner-based
        let center_mode = !runtime.world.is_corner_mode();

        let map_to_screen = |x: f32, y: f32| -> egui::Pos2 {
            if center_mode {
                egui::Pos2::new(
                    stage_rect.center().x + x * scale,
                    stage_rect.center().y - y * scale,
                )
            } else {
                egui::Pos2::new(
                    stage_rect.min.x + x * scale,
                    stage_rect.max.y - y * scale,
                )
            }
        };

        // Handle Interactive Inputs
        if interactive {
            ui.input(|i| {
                // Actions mapping: Up is decoupled from Jump so holding Up moves smoothly!
                let right = i.key_down(egui::Key::ArrowRight) || i.key_down(egui::Key::D);
                let left = i.key_down(egui::Key::ArrowLeft) || i.key_down(egui::Key::A);
                let up = i.key_down(egui::Key::ArrowUp) || i.key_down(egui::Key::W);
                let down = i.key_down(egui::Key::ArrowDown) || i.key_down(egui::Key::S);
                let jump = i.key_down(egui::Key::Space);

                runtime.world.set_action_down("right", right);
                runtime.world.set_action_down("left", left);
                runtime.world.set_action_down("up", up);
                runtime.world.set_action_down("down", down);
                runtime.world.set_action_down("jump", jump);

                // Specific key alias mappings for script authoring flexibility
                runtime.world.set_action_down("w", i.key_down(egui::Key::W));
                runtime.world.set_action_down("s", i.key_down(egui::Key::S));
                runtime.world.set_action_down("a", i.key_down(egui::Key::A));
                runtime.world.set_action_down("d", i.key_down(egui::Key::D));
                runtime.world.set_action_down("ArrowUp", i.key_down(egui::Key::ArrowUp));
                runtime.world.set_action_down("ArrowDown", i.key_down(egui::Key::ArrowDown));
                runtime.world.set_action_down("ArrowLeft", i.key_down(egui::Key::ArrowLeft));
                runtime.world.set_action_down("ArrowRight", i.key_down(egui::Key::ArrowRight));
                runtime.world.set_action_down("space", jump);

                // Mouse sensing
                if let Some(pos) = i.pointer.hover_pos() {
                    if stage_rect.contains(pos) {
                        let game_x = if center_mode {
                            (pos.x - stage_rect.center().x) / scale
                        } else {
                            (pos.x - stage_rect.min.x) / scale
                        };
                        let game_y = if center_mode {
                            (stage_rect.center().y - pos.y) / scale
                        } else {
                            (stage_rect.max.y - pos.y) / scale
                        };
                        runtime.world.mouse_pos = (game_x, game_y);
                    }
                }
                runtime.world.mouse_down = i.pointer.primary_down();
            });
        }

        // Render Pen Drawing Trails
        for stroke in &runtime.world.pen_strokes {
            let p1 = map_to_screen(stroke.x1, stroke.y1);
            let p2 = map_to_screen(stroke.x2, stroke.y2);
            let col = egui::Color32::from_rgba_premultiplied(
                (stroke.color[0] * 255.0) as u8,
                (stroke.color[1] * 255.0) as u8,
                (stroke.color[2] * 255.0) as u8,
                (stroke.color[3] * 255.0) as u8,
            );
            painter.line_segment([p1, p2], egui::Stroke::new(stroke.size * scale, col));
        }

        // Render Pen Stamps
        for stamp in &runtime.world.pen_stamps {
            let p = map_to_screen(stamp.x, stamp.y);
            let stamp_w = (32.0 * stamp.scale_x * scale).max(12.0);
            let stamp_h = (32.0 * stamp.scale_y * scale).max(12.0);
            let stamp_rect = egui::Rect::from_center_size(p, egui::Vec2::new(stamp_w, stamp_h));
            let col = egui::Color32::from_rgba_premultiplied(
                (stamp.color[0] * 255.0 * 0.75) as u8,
                (stamp.color[1] * 255.0 * 0.75) as u8,
                (stamp.color[2] * 255.0 * 0.75) as u8,
                (stamp.color[3] * 255.0 * 0.75) as u8,
            );
            painter.rect_filled(stamp_rect, 4.0, col);
        }

        // Render Entities
        for ent in runtime.world.iter_entities() {
            if !ent.visible {
                continue;
            }

            let center = map_to_screen(ent.transform.x, ent.transform.y);
            let ent_w = (ent.size[0] * ent.transform.scale_x * scale).max(16.0);
            let ent_h = (ent.size[1] * ent.transform.scale_y * scale).max(16.0);
            let ent_rect = egui::Rect::from_center_size(center, egui::Vec2::new(ent_w, ent_h));

            // Visual rendering per entity type
            let (fill_col, stroke_col) = get_entity_colors(ent, &ent.name);

            // Sprite body
            painter.rect_filled(ent_rect, 6.0, fill_col);
            painter.rect_stroke(ent_rect, 6.0, egui::Stroke::new(1.5, stroke_col), egui::StrokeKind::Outside);

            // Character details
            if ent.name == "Player" {
                draw_player_face(&painter, ent_rect);
            } else if ent.name == "Coin" {
                draw_coin_symbol(&painter, ent_rect);
            } else if ent.name.contains("Goal") || ent.name.contains("Flag") {
                draw_flag_symbol(&painter, ent_rect);
            }

            // Name label below sprite
            let name_pos = egui::Pos2::new(center.x, ent_rect.max.y + 10.0);
            painter.text(
                name_pos,
                egui::Align2::CENTER_CENTER,
                &ent.name,
                egui::FontId::proportional(11.0),
                egui::Color32::from_rgb(220, 230, 245),
            );

            // Dialogue speech bubble (say / think)
            let say_key = format!("__say_{}", ent.name);
            let think_key = format!("__think_{}", ent.name);
            if let Some(msg) = runtime.world.get_var(&say_key).and_then(|v| v.as_string()) {
                draw_speech_bubble(&painter, ent_rect, msg, false);
            } else if let Some(msg) = runtime.world.get_var(&think_key).and_then(|v| v.as_string()) {
                draw_speech_bubble(&painter, ent_rect, msg, true);
            }
        }

        // On-screen Variable Monitors (top-left)
        let mut var_y = stage_rect.min.y + 12.0;
        let mut visible_vars: Vec<(String, String)> = Vec::new();
        for var_name in ["score", "health", "time", "coins", "level", "points"] {
            if let Some(val) = runtime.world.get_var(var_name) {
                visible_vars.push((var_name.to_string(), val.to_string()));
            }
        }
        for (name, val) in visible_vars {
            let label = format!("{}: {}", name, val);
            let pill_rect = egui::Rect::from_min_size(
                egui::Pos2::new(stage_rect.min.x + 12.0, var_y),
                egui::Vec2::new(label.len() as f32 * 8.5 + 20.0, 24.0),
            );
            painter.rect_filled(pill_rect, 5.0, egui::Color32::from_rgba_premultiplied(255, 140, 0, 230));
            painter.rect_stroke(pill_rect, 5.0, egui::Stroke::new(1.0, egui::Color32::WHITE), egui::StrokeKind::Outside);
            painter.text(
                pill_rect.center(),
                egui::Align2::CENTER_CENTER,
                &label,
                egui::FontId::monospace(12.0),
                egui::Color32::WHITE,
            );
            var_y += 30.0;
        }

        // On-screen List Monitors (top-right)
        let mut list_y = stage_rect.min.y + 12.0;
        for list_name in ["inventory", "items", "highscores"] {
            if let Some(items) = runtime.world.get_list(list_name) {
                let box_w = 140.0;
                let box_h = 24.0 + (items.len().min(5) as f32 * 18.0);
                let list_rect = egui::Rect::from_min_size(
                    egui::Pos2::new(stage_rect.max.x - box_w - 12.0, list_y),
                    egui::Vec2::new(box_w, box_h),
                );
                painter.rect_filled(list_rect, 6.0, egui::Color32::from_rgba_premultiplied(220, 50, 80, 230));
                painter.rect_stroke(list_rect, 6.0, egui::Stroke::new(1.0, egui::Color32::WHITE), egui::StrokeKind::Outside);

                // Header
                painter.text(
                    egui::Pos2::new(list_rect.center().x, list_rect.min.y + 12.0),
                    egui::Align2::CENTER_CENTER,
                    format!("📋 {} ({})", list_name, items.len()),
                    egui::FontId::proportional(11.0),
                    egui::Color32::WHITE,
                );
                // Items
                for (idx, itm) in items.iter().take(5).enumerate() {
                    let item_pos = egui::Pos2::new(list_rect.min.x + 8.0, list_rect.min.y + 28.0 + (idx as f32 * 17.0));
                    painter.text(
                        item_pos,
                        egui::Align2::LEFT_CENTER,
                        format!("{}. {}", idx + 1, itm),
                        egui::FontId::proportional(11.0),
                        egui::Color32::from_rgb(255, 240, 245),
                    );
                }
                list_y += box_h + 10.0;
            }
        }

        // Interactive "Ask" Prompt Bar
        if let Some(prompt_q) = &runtime.world.active_prompt.clone() {
            let prompt_w = (stage_rect.width() - 40.0).max(200.0);
            let prompt_h = 42.0;
            let prompt_rect = egui::Rect::from_min_size(
                egui::Pos2::new(stage_rect.center().x - prompt_w / 2.0, stage_rect.max.y - prompt_h - 15.0),
                egui::Vec2::new(prompt_w, prompt_h),
            );
            painter.rect_filled(prompt_rect, 8.0, egui::Color32::from_rgba_premultiplied(15, 23, 42, 240));
            painter.rect_stroke(prompt_rect, 8.0, egui::Stroke::new(1.5, egui::Color32::from_rgb(56, 189, 248)), egui::StrokeKind::Outside);

            let question_text = format!("💬 {}: ", prompt_q);
            painter.text(
                egui::Pos2::new(prompt_rect.min.x + 12.0, prompt_rect.center().y),
                egui::Align2::LEFT_CENTER,
                &question_text,
                egui::FontId::proportional(13.0),
                egui::Color32::WHITE,
            );

            // Submitting answer via Enter key or direct check
            if interactive {
                ui.input(|i| {
                    if i.key_pressed(egui::Key::Enter) && !self.ask_buffer.trim().is_empty() {
                        let ans = self.ask_buffer.trim().to_string();
                        runtime.world.submit_answer(ans);
                        runtime.world.broadcast("answered");
                        self.ask_buffer.clear();
                    }
                });
            }
        }

        // Floating Toast Notifications (Sounds, Broadcasts)
        let mut toast_y = stage_rect.min.y + 12.0;
        for (text, _) in &self.notifications {
            let toast_w = text.len() as f32 * 7.5 + 24.0;
            let toast_rect = egui::Rect::from_min_size(
                egui::Pos2::new(stage_rect.center().x - toast_w / 2.0, toast_y),
                egui::Vec2::new(toast_w, 26.0),
            );
            painter.rect_filled(toast_rect, 13.0, egui::Color32::from_rgba_premultiplied(30, 41, 59, 230));
            painter.rect_stroke(toast_rect, 13.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(148, 163, 184)), egui::StrokeKind::Outside);
            painter.text(
                toast_rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::proportional(12.0),
                egui::Color32::from_rgb(241, 245, 249),
            );
            toast_y += 32.0;
        }
    }
}

fn parse_background_color(bg: &str) -> egui::Color32 {
    match bg.to_lowercase().as_str() {
        "forest" => egui::Color32::from_rgb(20, 52, 35),
        "night" => egui::Color32::from_rgb(11, 19, 43),
        "sky" | "blue" => egui::Color32::from_rgb(56, 189, 248),
        "white" => egui::Color32::from_rgb(248, 250, 252),
        "black" => egui::Color32::from_rgb(15, 23, 42),
        "space" => egui::Color32::from_rgb(5, 8, 20),
        "desert" => egui::Color32::from_rgb(180, 130, 70),
        hex if hex.starts_with('#') => {
            let clean = hex.trim_start_matches('#');
            if clean.len() == 6 {
                let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(240);
                let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(240);
                let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(240);
                egui::Color32::from_rgb(r, g, b)
            } else {
                egui::Color32::from_rgb(248, 250, 252)
            }
        }
        _ => egui::Color32::from_rgb(30, 41, 59),
    }
}

fn draw_stars(painter: &egui::Painter, rect: egui::Rect) {
    let stars = [
        (0.15, 0.2), (0.35, 0.1), (0.6, 0.25), (0.8, 0.15),
        (0.2, 0.5), (0.75, 0.45), (0.45, 0.7), (0.85, 0.8),
    ];
    for &(sx, sy) in &stars {
        let pos = egui::Pos2::new(
            rect.min.x + sx * rect.width(),
            rect.min.y + sy * rect.height(),
        );
        painter.circle_filled(pos, 1.5, egui::Color32::from_rgba_premultiplied(255, 255, 255, 180));
    }
}

fn get_entity_colors(ent: &scratch_runtime::Entity, name: &str) -> (egui::Color32, egui::Color32) {
    if name == "Player" {
        (egui::Color32::from_rgb(59, 130, 246), egui::Color32::from_rgb(147, 197, 253))
    } else if name == "Coin" {
        (egui::Color32::from_rgb(234, 179, 8), egui::Color32::from_rgb(254, 240, 138))
    } else if name.contains("Enemy") || name.contains("Hazard") {
        (egui::Color32::from_rgb(239, 68, 68), egui::Color32::from_rgb(252, 165, 165))
    } else if name.contains("Goal") || name.contains("Flag") {
        (egui::Color32::from_rgb(34, 197, 94), egui::Color32::from_rgb(134, 239, 172))
    } else {
        let r = (ent.color[0] * 255.0) as u8;
        let g = (ent.color[1] * 255.0) as u8;
        let b = (ent.color[2] * 255.0) as u8;
        (egui::Color32::from_rgb(r, g, b), egui::Color32::WHITE)
    }
}

fn draw_player_face(painter: &egui::Painter, rect: egui::Rect) {
    // Cute eye dots
    let eye_y = rect.center().y - 2.0;
    let eye_offset = rect.width() * 0.18;
    let eye_radius = (rect.width() * 0.08).clamp(2.0, 4.0);

    painter.circle_filled(egui::Pos2::new(rect.center().x - eye_offset, eye_y), eye_radius, egui::Color32::WHITE);
    painter.circle_filled(egui::Pos2::new(rect.center().x + eye_offset, eye_y), eye_radius, egui::Color32::WHITE);
    painter.circle_filled(egui::Pos2::new(rect.center().x - eye_offset + 0.5, eye_y), eye_radius * 0.5, egui::Color32::BLACK);
    painter.circle_filled(egui::Pos2::new(rect.center().x + eye_offset + 0.5, eye_y), eye_radius * 0.5, egui::Color32::BLACK);
}

fn draw_coin_symbol(painter: &egui::Painter, rect: egui::Rect) {
    painter.circle_filled(rect.center(), rect.width() * 0.35, egui::Color32::from_rgb(250, 204, 21));
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "★",
        egui::FontId::proportional(12.0),
        egui::Color32::from_rgb(161, 98, 7),
    );
}

fn draw_flag_symbol(painter: &egui::Painter, rect: egui::Rect) {
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "🏁",
        egui::FontId::proportional(14.0),
        egui::Color32::WHITE,
    );
}

fn draw_speech_bubble(painter: &egui::Painter, ent_rect: egui::Rect, text: &str, is_think: bool) {
    let bubble_w = (text.len() as f32 * 7.5 + 16.0).clamp(40.0, 220.0);
    let bubble_h = 24.0;
    let bubble_rect = egui::Rect::from_min_size(
        egui::Pos2::new(ent_rect.center().x - bubble_w / 2.0, ent_rect.min.y - bubble_h - 8.0),
        egui::Vec2::new(bubble_w, bubble_h),
    );

    let roundness = if is_think { 12.0 } else { 6.0 };
    painter.rect_filled(bubble_rect, roundness, egui::Color32::WHITE);
    painter.rect_stroke(bubble_rect, roundness, egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 116, 139)), egui::StrokeKind::Outside);

    // Pointer tail
    let tail_tip = egui::Pos2::new(ent_rect.center().x, ent_rect.min.y - 1.0);
    let tail_left = egui::Pos2::new(ent_rect.center().x - 4.0, bubble_rect.max.y);
    let tail_right = egui::Pos2::new(ent_rect.center().x + 4.0, bubble_rect.max.y);
    painter.add(egui::Shape::convex_polygon(
        vec![tail_left, tail_right, tail_tip],
        egui::Color32::WHITE,
        egui::Stroke::NONE,
    ));

    painter.text(
        bubble_rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(11.0),
        egui::Color32::from_rgb(15, 23, 42),
    );
}
