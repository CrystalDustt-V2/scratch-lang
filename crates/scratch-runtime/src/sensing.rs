use crate::value::RuntimeValue;
use crate::world::World;

pub struct SensingSystem;

impl SensingSystem {
    pub fn touching_color(world: &World, target: &str, color_query: &str) -> bool {
        let target_ent = match world.get_entity_by_name(target) {
            Some(e) if e.visible => e,
            _ => return false,
        };

        // Check if stage background matches color
        if color_matches_named(&world.background, color_query) {
            // Check if entity is near edges
            if target_ent.transform.x <= 20.0
                || target_ent.transform.x >= 1260.0
                || target_ent.transform.y <= 20.0
                || target_ent.transform.y >= 700.0
            {
                return true;
            }
        }

        let half_w_a = target_ent.size[0] / 2.0;
        let half_h_a = target_ent.size[1] / 2.0;

        for other in world.iter_entities() {
            if other.id == target_ent.id || !other.visible {
                continue;
            }

            let half_w_b = other.size[0] / 2.0;
            let half_h_b = other.size[1] / 2.0;

            let overlap_x = (target_ent.transform.x - other.transform.x).abs() <= (half_w_a + half_w_b);
            let overlap_y = (target_ent.transform.y - other.transform.y).abs() <= (half_h_a + half_h_b);

            if overlap_x && overlap_y {
                if color_matches(other.color, color_query)
                    || other.tags.iter().any(|t| t.eq_ignore_ascii_case(color_query))
                    || other.name.to_lowercase().contains(&color_query.to_lowercase())
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn color_touching_color(world: &World, color1: &str, color2: &str) -> bool {
        let entities: Vec<_> = world.iter_entities().filter(|e| e.visible).collect();

        for i in 0..entities.len() {
            let a = entities[i];
            let matches_c1 = color_matches(a.color, color1)
                || a.tags.iter().any(|t| t.eq_ignore_ascii_case(color1))
                || a.name.to_lowercase().contains(&color1.to_lowercase());

            if !matches_c1 {
                continue;
            }

            let half_w_a = a.size[0] / 2.0;
            let half_h_a = a.size[1] / 2.0;

            for j in 0..entities.len() {
                if i == j {
                    continue;
                }
                let b = entities[j];
                let matches_c2 = color_matches(b.color, color2)
                    || b.tags.iter().any(|t| t.eq_ignore_ascii_case(color2))
                    || b.name.to_lowercase().contains(&color2.to_lowercase());

                if !matches_c2 {
                    continue;
                }

                let half_w_b = b.size[0] / 2.0;
                let half_h_b = b.size[1] / 2.0;

                let overlap_x = (a.transform.x - b.transform.x).abs() <= (half_w_a + half_w_b);
                let overlap_y = (a.transform.y - b.transform.y).abs() <= (half_h_a + half_h_b);

                if overlap_x && overlap_y {
                    return true;
                }
            }
        }

        false
    }

    pub fn property_of(world: &World, target: &str, property: &str) -> RuntimeValue {
        if let Some(ent) = world.get_entity_by_name(target) {
            match property.to_lowercase().as_str() {
                "x position" | "x" | "x_position" => RuntimeValue::Number(ent.transform.x as f64),
                "y position" | "y" | "y_position" => RuntimeValue::Number(ent.transform.y as f64),
                "direction" | "rotation" => RuntimeValue::Number(ent.transform.rotation as f64),
                "layer" => RuntimeValue::Number(ent.layer as f64),
                "costume #" | "costume_number" | "costume" => RuntimeValue::Number(1.0),
                "size" | "scale" => RuntimeValue::Number((ent.transform.scale_x * 100.0).round() as f64),
                "volume" => world.get_var("__master_volume").cloned().unwrap_or(RuntimeValue::Number(100.0)),
                var_name => world.get_var(var_name).cloned().unwrap_or(RuntimeValue::Nil),
            }
        } else {
            world.get_var(property).cloned().unwrap_or(RuntimeValue::Nil)
        }
    }

    pub fn go_to_front(world: &mut World, target: &str) {
        let max_layer = world.iter_entities().map(|e| e.layer).max().unwrap_or(0);
        if let Some(ent) = world.get_entity_by_name_mut(target) {
            ent.layer = max_layer + 1;
        }
    }

    pub fn go_back_layers(world: &mut World, target: &str, count: i32) {
        if let Some(ent) = world.get_entity_by_name_mut(target) {
            ent.layer -= count;
        }
    }

    pub fn get_loudness(world: &World) -> f64 {
        world
            .get_var("__loudness")
            .and_then(|v| v.as_number())
            .unwrap_or(0.0)
    }

    pub fn set_drag_mode(world: &mut World, target: &str, mode: &str) {
        let draggable = mode.eq_ignore_ascii_case("draggable") || mode.eq_ignore_ascii_case("true");
        if let Some(ent) = world.get_entity_by_name_mut(target) {
            ent.draggable = draggable;
        }
    }

    pub fn current(unit: &str) -> f64 {
        let dur = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let total_secs = dur.as_secs();
        match unit.to_lowercase().as_str() {
            "second" | "seconds" => (total_secs % 60) as f64,
            "minute" | "minutes" => ((total_secs / 60) % 60) as f64,
            "hour" | "hours" => ((total_secs / 3600) % 24) as f64,
            "day of week" | "day_of_week" => (((total_secs / 86400) + 4) % 7 + 1) as f64,
            "date" | "days" => {
                let days_since_1970 = total_secs / 86400;
                ((days_since_1970 % 30) + 1) as f64
            }
            "month" => {
                let days_since_1970 = total_secs / 86400;
                (((days_since_1970 / 30) % 12) + 1) as f64
            }
            "year" => {
                let years = 1970 + total_secs / (86400 * 365);
                years as f64
            }
            _ => 0.0,
        }
    }

    pub fn days_since_2000() -> f64 {
        let dur = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let secs_since_1970 = dur.as_secs() as f64;
        let secs_from_1970_to_2000 = 946684800.0;
        let days = (secs_since_1970 - secs_from_1970_to_2000) / 86400.0;
        days.max(0.0)
    }

    pub fn get_username() -> String {
        std::env::var("USERNAME")
            .or_else(|_| std::env::var("USER"))
            .unwrap_or_else(|_| "scratch_user".to_string())
    }
}

fn color_matches(rgba: [f32; 4], query: &str) -> bool {
    let q = query.to_lowercase();
    match q.as_str() {
        "red" => rgba[0] > 0.6 && rgba[1] < 0.4 && rgba[2] < 0.4,
        "green" => rgba[1] > 0.6 && rgba[0] < 0.4 && rgba[2] < 0.4,
        "blue" => rgba[2] > 0.6 && rgba[0] < 0.4 && rgba[1] < 0.4,
        "white" => rgba[0] > 0.8 && rgba[1] > 0.8 && rgba[2] > 0.8,
        "black" => rgba[0] < 0.2 && rgba[1] < 0.2 && rgba[2] < 0.2,
        "yellow" => rgba[0] > 0.6 && rgba[1] > 0.6 && rgba[2] < 0.4,
        hex if hex.starts_with('#') => {
            if hex.len() == 7 {
                let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0) as f32 / 255.0;
                let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0) as f32 / 255.0;
                let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0) as f32 / 255.0;
                (rgba[0] - r).abs() < 0.15 && (rgba[1] - g).abs() < 0.15 && (rgba[2] - b).abs() < 0.15
            } else {
                false
            }
        }
        _ => false,
    }
}

fn color_matches_named(bg: &str, query: &str) -> bool {
    bg.eq_ignore_ascii_case(query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensing_system() {
        let mut world = World::new();
        world.spawn_entity("Player");
        world.spawn_entity("RedBlock");

        if let Some(r) = world.get_entity_by_name_mut("RedBlock") {
            r.color = [1.0, 0.0, 0.0, 1.0];
            r.transform.x = 0.0;
            r.transform.y = 0.0;
        }

        // Both at (0, 0) overlapping, RedBlock is red
        assert!(SensingSystem::touching_color(&world, "Player", "red"));
        assert!(!SensingSystem::touching_color(&world, "Player", "green"));

        // Property of
        assert_eq!(SensingSystem::property_of(&world, "Player", "x position"), RuntimeValue::Number(0.0));
        assert_eq!(SensingSystem::property_of(&world, "Player", "size"), RuntimeValue::Number(100.0));

        // Layers
        SensingSystem::go_to_front(&mut world, "Player");
        assert_eq!(world.get_entity_by_name("Player").unwrap().layer, 1);
        SensingSystem::go_back_layers(&mut world, "Player", 2);
        assert_eq!(world.get_entity_by_name("Player").unwrap().layer, -1);
    }
}
