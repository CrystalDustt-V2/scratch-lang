use std::collections::HashMap;
use crate::block::BlockDefinition;
use crate::types::{BlockCategory, BlockType};

#[derive(Debug, Clone, Default)]
pub struct BlockRegistry {
    blocks: HashMap<String, BlockDefinition>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    /// Creates a BlockRegistry populated with the standard core game blocks.
    pub fn core() -> Self {
        let mut registry = Self::new();

        // 1. Movement: move(target, x, [y])
        registry.register(
            BlockDefinition::new("move", BlockCategory::Movement, "Move an object by x and optional y pixels", BlockType::Void, "MovementSystem::move")
                .with_param("target", BlockType::Object, false, None, "Target object or group to move")
                .with_param("x", BlockType::Number, false, None, "Horizontal delta in pixels")
                .with_param("y", BlockType::Number, true, Some("0"), "Vertical delta in pixels")
                .with_doc("Moves the specified target object horizontally by x and vertically by y pixels.")
                .with_example("move(Player, 5)")
                .with_example("move(Player, -5, 2)")
        );

        // 2. Movement: jump(target, force)
        registry.register(
            BlockDefinition::new("jump", BlockCategory::Movement, "Apply upward jump force to an object", BlockType::Void, "MovementSystem::jump")
                .with_param("target", BlockType::Object, false, None, "Target object to jump")
                .with_param("force", BlockType::Number, false, Some("10"), "Upward jump force magnitude")
                .with_doc("Applies an immediate upward vertical impulse to the object.")
                .with_example("jump(Player, 12)")
        );

        // 3. Movement: stop(target)
        registry.register(
            BlockDefinition::new("stop", BlockCategory::Movement, "Immediately stop movement of an object", BlockType::Void, "MovementSystem::stop")
                .with_param("target", BlockType::Object, false, None, "Target object to stop")
                .with_doc("Sets the target's linear velocity to zero.")
                .with_example("stop(Player)")
        );

        // 4. Movement: teleport(target, x, y)
        registry.register(
            BlockDefinition::new("teleport", BlockCategory::Movement, "Teleport object directly to coordinates", BlockType::Void, "MovementSystem::teleport")
                .with_param("target", BlockType::Object, false, None, "Target object")
                .with_param("x", BlockType::Number, false, None, "New X coordinate")
                .with_param("y", BlockType::Number, false, None, "New Y coordinate")
                .with_doc("Instantly sets the object position to (x, y).")
                .with_example("teleport(Player, 100, 300)")
        );

        // 5. Conditions: touching(target, other)
        registry.register(
            BlockDefinition::new("touching", BlockCategory::Condition, "Check if two objects or groups are colliding", BlockType::Boolean, "CollisionSystem::touching")
                .with_param("target", BlockType::Object, false, None, "First object or group")
                .with_param("other", BlockType::Object, false, None, "Second object or group")
                .with_doc("Returns true if target's collider overlaps other's collider.")
                .with_example("if touching(Player, Enemy):")
        );

        // 6. Gameplay: damage(target, amount)
        registry.register(
            BlockDefinition::new("damage", BlockCategory::Gameplay, "Deal damage to target", BlockType::Void, "GameplaySystem::damage")
                .with_param("target", BlockType::Object, false, None, "Target object")
                .with_param("amount", BlockType::Number, false, Some("1"), "Amount of damage")
                .with_doc("Reduces the target's health by the specified amount.")
                .with_example("damage(Player, 1)")
        );

        // 7. Gameplay: heal(target, amount)
        registry.register(
            BlockDefinition::new("heal", BlockCategory::Gameplay, "Restore health to target", BlockType::Void, "GameplaySystem::heal")
                .with_param("target", BlockType::Object, false, None, "Target object")
                .with_param("amount", BlockType::Number, false, Some("1"), "Amount of health to restore")
                .with_doc("Increases the target's health by the specified amount.")
                .with_example("heal(Player, 1)")
        );

        // 8. Gameplay: respawn(target)
        registry.register(
            BlockDefinition::new("respawn", BlockCategory::Gameplay, "Respawn target at initial position", BlockType::Void, "GameplaySystem::respawn")
                .with_param("target", BlockType::Object, false, None, "Target object")
                .with_doc("Resets the target to its original spawn position and default properties.")
                .with_example("respawn(Player)")
        );

        // 9. Gameplay: collect(target)
        registry.register(
            BlockDefinition::new("collect", BlockCategory::Gameplay, "Collect and remove collectible object", BlockType::Void, "GameplaySystem::collect")
                .with_param("target", BlockType::Object, false, None, "Collectible object")
                .with_doc("Hides or destroys the collectible object.")
                .with_example("collect(Coin)")
        );

        // 10. Audio: sound.play(name)
        registry.register(
            BlockDefinition::new("sound.play", BlockCategory::Audio, "Play a sound effect by name", BlockType::Void, "AudioSystem::play")
                .with_param("name", BlockType::String, false, None, "Sound asset name")
                .with_doc("Plays the specified audio asset.")
                .with_example("sound.play(\"coin\")")
        );

        // 11. Camera: camera.follow(target)
        registry.register(
            BlockDefinition::new("camera.follow", BlockCategory::Camera, "Make camera follow target object", BlockType::Void, "CameraSystem::follow")
                .with_param("target", BlockType::Object, false, None, "Target object to track")
                .with_doc("Configures the main 2D camera to smoothly follow the target entity.")
                .with_example("camera.follow(Player)")
        );

        // 12. Background: background.set(name_or_color)
        registry.register(
            BlockDefinition::new("background.set", BlockCategory::Scene, "Set scene background color or image", BlockType::Void, "SceneSystem::setBackground")
                .with_param("value", BlockType::String, false, None, "Color name (e.g. \"white\") or asset name")
                .with_doc("Updates the background.")
                .with_example("background.set(\"forest\")")
        );

        registry
    }

    pub fn register(&mut self, block: BlockDefinition) {
        self.blocks.insert(block.name.clone(), block);
    }

    pub fn get(&self, name: &str) -> Option<&BlockDefinition> {
        self.blocks.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.blocks.contains_key(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &BlockDefinition> {
        self.blocks.values()
    }

    /// Finds candidate matches for a misspelled block name using Levenshtein distance.
    pub fn find_similar<'a>(&'a self, query: &str) -> Option<&'a str> {
        let mut closest_name = None;
        let mut min_distance = usize::MAX;

        for name in self.blocks.keys() {
            let dist = levenshtein(query, name);
            if dist < min_distance && dist <= 3 {
                min_distance = dist;
                closest_name = Some(name.as_str());
            }
        }

        closest_name
    }
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let (m, n) = (a_chars.len(), b_chars.len());

    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1].to_ascii_lowercase() == b_chars[j - 1].to_ascii_lowercase() {
                0
            } else {
                1
            };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[m][n]
}
