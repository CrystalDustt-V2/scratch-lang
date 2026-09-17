use crate::value::RuntimeValue;
use crate::world::World;

pub struct ListSystem;

impl ListSystem {
    pub fn add(world: &mut World, list_name: &str, item: RuntimeValue) {
        world.get_list_mut(list_name).push(item);
    }

    pub fn delete(world: &mut World, list_name: &str, index_val: &RuntimeValue) {
        let list = world.get_list_mut(list_name);
        if let Some(s) = index_val.as_string() {
            if s.eq_ignore_ascii_case("all") {
                list.clear();
                return;
            } else if s.eq_ignore_ascii_case("last") {
                list.pop();
                return;
            }
        }

        if let Some(n) = index_val.as_number() {
            let idx = n.round() as i64;
            if idx >= 1 && (idx as usize) <= list.len() {
                list.remove((idx - 1) as usize);
            }
        }
    }

    pub fn insert(world: &mut World, list_name: &str, index_val: &RuntimeValue, item: RuntimeValue) {
        let list = world.get_list_mut(list_name);
        if let Some(s) = index_val.as_string() {
            if s.eq_ignore_ascii_case("last") {
                list.push(item);
                return;
            }
        }

        if let Some(n) = index_val.as_number() {
            let idx = n.round() as i64;
            if idx <= 1 {
                list.insert(0, item);
            } else if (idx as usize) > list.len() {
                list.push(item);
            } else {
                list.insert((idx - 1) as usize, item);
            }
        } else {
            list.push(item);
        }
    }

    pub fn replace(world: &mut World, list_name: &str, index_val: &RuntimeValue, item: RuntimeValue) {
        let list = world.get_list_mut(list_name);
        if let Some(s) = index_val.as_string() {
            if s.eq_ignore_ascii_case("last") {
                if let Some(last) = list.last_mut() {
                    *last = item;
                }
                return;
            }
        }

        if let Some(n) = index_val.as_number() {
            let idx = n.round() as i64;
            if idx >= 1 && (idx as usize) <= list.len() {
                list[(idx - 1) as usize] = item;
            }
        }
    }

    pub fn item(world: &World, list_name: &str, index_val: &RuntimeValue) -> RuntimeValue {
        let list = match world.get_list(list_name) {
            Some(l) if !l.is_empty() => l,
            _ => return RuntimeValue::String(String::new()),
        };

        if let Some(s) = index_val.as_string() {
            if s.eq_ignore_ascii_case("last") {
                return list.last().cloned().unwrap_or(RuntimeValue::Nil);
            } else if s.eq_ignore_ascii_case("random") {
                let len = list.len();
                let seed = world
                    .get_var("__random_seed")
                    .and_then(|v| v.as_number())
                    .unwrap_or(31415.0);
                let next_seed = (seed * 1103515245.0 + 12345.0) % 2147483648.0;
                let rand_idx = ((next_seed / 2147483648.0) * len as f64).floor() as usize;
                let clamped_idx = rand_idx.min(len - 1);
                return list.get(clamped_idx).cloned().unwrap_or(RuntimeValue::Nil);
            }
        }

        if let Some(n) = index_val.as_number() {
            let idx = n.round() as i64;
            if idx >= 1 && (idx as usize) <= list.len() {
                return list[(idx - 1) as usize].clone();
            }
        }

        RuntimeValue::String(String::new())
    }

    pub fn length(world: &World, list_name: &str) -> f64 {
        world.get_list(list_name).map(|l| l.len() as f64).unwrap_or(0.0)
    }

    pub fn contains(world: &World, list_name: &str, item: &RuntimeValue) -> bool {
        let list = match world.get_list(list_name) {
            Some(l) => l,
            None => return false,
        };

        list.iter().any(|elem| values_equal(elem, item))
    }

    pub fn clear(world: &mut World, list_name: &str) {
        world.get_list_mut(list_name).clear();
    }

    pub fn show(world: &mut World, list_name: &str) {
        world.set_var(format!("__list_visible_{}", list_name), RuntimeValue::Bool(true));
    }

    pub fn hide(world: &mut World, list_name: &str) {
        world.set_var(format!("__list_visible_{}", list_name), RuntimeValue::Bool(false));
    }

