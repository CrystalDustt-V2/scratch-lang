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

        // 13. Scene: scene.switch(name)
        registry.register(
            BlockDefinition::new("scene.switch", BlockCategory::Scene, "Switch to a new scene by name", BlockType::Void, "SceneSystem::switch")
                .with_param("name", BlockType::String, false, None, "Scene name")
                .with_doc("Transitions execution to the specified scene level.")
                .with_example("scene.switch(\"Level2\")")
        );

        // 14. Scene: scene.restart()
        registry.register(
            BlockDefinition::new("scene.restart", BlockCategory::Scene, "Restart the current scene", BlockType::Void, "SceneSystem::restart")
                .with_doc("Reloads the current scene layout and object states.")
                .with_example("scene.restart()")
        );

        // === MOTION BLOCKS (from Scratch PDF p. 1-2) ===

        // 15. change_x(target, dx)
        registry.register(
            BlockDefinition::new("change_x", BlockCategory::Movement, "Change object X coordinate by delta", BlockType::Void, "MovementSystem::changeX")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("dx", BlockType::Number, false, None, "Horizontal delta in pixels")
                .with_doc("Increases or decreases the target entity's X position.")
                .with_example("change_x(Player, 10)")
        );

        // 16. set_x(target, x)
        registry.register(
            BlockDefinition::new("set_x", BlockCategory::Movement, "Set object X coordinate directly", BlockType::Void, "MovementSystem::setX")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("x", BlockType::Number, false, None, "New X coordinate")
                .with_doc("Directly assigns the target entity's horizontal position.")
                .with_example("set_x(Player, 0)")
        );

        // 17. change_y(target, dy)
        registry.register(
            BlockDefinition::new("change_y", BlockCategory::Movement, "Change object Y coordinate by delta", BlockType::Void, "MovementSystem::changeY")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("dy", BlockType::Number, false, None, "Vertical delta in pixels")
                .with_doc("Increases or decreases the target entity's Y position.")
                .with_example("change_y(Player, 10)")
        );

        // 18. set_y(target, y)
        registry.register(
            BlockDefinition::new("set_y", BlockCategory::Movement, "Set object Y coordinate directly", BlockType::Void, "MovementSystem::setY")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("y", BlockType::Number, false, None, "New Y coordinate")
                .with_doc("Directly assigns the target entity's vertical position.")
                .with_example("set_y(Player, 0)")
        );

        // 19. turn_right(target, degrees)
        registry.register(
            BlockDefinition::new("turn_right", BlockCategory::Movement, "Rotate object clockwise by degrees", BlockType::Void, "MovementSystem::turnRight")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("degrees", BlockType::Number, false, Some("15"), "Degrees to rotate clockwise")
                .with_doc("Rotates the sprite clockwise around its center point.")
                .with_example("turn_right(Player, 15)")
        );

        // 20. turn_left(target, degrees)
        registry.register(
            BlockDefinition::new("turn_left", BlockCategory::Movement, "Rotate object counter-clockwise by degrees", BlockType::Void, "MovementSystem::turnLeft")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("degrees", BlockType::Number, false, Some("15"), "Degrees to rotate counter-clockwise")
                .with_doc("Rotates the sprite counter-clockwise around its center point.")
                .with_example("turn_left(Player, 15)")
        );

        // 21. point_in_direction(target, degrees)
        registry.register(
            BlockDefinition::new("point_in_direction", BlockCategory::Movement, "Set absolute facing angle of sprite", BlockType::Void, "MovementSystem::pointInDirection")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("degrees", BlockType::Number, false, Some("90"), "Absolute direction (0: up, 90: right, 180: down, 270: left)")
                .with_doc("Points the sprite in an exact direction in degrees.")
                .with_example("point_in_direction(Player, 90)")
        );

        // 22. point_towards(target, other)
        registry.register(
            BlockDefinition::new("point_towards", BlockCategory::Movement, "Point sprite towards another entity or mouse", BlockType::Void, "MovementSystem::pointTowards")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("other", BlockType::Object, false, None, "Entity or 'mouse' to face towards")
                .with_doc("Adjusts the target's rotation so it faces directly at the other entity.")
                .with_example("point_towards(Player, Enemy)")
        );

        // 23. bounce_on_edge(target)
        registry.register(
            BlockDefinition::new("bounce_on_edge", BlockCategory::Movement, "Bounce sprite if touching stage boundaries", BlockType::Void, "MovementSystem::bounceOnEdge")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_doc("Inverts the velocity direction if the sprite collides with stage borders.")
                .with_example("bounce_on_edge(Player)")
        );

        // 23b. glide(target, seconds, x, y)
        registry.register(
            BlockDefinition::new("glide", BlockCategory::Movement, "Smoothly glide sprite to (x, y) over duration", BlockType::Void, "MovementSystem::glide")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("seconds", BlockType::Number, false, Some("1"), "Duration of glide in seconds")
                .with_param("x", BlockType::Number, false, None, "Target X coordinate")
                .with_param("y", BlockType::Number, false, None, "Target Y coordinate")
                .with_doc("Smoothly animates the sprite's movement towards (x, y) over the specified time.")
                .with_example("glide(Player, 1, 100, 200)")
        );

        // 23c. go_to(target, destination)
        registry.register(
            BlockDefinition::new("go_to", BlockCategory::Movement, "Move sprite directly to mouse, random position, or entity", BlockType::Void, "MovementSystem::goTo")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("destination", BlockType::String, false, Some("\"mouse\""), "Target destination ('mouse', 'random', or sprite name)")
                .with_doc("Instantly relocates the target to mouse coordinates, random position, or another entity's position.")
                .with_example("go_to(Player, \"mouse\")")
                .with_example("go_to(Player, \"random\")")
                .with_example("go_to(Player, \"Coin\")")
        );

        // 23d. set_rotation_style(target, style)
        registry.register(
            BlockDefinition::new("set_rotation_style", BlockCategory::Movement, "Set rotation style of sprite", BlockType::Void, "MovementSystem::setRotationStyle")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("style", BlockType::String, false, Some("\"all-around\""), "Rotation style ('all-around', 'left-right', or 'don't rotate')")
                .with_doc("Controls how the sprite rotates: all-around (360°), left-right (horizontal flip), or don't rotate.")
                .with_example("set_rotation_style(Player, \"left-right\")")
        );

        // 24. distance_to(target, other) -> Number
        registry.register(
            BlockDefinition::new("distance_to", BlockCategory::Sensing, "Calculate Euclidean distance between two entities", BlockType::Number, "SensingSystem::distanceTo")
                .with_param("target", BlockType::Object, false, None, "First entity")
                .with_param("other", BlockType::Object, false, None, "Second entity")
                .with_doc("Returns the distance in pixels between the centers of two entities.")
                .with_example("if distance_to(Player, Enemy) < 100:")
        );

        // 25. x_position(target) -> Number
        registry.register(
            BlockDefinition::new("x_position", BlockCategory::Movement, "Get X coordinate of target object", BlockType::Number, "MovementSystem::getX")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_doc("Returns the current X position of the target entity.")
                .with_example("current_x = x_position(Player)")
        );

        // 26. y_position(target) -> Number
        registry.register(
            BlockDefinition::new("y_position", BlockCategory::Movement, "Get Y coordinate of target object", BlockType::Number, "MovementSystem::getY")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_doc("Returns the current Y position of the target entity.")
                .with_example("current_y = y_position(Player)")
        );

        // === LOOKS BLOCKS (from Scratch PDF p. 2-3) ===

        // 27. say(target, text)
        registry.register(
            BlockDefinition::new("say", BlockCategory::Looks, "Show speech bubble text above sprite", BlockType::Void, "LooksSystem::say")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("text", BlockType::String, false, None, "Message text to display")
                .with_doc("Renders a speech dialogue bubble above the sprite on stage.")
                .with_example("say(Player, \"Hello, world!\")")
        );

        // 28. say_for(target, text, seconds)
        registry.register(
            BlockDefinition::new("say_for", BlockCategory::Looks, "Show speech bubble text for duration", BlockType::Void, "LooksSystem::sayFor")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("text", BlockType::String, false, None, "Message text")
                .with_param("seconds", BlockType::Number, false, Some("2"), "Duration in seconds")
                .with_doc("Renders a speech bubble above the sprite for a timed duration.")
                .with_example("say_for(Player, \"Nice job!\", 2)")
        );

        // 29. think(target, text)
        registry.register(
            BlockDefinition::new("think", BlockCategory::Looks, "Show thought bubble above sprite", BlockType::Void, "LooksSystem::think")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("text", BlockType::String, false, None, "Thought text")
                .with_doc("Renders a cloud thought bubble above the sprite.")
                .with_example("think(Player, \"Hmm...\")")
        );

        // 30. show(target)
        registry.register(
            BlockDefinition::new("show", BlockCategory::Looks, "Make sprite visible on stage", BlockType::Void, "LooksSystem::show")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_doc("Makes the sprite visible if it was previously hidden.")
                .with_example("show(Player)")
        );

        // 31. hide(target)
        registry.register(
            BlockDefinition::new("hide", BlockCategory::Looks, "Hide sprite from stage", BlockType::Void, "LooksSystem::hide")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_doc("Hides the sprite from the stage without deleting it.")
                .with_example("hide(Player)")
        );

        // 32. set_size(target, percent)
        registry.register(
            BlockDefinition::new("set_size", BlockCategory::Looks, "Set sprite scale percentage", BlockType::Void, "LooksSystem::setSize")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("percent", BlockType::Number, false, Some("100"), "Scale percentage (100 is normal size)")
                .with_doc("Sets the visual scale of the sprite as a percentage.")
                .with_example("set_size(Player, 150)")
        );

        // 33. change_size(target, delta)
        registry.register(
            BlockDefinition::new("change_size", BlockCategory::Looks, "Change sprite scale percentage by delta", BlockType::Void, "LooksSystem::changeSize")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("delta", BlockType::Number, false, Some("10"), "Percentage delta to add or subtract")
                .with_doc("Increases or decreases sprite size percentage.")
                .with_example("change_size(Player, -10)")
        );

        // 34. next_costume(target)
        registry.register(
            BlockDefinition::new("next_costume", BlockCategory::Looks, "Switch sprite to next costume frame", BlockType::Void, "LooksSystem::nextCostume")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_doc("Cycles the sprite appearance to its next costume asset.")
                .with_example("next_costume(Player)")
        );

        // 35. switch_costume(target, name)
        registry.register(
            BlockDefinition::new("switch_costume", BlockCategory::Looks, "Switch sprite to named costume asset", BlockType::Void, "LooksSystem::switchCostume")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("name", BlockType::String, false, None, "Costume / sprite asset name")
                .with_doc("Sets the active costume of the sprite by name.")
                .with_example("switch_costume(Player, \"running\")")
        );

        // 36. clear_effects(target)
        registry.register(
            BlockDefinition::new("clear_effects", BlockCategory::Looks, "Clear all graphical shader effects", BlockType::Void, "LooksSystem::clearEffects")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_doc("Removes color, ghost transparency, and brightness filters from the sprite.")
                .with_example("clear_effects(Player)")
        );

        // 37. set_effect(target, effect, value)
        registry.register(
            BlockDefinition::new("set_effect", BlockCategory::Looks, "Set visual shader effect value", BlockType::Void, "LooksSystem::setEffect")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("effect", BlockType::String, false, None, "Effect name (e.g. \"ghost\", \"brightness\", \"color\")")
                .with_param("value", BlockType::Number, false, Some("0"), "Effect intensity value")
                .with_doc("Applies a visual filter or transparency effect to the sprite.")
                .with_example("set_effect(Player, \"ghost\", 50)")
        );

        // 38. next_backdrop()
        registry.register(
            BlockDefinition::new("next_backdrop", BlockCategory::Scene, "Cycle stage to next background", BlockType::Void, "SceneSystem::nextBackdrop")
                .with_doc("Cycles stage appearance to the next backdrop.")
                .with_example("next_backdrop()")
        );

        // === SOUND & AUDIO BLOCKS (from Scratch PDF p. 3-4) ===

        // 39. sound.stop_all()
        registry.register(
            BlockDefinition::new("sound.stop_all", BlockCategory::Audio, "Stop all playing sounds and music", BlockType::Void, "AudioSystem::stopAll")
                .with_doc("Halts all active sound effects and background music tracks immediately.")
                .with_example("sound.stop_all()")
        );

        // 40. sound.set_volume(percent)
        registry.register(
            BlockDefinition::new("sound.set_volume", BlockCategory::Audio, "Set audio volume percentage", BlockType::Void, "AudioSystem::setVolume")
                .with_param("percent", BlockType::Number, false, Some("100"), "Volume percentage (0 to 100)")
                .with_doc("Sets master audio playback volume.")
                .with_example("sound.set_volume(80)")
        );

        // 41. sound.change_volume(delta)
        registry.register(
            BlockDefinition::new("sound.change_volume", BlockCategory::Audio, "Change audio volume percentage by delta", BlockType::Void, "AudioSystem::changeVolume")
                .with_param("delta", BlockType::Number, false, Some("-10"), "Volume delta to add or subtract")
                .with_doc("Increases or decreases audio playback volume.")
                .with_example("sound.change_volume(-10)")
        );

        // 42. music.play_note(note, beats)
        registry.register(
            BlockDefinition::new("music.play_note", BlockCategory::Audio, "Play musical note for specified beats", BlockType::Void, "AudioSystem::playNote")
                .with_param("note", BlockType::Number, false, Some("60"), "MIDI note number (60 is middle C)")
                .with_param("beats", BlockType::Number, false, Some("0.5"), "Duration in musical beats")
                .with_doc("Synthesizes a musical tone using the active tempo.")
                .with_example("music.play_note(60, 0.5)")
        );

        // 43. music.set_tempo(bpm)
        registry.register(
            BlockDefinition::new("music.set_tempo", BlockCategory::Audio, "Set music tempo in beats per minute", BlockType::Void, "AudioSystem::setTempo")
                .with_param("bpm", BlockType::Number, false, Some("60"), "Tempo in beats per minute")
                .with_doc("Sets the tempo speed for musical notes and percussion.")
                .with_example("music.set_tempo(120)")
        );

        // === SENSING BLOCKS (from Scratch PDF p. 8-9) ===

        // 44. mouse_x() -> Number
        registry.register(
            BlockDefinition::new("mouse_x", BlockCategory::Sensing, "Get current mouse X screen coordinate", BlockType::Number, "SensingSystem::mouseX")
                .with_doc("Returns the horizontal position of the mouse cursor.")
                .with_example("if mouse_x() > 640:")
        );

        // 45. mouse_y() -> Number
        registry.register(
            BlockDefinition::new("mouse_y", BlockCategory::Sensing, "Get current mouse Y screen coordinate", BlockType::Number, "SensingSystem::mouseY")
                .with_doc("Returns the vertical position of the mouse cursor.")
                .with_example("if mouse_y() > 360:")
        );

        // 46. mouse_down() -> Boolean
        registry.register(
            BlockDefinition::new("mouse_down", BlockCategory::Sensing, "Check if left mouse button is pressed", BlockType::Boolean, "SensingSystem::mouseDown")
                .with_doc("Returns true if the user is holding down the left mouse button.")
                .with_example("if mouse_down():")
        );

        // 47. key_pressed(key) -> Boolean
        registry.register(
            BlockDefinition::new("key_pressed", BlockCategory::Sensing, "Check if a key or action is pressed", BlockType::Boolean, "SensingSystem::keyPressed")
                .with_param("key", BlockType::String, false, None, "Action or key name (e.g. \"space\", \"jump\", \"left\")")
                .with_doc("Returns true if the designated key or action is currently active.")
                .with_example("if key_pressed(\"space\"):")
        );

        // 48. get_timer() -> Number
        registry.register(
            BlockDefinition::new("get_timer", BlockCategory::Sensing, "Get elapsed time since game start or reset", BlockType::Number, "SensingSystem::getTimer")
                .with_doc("Returns the number of elapsed seconds on the runtime clock.")
                .with_example("elapsed = get_timer()")
        );

        // 49. reset_timer()
        registry.register(
            BlockDefinition::new("reset_timer", BlockCategory::Sensing, "Reset game timer to zero", BlockType::Void, "SensingSystem::resetTimer")
                .with_doc("Resets the internal elapsed seconds timer to 0.")
                .with_example("reset_timer()")
        );

        // === CONTROL & CLONE BLOCKS (from Scratch PDF p. 7) ===

        // 50. wait(seconds)
        registry.register(
            BlockDefinition::new("wait", BlockCategory::Control, "Wait for a duration in seconds", BlockType::Void, "ControlSystem::wait")
                .with_param("seconds", BlockType::Number, false, Some("1"), "Time to pause in seconds")
                .with_doc("Delays execution of the handler for the given seconds.")
                .with_example("wait(1)")
        );

        // 51. clone(target)
        registry.register(
            BlockDefinition::new("clone", BlockCategory::Control, "Spawn a duplicate clone of target sprite", BlockType::Void, "ControlSystem::clone")
                .with_param("target", BlockType::Object, false, None, "Target entity to clone")
                .with_doc("Instantiates a clone of the sprite with identical components.")
                .with_example("clone(Coin)")
        );

        // 52. delete_clone(target)
        registry.register(
            BlockDefinition::new("delete_clone", BlockCategory::Control, "Delete current cloned entity instance", BlockType::Void, "ControlSystem::deleteClone")
                .with_param("target", BlockType::Object, false, None, "Clone entity to destroy")
                .with_doc("Removes the cloned entity from the active scene.")
                .with_example("delete_clone(Player)")
        );

        // 53. stop_all()
        registry.register(
            BlockDefinition::new("stop_all", BlockCategory::Control, "Stop all active game scripts and execution", BlockType::Void, "ControlSystem::stopAll")
                .with_doc("Immediately terminates all running game events and halts the game loop.")
                .with_example("stop_all()")
        );

        // === MATH & OPERATOR BLOCKS (from Scratch PDF p. 9) ===

        // 54. random(min, max) -> Number
        registry.register(
            BlockDefinition::new("random", BlockCategory::Operators, "Pick random number between min and max", BlockType::Number, "MathSystem::random")
                .with_param("min", BlockType::Number, false, Some("1"), "Minimum value")
                .with_param("max", BlockType::Number, false, Some("10"), "Maximum value")
                .with_doc("Returns a uniformly distributed random number between min and max inclusive.")
                .with_example("roll = random(1, 6)")
        );

        // 55. math.round(n) -> Number
        registry.register(
            BlockDefinition::new("math.round", BlockCategory::Operators, "Round number to nearest whole integer", BlockType::Number, "MathSystem::round")
                .with_param("n", BlockType::Number, false, None, "Input decimal number")
                .with_doc("Rounds a floating-point number to the nearest integer.")
                .with_example("val = math.round(4.6)")
        );

        // 56. math.abs(n) -> Number
        registry.register(
            BlockDefinition::new("math.abs", BlockCategory::Operators, "Absolute value of a number", BlockType::Number, "MathSystem::abs")
                .with_param("n", BlockType::Number, false, None, "Input number")
                .with_doc("Returns the positive magnitude of a number.")
                .with_example("dist = math.abs(x1 - x2)")
        );

        // 57. math.sqrt(n) -> Number
        registry.register(
            BlockDefinition::new("math.sqrt", BlockCategory::Operators, "Square root of a number", BlockType::Number, "MathSystem::sqrt")
                .with_param("n", BlockType::Number, false, None, "Input non-negative number")
                .with_doc("Calculates the principal square root.")
                .with_example("hyp = math.sqrt(a * a + b * b)")
        );

        // 58. math.sin(n) -> Number
        registry.register(
            BlockDefinition::new("math.sin", BlockCategory::Operators, "Sine of angle in degrees", BlockType::Number, "MathSystem::sin")
                .with_param("degrees", BlockType::Number, false, None, "Angle in degrees")
                .with_doc("Computes the trigonometric sine.")
                .with_example("y_offset = math.sin(angle) * 10")
        );

        // 59. math.cos(n) -> Number
        registry.register(
            BlockDefinition::new("math.cos", BlockCategory::Operators, "Cosine of angle in degrees", BlockType::Number, "MathSystem::cos")
                .with_param("degrees", BlockType::Number, false, None, "Angle in degrees")
                .with_doc("Computes the trigonometric cosine.")
                .with_example("x_offset = math.cos(angle) * 10")
        );

        // 60. text.join(a, b) -> String
        registry.register(
            BlockDefinition::new("text.join", BlockCategory::Operators, "Concatenate two strings", BlockType::String, "TextSystem::join")
                .with_param("a", BlockType::String, false, None, "First string")
                .with_param("b", BlockType::String, false, None, "Second string")
                .with_doc("Attaches the second string to the end of the first string.")
                .with_example("msg = text.join(\"Score: \", score)")
        );

        // 61. text.length(s) -> Number
        registry.register(
            BlockDefinition::new("text.length", BlockCategory::Operators, "Get character length of a string", BlockType::Number, "TextSystem::length")
                .with_param("s", BlockType::String, false, None, "Input string")
                .with_doc("Returns the number of characters in the text.")
                .with_example("len = text.length(player_name)")
        );

        // === PEN BLOCKS (from Scratch PDF p. 4) ===

        // 62. pen.clear()
        registry.register(
            BlockDefinition::new("pen.clear", BlockCategory::Pen, "Clear all canvas drawing trails", BlockType::Void, "PenSystem::clear")
                .with_doc("Erases all drawn lines and stamps from the stage canvas.")
                .with_example("pen.clear()")
        );

        // 63. pen.down(target)
        registry.register(
            BlockDefinition::new("pen.down", BlockCategory::Pen, "Put pen down to draw movement trails", BlockType::Void, "PenSystem::down")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_doc("Lowers the pen so lines are drawn whenever the sprite moves.")
                .with_example("pen.down(Player)")
        );

        // 64. pen.up(target)
        registry.register(
            BlockDefinition::new("pen.up", BlockCategory::Pen, "Lift pen up to stop drawing trails", BlockType::Void, "PenSystem::up")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_doc("Raises the pen so moving the sprite does not draw lines.")
                .with_example("pen.up(Player)")
        );

        // 65. pen.set_color(color)
        registry.register(
            BlockDefinition::new("pen.set_color", BlockCategory::Pen, "Set pen line stroke color", BlockType::Void, "PenSystem::setColor")
                .with_param("color", BlockType::String, false, None, "Color name or hex string")
                .with_doc("Sets the color of lines drawn by the pen.")
                .with_example("pen.set_color(\"red\")")
        );

        // 66. pen.set_size(size)
        registry.register(
            BlockDefinition::new("pen.set_size", BlockCategory::Pen, "Set pen line thickness in pixels", BlockType::Void, "PenSystem::setSize")
                .with_param("size", BlockType::Number, false, Some("1"), "Line thickness in pixels")
                .with_doc("Sets the line thickness of pen drawings.")
                .with_example("pen.set_size(3)")
        );

        // 67. pen.stamp(target)
        registry.register(
            BlockDefinition::new("pen.stamp", BlockCategory::Pen, "Stamp sprite image onto stage", BlockType::Void, "PenSystem::stamp")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_doc("Draws an imprint of the sprite's current costume onto the stage canvas.")
                .with_example("pen.stamp(Player)")
        );

        // === ADDITIONAL OPERATOR & MATH BLOCKS ===

        registry.register(
            BlockDefinition::new("math.floor", BlockCategory::Operators, "Round number down to nearest integer", BlockType::Number, "MathSystem::floor")
                .with_param("n", BlockType::Number, false, None, "Input decimal number")
                .with_doc("Returns the largest integer less than or equal to n.")
                .with_example("val = math.floor(4.9)")
        );

        registry.register(
            BlockDefinition::new("math.ceil", BlockCategory::Operators, "Round number up to nearest integer", BlockType::Number, "MathSystem::ceil")
                .with_param("n", BlockType::Number, false, None, "Input decimal number")
                .with_doc("Returns the smallest integer greater than or equal to n.")
                .with_example("val = math.ceil(4.1)")
        );

        registry.register(
            BlockDefinition::new("math.tan", BlockCategory::Operators, "Tangent of angle in degrees", BlockType::Number, "MathSystem::tan")
                .with_param("degrees", BlockType::Number, false, None, "Angle in degrees")
                .with_doc("Computes the trigonometric tangent.")
                .with_example("slope = math.tan(angle)")
        );

        registry.register(
            BlockDefinition::new("math.asin", BlockCategory::Operators, "Arcsine in degrees", BlockType::Number, "MathSystem::asin")
                .with_param("n", BlockType::Number, false, None, "Value between -1.0 and 1.0")
                .with_doc("Calculates inverse sine in degrees.")
                .with_example("angle = math.asin(0.5)")
        );

        registry.register(
            BlockDefinition::new("math.acos", BlockCategory::Operators, "Arccosine in degrees", BlockType::Number, "MathSystem::acos")
                .with_param("n", BlockType::Number, false, None, "Value between -1.0 and 1.0")
                .with_doc("Calculates inverse cosine in degrees.")
                .with_example("angle = math.acos(0.5)")
        );

        registry.register(
            BlockDefinition::new("math.atan", BlockCategory::Operators, "Arctangent in degrees", BlockType::Number, "MathSystem::atan")
                .with_param("n", BlockType::Number, false, None, "Input number")
                .with_doc("Calculates inverse tangent in degrees.")
                .with_example("angle = math.atan(ratio)")
        );

        registry.register(
            BlockDefinition::new("math.ln", BlockCategory::Operators, "Natural logarithm (base e)", BlockType::Number, "MathSystem::ln")
                .with_param("n", BlockType::Number, false, None, "Input positive number")
                .with_doc("Calculates natural logarithm.")
                .with_example("log_val = math.ln(10)")
        );

        registry.register(
            BlockDefinition::new("math.log", BlockCategory::Operators, "Common logarithm (base 10)", BlockType::Number, "MathSystem::log")
                .with_param("n", BlockType::Number, false, None, "Input positive number")
                .with_doc("Calculates base-10 logarithm.")
                .with_example("orders = math.log(100)")
        );

        registry.register(
            BlockDefinition::new("math.exp", BlockCategory::Operators, "Exponential e raised to power n", BlockType::Number, "MathSystem::exp")
                .with_param("n", BlockType::Number, false, None, "Power exponent")
                .with_doc("Calculates e^n.")
                .with_example("val = math.exp(2)")
        );

        registry.register(
            BlockDefinition::new("math.pow", BlockCategory::Operators, "Raise base to exponent power", BlockType::Number, "MathSystem::pow")
                .with_param("base", BlockType::Number, false, None, "Base number")
                .with_param("exponent", BlockType::Number, false, None, "Exponent power")
                .with_doc("Calculates base^exponent.")
                .with_example("area = math.pow(radius, 2) * 3.14159")
        );

        registry.register(
            BlockDefinition::new("math.mod", BlockCategory::Operators, "Euclidean modulo remainder", BlockType::Number, "MathSystem::mod")
                .with_param("a", BlockType::Number, false, None, "Dividend")
                .with_param("b", BlockType::Number, false, None, "Divisor")
                .with_doc("Calculates positive mathematical modulo remainder.")
                .with_example("rem = math.mod(10, 3)")
        );

        registry.register(
            BlockDefinition::new("math.min", BlockCategory::Operators, "Minimum of two numbers", BlockType::Number, "MathSystem::min")
                .with_param("a", BlockType::Number, false, None, "First number")
                .with_param("b", BlockType::Number, false, None, "Second number")
                .with_doc("Returns the smaller of two numbers.")
                .with_example("lowest = math.min(health, max_health)")
        );

        registry.register(
            BlockDefinition::new("math.max", BlockCategory::Operators, "Maximum of two numbers", BlockType::Number, "MathSystem::max")
                .with_param("a", BlockType::Number, false, None, "First number")
                .with_param("b", BlockType::Number, false, None, "Second number")
                .with_doc("Returns the larger of two numbers.")
                .with_example("clamped = math.max(0, health)")
        );

        // === ADDITIONAL STRING REPORTERS ===

        registry.register(
            BlockDefinition::new("text.contains", BlockCategory::Operators, "Check if string contains substring", BlockType::Boolean, "TextSystem::contains")
                .with_param("s", BlockType::String, false, None, "Source string")
                .with_param("substring", BlockType::String, false, None, "Substring to search for")
                .with_doc("Returns true if s contains substring.")
                .with_example("if text.contains(word, \"cat\"):")
        );

        registry.register(
            BlockDefinition::new("text.letter_at", BlockCategory::Operators, "Get 1-indexed character of string", BlockType::String, "TextSystem::letterAt")
                .with_param("s", BlockType::String, false, None, "Input string")
                .with_param("index", BlockType::Number, false, Some("1"), "1-based character position")
                .with_doc("Returns the single character at the 1-based index.")
                .with_example("first = text.letter_at(\"world\", 1)")
        );

        registry.register(
            BlockDefinition::new("text.upper", BlockCategory::Operators, "Convert text to uppercase", BlockType::String, "TextSystem::upper")
                .with_param("s", BlockType::String, false, None, "Input string")
                .with_doc("Returns text converted entirely to uppercase.")
                .with_example("loud = text.upper(\"hello\")")
        );

        registry.register(
            BlockDefinition::new("text.lower", BlockCategory::Operators, "Convert text to lowercase", BlockType::String, "TextSystem::lower")
                .with_param("s", BlockType::String, false, None, "Input string")
                .with_doc("Returns text converted entirely to lowercase.")
                .with_example("quiet = text.lower(\"HELLO\")")
        );

        // === MOTION & LOOKS REPORTERS ===

        registry.register(
            BlockDefinition::new("get_direction", BlockCategory::Movement, "Get facing rotation of sprite in degrees", BlockType::Number, "MovementSystem::getDirection")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_doc("Returns the current facing angle in degrees.")
                .with_example("angle = get_direction(Player)")
        );

        registry.register(
            BlockDefinition::new("get_size", BlockCategory::Looks, "Get sprite scale percentage", BlockType::Number, "LooksSystem::getSize")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_doc("Returns the current scale percentage of the sprite.")
                .with_example("scale = get_size(Player)")
        );

        registry.register(
            BlockDefinition::new("get_costume_number", BlockCategory::Looks, "Get active costume number", BlockType::Number, "LooksSystem::getCostumeNumber")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_doc("Returns the index of the sprite's active costume frame.")
                .with_example("c_num = get_costume_number(Player)")
        );

        registry.register(
            BlockDefinition::new("get_backdrop_name", BlockCategory::Scene, "Get active scene backdrop name", BlockType::String, "SceneSystem::getBackdropName")
                .with_doc("Returns the name of current background/backdrop.")
                .with_example("bg = get_backdrop_name()")
        );

        registry.register(
            BlockDefinition::new("think_for", BlockCategory::Looks, "Show thought bubble for duration", BlockType::Void, "LooksSystem::thinkFor")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("text", BlockType::String, false, None, "Thought text")
                .with_param("seconds", BlockType::Number, false, Some("2"), "Duration in seconds")
                .with_doc("Displays a thought bubble for the specified time.")
                .with_example("think_for(Player, \"Hmm...\", 2)")
        );

        registry.register(
            BlockDefinition::new("change_effect", BlockCategory::Looks, "Change visual shader effect by delta", BlockType::Void, "LooksSystem::changeEffect")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("effect", BlockType::String, false, Some("\"color\""), "Effect name")
                .with_param("delta", BlockType::Number, false, Some("25"), "Delta amount")
                .with_doc("Alters a visual effect by delta value.")
                .with_example("change_effect(Player, \"ghost\", 25)")
        );

        // === SOUND & MUSIC REPORTERS & EXTENSIONS ===

        registry.register(
            BlockDefinition::new("sound.get_volume", BlockCategory::Audio, "Get current master audio volume", BlockType::Number, "AudioSystem::getVolume")
                .with_doc("Returns master sound volume percentage (0 - 100).")
                .with_example("vol = sound.get_volume()")
        );

        registry.register(
            BlockDefinition::new("music.get_tempo", BlockCategory::Audio, "Get current music tempo BPM", BlockType::Number, "AudioSystem::getTempo")
                .with_doc("Returns current musical tempo in beats per minute.")
                .with_example("bpm = music.get_tempo()")
        );

        registry.register(
            BlockDefinition::new("music.change_tempo", BlockCategory::Audio, "Change music tempo by delta BPM", BlockType::Void, "AudioSystem::changeTempo")
                .with_param("delta", BlockType::Number, false, Some("20"), "Delta BPM")
                .with_doc("Adjusts music tempo BPM by delta.")
                .with_example("music.change_tempo(10)")
        );

        registry.register(
            BlockDefinition::new("music.set_instrument", BlockCategory::Audio, "Set MIDI instrument sound preset", BlockType::Void, "AudioSystem::setInstrument")
                .with_param("instrument", BlockType::Number, false, Some("1"), "Instrument ID number")
                .with_doc("Selects instrument voice preset (1 - 21).")
                .with_example("music.set_instrument(2)")
        );

        registry.register(
            BlockDefinition::new("music.play_drum", BlockCategory::Audio, "Play drum instrument hit for beats", BlockType::Void, "AudioSystem::playDrum")
                .with_param("drum", BlockType::Number, false, Some("1"), "Drum ID (1 - 18)")
                .with_param("beats", BlockType::Number, false, Some("0.25"), "Duration in beats")
                .with_doc("Plays percussive drum hit for duration.")
                .with_example("music.play_drum(1, 0.25)")
        );

        registry.register(
            BlockDefinition::new("music.rest", BlockCategory::Audio, "Rest/pause music for specified beats", BlockType::Void, "AudioSystem::rest")
                .with_param("beats", BlockType::Number, false, Some("0.25"), "Rest duration in beats")
                .with_doc("Pauses music playback for number of beats.")
                .with_example("music.rest(0.5)")
        );

        // === PEN EXTENSIONS ===

        registry.register(
            BlockDefinition::new("pen.change_size", BlockCategory::Pen, "Change pen line thickness by delta", BlockType::Void, "PenSystem::changeSize")
                .with_param("delta", BlockType::Number, false, Some("1"), "Delta thickness")
                .with_doc("Adjusts the pen line thickness.")
                .with_example("pen.change_size(2)")
        );

        registry.register(
            BlockDefinition::new("pen.change_color", BlockCategory::Pen, "Change pen color hue by delta", BlockType::Void, "PenSystem::changeColor")
                .with_param("delta", BlockType::Number, false, Some("10"), "Hue delta value")
                .with_doc("Increments pen color hue value.")
                .with_example("pen.change_color(10)")
        );

        registry.register(
            BlockDefinition::new("pen.set_shade", BlockCategory::Pen, "Set pen darkness/shade percentage", BlockType::Void, "PenSystem::setShade")
                .with_param("shade", BlockType::Number, false, Some("50"), "Shade percentage (0 - 100)")
                .with_doc("Sets pen darkness/brightness shade.")
                .with_example("pen.set_shade(70)")
        );

        registry.register(
            BlockDefinition::new("pen.change_shade", BlockCategory::Pen, "Change pen darkness/shade by delta", BlockType::Void, "PenSystem::changeShade")
                .with_param("delta", BlockType::Number, false, Some("10"), "Delta shade")
                .with_doc("Adjusts pen darkness/brightness shade.")
                .with_example("pen.change_shade(-10)")
        );

        // === SENSING & SYSTEM REPORTERS ===

        registry.register(
            BlockDefinition::new("current_time", BlockCategory::Sensing, "Get current system time unit value", BlockType::Number, "SensingSystem::currentTime")
                .with_param("unit", BlockType::String, false, Some("\"second\""), "Time unit (year, month, date, day_of_week, hour, minute, second)")
                .with_doc("Queries current system calendar date or clock time.")
                .with_example("hr = current_time(\"hour\")")
        );

        registry.register(
            BlockDefinition::new("days_since_2000", BlockCategory::Sensing, "Days elapsed since January 1, 2000", BlockType::Number, "SensingSystem::daysSince2000")
                .with_doc("Returns exact decimal days since Jan 1, 2000 00:00:00 UTC.")
                .with_example("days = days_since_2000()")
        );

        registry.register(
            BlockDefinition::new("get_username", BlockCategory::Sensing, "Get current player username", BlockType::String, "SensingSystem::getUsername")
                .with_doc("Returns the username of the active player or environment.")
                .with_example("name = get_username()")
        );

        // === BROADCAST & MESSAGING (from Scratch PDF p. 8) ===

        registry.register(
            BlockDefinition::new("broadcast", BlockCategory::Events, "Broadcast a message event to all scripts", BlockType::Void, "EventSystem::broadcast")
                .with_param("message", BlockType::String, false, None, "Message name string")
                .with_doc("Sends a global message that triggers any 'when message(\"...\"):' event handlers.")
                .with_example("broadcast(\"game_over\")")
                .with_example("broadcast(\"next_wave\")")
        );

        registry.register(
            BlockDefinition::new("broadcast_and_wait", BlockCategory::Events, "Broadcast a message and wait for handlers", BlockType::Void, "EventSystem::broadcastAndWait")
                .with_param("message", BlockType::String, false, None, "Message name string")
                .with_doc("Sends a global message and waits for all matching handlers to complete.")
                .with_example("broadcast_and_wait(\"level_cleared\")")
        );

        // === DYNAMIC LISTS SUBSYSTEM (from Scratch PDF p. 6) ===

        registry.register(
            BlockDefinition::new("list.add", BlockCategory::Variables, "Append item to end of list", BlockType::Void, "ListSystem::add")
                .with_param("list", BlockType::String, false, None, "Name of the list variable")
                .with_param("item", BlockType::Any, false, None, "Item to add to list")
                .with_doc("Appends an item to the end of the named list.")
                .with_example("list.add(\"inventory\", \"sword\")")
                .with_example("list.add(\"scores\", 100)")
        );

        registry.register(
            BlockDefinition::new("list.delete", BlockCategory::Variables, "Delete item from list by 1-based index, 'last', or 'all'", BlockType::Void, "ListSystem::delete")
                .with_param("list", BlockType::String, false, None, "Name of the list variable")
                .with_param("index", BlockType::Any, false, None, "1-based index number, 'last', or 'all'")
                .with_doc("Deletes an item from the list at the specified position, or clears the list if 'all'.")
                .with_example("list.delete(\"inventory\", 1)")
                .with_example("list.delete(\"inventory\", \"last\")")
                .with_example("list.delete(\"inventory\", \"all\")")
        );

        registry.register(
            BlockDefinition::new("list.insert", BlockCategory::Variables, "Insert item at 1-based index in list", BlockType::Void, "ListSystem::insert")
                .with_param("list", BlockType::String, false, None, "Name of the list variable")
                .with_param("index", BlockType::Any, false, None, "1-based index or 'last'")
                .with_param("item", BlockType::Any, false, None, "Item to insert")
                .with_doc("Inserts an item into the list at the specified position, shifting subsequent items.")
                .with_example("list.insert(\"inventory\", 1, \"shield\")")
        );

        registry.register(
            BlockDefinition::new("list.replace", BlockCategory::Variables, "Replace item at 1-based index with new value", BlockType::Void, "ListSystem::replace")
                .with_param("list", BlockType::String, false, None, "Name of the list variable")
                .with_param("index", BlockType::Any, false, None, "1-based index or 'last'")
                .with_param("item", BlockType::Any, false, None, "Replacement item value")
                .with_doc("Overwrites the item at the specified position in the list.")
                .with_example("list.replace(\"inventory\", 1, \"potion\")")
        );

        registry.register(
            BlockDefinition::new("list.item", BlockCategory::Variables, "Get item at 1-based index, 'last', or 'random'", BlockType::Any, "ListSystem::item")
                .with_param("list", BlockType::String, false, None, "Name of the list variable")
                .with_param("index", BlockType::Any, false, None, "1-based index, 'last', or 'random'")
                .with_doc("Returns the value stored at the given position in the list.")
                .with_example("top_item = list.item(\"inventory\", 1)")
                .with_example("random_item = list.item(\"inventory\", \"random\")")
        );

        registry.register(
            BlockDefinition::new("list.length", BlockCategory::Variables, "Get total number of items in list", BlockType::Number, "ListSystem::length")
                .with_param("list", BlockType::String, false, None, "Name of the list variable")
                .with_doc("Returns the number of elements contained in the list.")
                .with_example("count = list.length(\"inventory\")")
        );

        registry.register(
            BlockDefinition::new("list.contains", BlockCategory::Variables, "Check if list contains an item", BlockType::Boolean, "ListSystem::contains")
                .with_param("list", BlockType::String, false, None, "Name of the list variable")
                .with_param("item", BlockType::Any, false, None, "Item to search for")
                .with_doc("Returns true if the list contains the specified item, false otherwise.")
                .with_example("if list.contains(\"inventory\", \"key\"):")
        );

        registry.register(
            BlockDefinition::new("list.clear", BlockCategory::Variables, "Remove all items from list", BlockType::Void, "ListSystem::clear")
                .with_param("list", BlockType::String, false, None, "Name of the list variable")
                .with_doc("Removes all entries from the named list.")
                .with_example("list.clear(\"inventory\")")
        );

        registry.register(
            BlockDefinition::new("list.show", BlockCategory::Variables, "Display list monitor on stage overlay", BlockType::Void, "ListSystem::show")
                .with_param("list", BlockType::String, false, None, "Name of the list variable")
                .with_doc("Makes the on-screen list monitor visible.")
                .with_example("list.show(\"inventory\")")
        );

        registry.register(
            BlockDefinition::new("list.hide", BlockCategory::Variables, "Hide list monitor from stage overlay", BlockType::Void, "ListSystem::hide")
                .with_param("list", BlockType::String, false, None, "Name of the list variable")
                .with_doc("Hides the on-screen list monitor.")
                .with_example("list.hide(\"inventory\")")
        );

        // Standard Scratch aliases for lists
        registry.register(
            BlockDefinition::new("add_to_list", BlockCategory::Variables, "Append item to list", BlockType::Void, "ListSystem::add")
                .with_param("list", BlockType::String, false, None, "List name")
                .with_param("item", BlockType::Any, false, None, "Item to append")
                .with_doc("Alias for list.add.")
                .with_example("add_to_list(\"inventory\", \"sword\")")
        );
        registry.register(
            BlockDefinition::new("delete_of_list", BlockCategory::Variables, "Delete item from list", BlockType::Void, "ListSystem::delete")
                .with_param("list", BlockType::String, false, None, "List name")
                .with_param("index", BlockType::Any, false, None, "Index")
                .with_doc("Alias for list.delete.")
                .with_example("delete_of_list(\"inventory\", 1)")
        );
        registry.register(
            BlockDefinition::new("insert_at_list", BlockCategory::Variables, "Insert item at position in list", BlockType::Void, "ListSystem::insert")
                .with_param("list", BlockType::String, false, None, "List name")
                .with_param("index", BlockType::Any, false, None, "Index")
                .with_param("item", BlockType::Any, false, None, "Item")
                .with_doc("Alias for list.insert.")
                .with_example("insert_at_list(\"inventory\", 1, \"potion\")")
        );
        registry.register(
            BlockDefinition::new("replace_item_of_list", BlockCategory::Variables, "Replace item in list", BlockType::Void, "ListSystem::replace")
                .with_param("list", BlockType::String, false, None, "List name")
                .with_param("index", BlockType::Any, false, None, "Index")
                .with_param("item", BlockType::Any, false, None, "Replacement item")
                .with_doc("Alias for list.replace.")
                .with_example("replace_item_of_list(\"inventory\", 1, \"potion\")")
        );
        registry.register(
            BlockDefinition::new("item_of_list", BlockCategory::Variables, "Get item of list", BlockType::Any, "ListSystem::item")
                .with_param("list", BlockType::String, false, None, "List name")
                .with_param("index", BlockType::Any, false, None, "Index")
                .with_doc("Alias for list.item.")
                .with_example("it = item_of_list(\"inventory\", 1)")
        );
        registry.register(
            BlockDefinition::new("length_of_list", BlockCategory::Variables, "Get length of list", BlockType::Number, "ListSystem::length")
                .with_param("list", BlockType::String, false, None, "List name")
                .with_doc("Alias for list.length.")
                .with_example("len = length_of_list(\"inventory\")")
        );
        registry.register(
            BlockDefinition::new("list_contains", BlockCategory::Variables, "Check if list contains item", BlockType::Boolean, "ListSystem::contains")
                .with_param("list", BlockType::String, false, None, "List name")
                .with_param("item", BlockType::Any, false, None, "Item to search for")
                .with_doc("Alias for list.contains.")
                .with_example("if list_contains(\"inventory\", \"key\"):")
        );

        // === INTERACTIVE DIALOGUES & INPUT OVERLAYS ===

        registry.register(
            BlockDefinition::new("ask", BlockCategory::Sensing, "Ask a question and display prompt for user input", BlockType::Void, "DialogueSystem::ask")
                .with_param("question", BlockType::String, false, None, "Question prompt string")
                .with_doc("Displays an interactive text prompt asking the player a question and waits for their response.")
                .with_example("ask(\"What is your name?\")")
        );

        registry.register(
            BlockDefinition::new("get_answer", BlockCategory::Sensing, "Get user text response to last ask prompt", BlockType::String, "DialogueSystem::getAnswer")
                .with_doc("Returns the string response entered by the user to the most recent ask prompt.")
                .with_example("player_name = get_answer()")
        );

        registry.register(
            BlockDefinition::new("answer", BlockCategory::Sensing, "Get user text response to last ask prompt", BlockType::String, "DialogueSystem::getAnswer")
                .with_doc("Alias for get_answer.")
                .with_example("name = answer()")
        );

        registry.register(
            BlockDefinition::new("variable.show", BlockCategory::Variables, "Display variable monitor on stage overlay", BlockType::Void, "VariableSystem::show")
                .with_param("name", BlockType::String, false, None, "Variable name")
                .with_doc("Makes the on-screen variable display monitor visible.")
                .with_example("variable.show(\"score\")")
        );

        registry.register(
            BlockDefinition::new("variable.hide", BlockCategory::Variables, "Hide variable monitor from stage overlay", BlockType::Void, "VariableSystem::hide")
                .with_param("name", BlockType::String, false, None, "Variable name")
                .with_doc("Hides the on-screen variable display monitor.")
                .with_example("variable.hide(\"score\")")
        );

        registry.register(
            BlockDefinition::new("show_variable", BlockCategory::Variables, "Show variable monitor", BlockType::Void, "VariableSystem::show")
                .with_param("name", BlockType::String, false, None, "Variable name")
                .with_doc("Alias for variable.show.")
                .with_example("show_variable(\"score\")")
        );

        registry.register(
            BlockDefinition::new("hide_variable", BlockCategory::Variables, "Hide variable monitor", BlockType::Void, "VariableSystem::hide")
                .with_param("name", BlockType::String, false, None, "Variable name")
                .with_doc("Alias for variable.hide.")
                .with_example("hide_variable(\"score\")")
        );

        // === PIXEL, COLOR SENSING & LAYERING ===

        registry.register(
            BlockDefinition::new("touching_color", BlockCategory::Sensing, "Check if sprite overlaps an object with specified color", BlockType::Boolean, "SensingSystem::touchingColor")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("color", BlockType::String, false, None, "Color name or hex string")
                .with_doc("Returns true if target's collider touches any entity or boundary with the given color.")
                .with_example("if touching_color(Player, \"red\"):")
        );

        registry.register(
            BlockDefinition::new("color_touching_color", BlockCategory::Sensing, "Check if objects of first color touch objects of second color", BlockType::Boolean, "SensingSystem::colorTouchingColor")
                .with_param("color1", BlockType::String, false, None, "First color")
                .with_param("color2", BlockType::String, false, None, "Second color")
                .with_doc("Returns true if any entity with color1 touches any entity with color2.")
                .with_example("if color_touching_color(\"green\", \"red\"):")
        );

        registry.register(
            BlockDefinition::new("get_loudness", BlockCategory::Sensing, "Get microphone input volume amplitude", BlockType::Number, "SensingSystem::getLoudness")
                .with_doc("Returns microphone loudness volume (0 to 100).")
                .with_example("vol = get_loudness()")
        );

        registry.register(
            BlockDefinition::new("loudness", BlockCategory::Sensing, "Get microphone input volume amplitude", BlockType::Number, "SensingSystem::getLoudness")
                .with_doc("Alias for get_loudness.")
                .with_example("vol = loudness()")
        );

        registry.register(
            BlockDefinition::new("go_to_front", BlockCategory::Looks, "Bring sprite to top rendering layer", BlockType::Void, "LooksSystem::goToFront")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_doc("Places the sprite in front of all other sprites on stage.")
                .with_example("go_to_front(Player)")
        );

        registry.register(
            BlockDefinition::new("go_back_layers", BlockCategory::Looks, "Move sprite back by number of layers", BlockType::Void, "LooksSystem::goBackLayers")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("count", BlockType::Number, false, Some("1"), "Number of layers to move back")
                .with_doc("Pushes sprite backwards in visual render order.")
                .with_example("go_back_layers(Player, 2)")
        );

        registry.register(
            BlockDefinition::new("property_of", BlockCategory::Sensing, "Introspect attribute or variable of a sprite", BlockType::Any, "SensingSystem::propertyOf")
                .with_param("target", BlockType::Object, false, None, "Target entity")
                .with_param("property", BlockType::String, false, None, "Property name ('x position', 'y position', 'direction', 'costume #', 'size', 'volume')")
                .with_doc("Reads a specified attribute or variable of another sprite.")
                .with_example("enemy_x = property_of(Enemy, \"x position\")")
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