    pub fn index_of(world: &World, list_name: &str, item: &RuntimeValue) -> f64 {
        let list = match world.get_list(list_name) {
            Some(l) => l,
            None => return 0.0,
        };

        for (i, elem) in list.iter().enumerate() {
            if values_equal(elem, item) {
                return (i + 1) as f64; // Scratch 1-based index
            }
        }
        0.0
    }

    pub fn to_string(world: &World, list_name: &str) -> String {
        let list = match world.get_list(list_name) {
            Some(l) => l,
            None => return String::new(),
        };

        let all_single_char = !list.is_empty() && list.iter().all(|item| {
            if let Some(s) = item.as_string() {
                s.chars().count() == 1
            } else {
                false
            }
        });

        if all_single_char {
            list.iter().map(|item| item.to_string()).collect::<Vec<_>>().join("")
        } else {
            list.iter().map(|item| item.to_string()).collect::<Vec<_>>().join(" ")
        }
    }
}

fn values_equal(a: &RuntimeValue, b: &RuntimeValue) -> bool {
    if a == b {
        return true;
    }
    match (a, b) {
        (RuntimeValue::Number(n), RuntimeValue::String(s))
        | (RuntimeValue::String(s), RuntimeValue::Number(n)) => {
            if let Ok(parsed) = s.parse::<f64>() {
                (parsed - *n).abs() < 1e-9
            } else {
                false
            }
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_crud_operations() {
        let mut world = World::new();

        // 1. Add items
        ListSystem::add(&mut world, "inventory", RuntimeValue::String("sword".to_string()));
        ListSystem::add(&mut world, "inventory", RuntimeValue::String("shield".to_string()));
        ListSystem::add(&mut world, "inventory", RuntimeValue::Number(42.0));

        assert_eq!(ListSystem::length(&world, "inventory"), 3.0);
        assert!(ListSystem::contains(&world, "inventory", &RuntimeValue::String("sword".to_string())));
        assert!(ListSystem::contains(&world, "inventory", &RuntimeValue::Number(42.0)));
        assert!(ListSystem::contains(&world, "inventory", &RuntimeValue::String("42".to_string()))); // Scratch type coercion

        // 2. Query items (1-based)
        assert_eq!(ListSystem::item(&mut world, "inventory", &RuntimeValue::Number(1.0)), RuntimeValue::String("sword".to_string()));
        assert_eq!(ListSystem::item(&mut world, "inventory", &RuntimeValue::Number(2.0)), RuntimeValue::String("shield".to_string()));
        assert_eq!(ListSystem::item(&mut world, "inventory", &RuntimeValue::String("last".to_string())), RuntimeValue::Number(42.0));

        // 3. Insert at position 2
        ListSystem::insert(&mut world, "inventory", &RuntimeValue::Number(2.0), RuntimeValue::String("potion".to_string()));
        assert_eq!(ListSystem::length(&world, "inventory"), 4.0);
        assert_eq!(ListSystem::item(&mut world, "inventory", &RuntimeValue::Number(2.0)), RuntimeValue::String("potion".to_string()));
        assert_eq!(ListSystem::item(&mut world, "inventory", &RuntimeValue::Number(3.0)), RuntimeValue::String("shield".to_string()));

        // 4. Replace item at position 1
        ListSystem::replace(&mut world, "inventory", &RuntimeValue::Number(1.0), RuntimeValue::String("diamond_sword".to_string()));
        assert_eq!(ListSystem::item(&mut world, "inventory", &RuntimeValue::Number(1.0)), RuntimeValue::String("diamond_sword".to_string()));

        // 5. Delete item at position 2 (potion)
        ListSystem::delete(&mut world, "inventory", &RuntimeValue::Number(2.0));
        assert_eq!(ListSystem::length(&world, "inventory"), 3.0);
        assert!(!ListSystem::contains(&world, "inventory", &RuntimeValue::String("potion".to_string())));

        // 6. Delete last
        ListSystem::delete(&mut world, "inventory", &RuntimeValue::String("last".to_string()));
        assert_eq!(ListSystem::length(&world, "inventory"), 2.0);

        // 7. Clear
        ListSystem::clear(&mut world, "inventory");
        assert_eq!(ListSystem::length(&world, "inventory"), 0.0);
        assert_eq!(ListSystem::item(&mut world, "inventory", &RuntimeValue::Number(1.0)), RuntimeValue::String(String::new()));
    }
}
