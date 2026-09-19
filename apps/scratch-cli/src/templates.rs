pub fn get_template_code(template_name: &str) -> &'static str {
    match template_name.to_lowercase().as_str() {
        "platformer" => r#"when start:
    score = 0
    variable.show("score")
    background.set("night")

when action.down("right"):
    move(Player, 5)

when action.down("left"):
    move(Player, -5)

when action.down("up"):
    move_up(Player, 5)

when action.down("down"):
    move_down(Player, 5)

when action.press("jump"):
    jump(Player, 12)
    sound.play("jump")

when Player touches Coin:
    score += 10
    teleport(Coin, random(50, 430), random(50, 310))
    sound.play("coin")
"#,
        "coins" => r#"when start:
    score = 0
    variable.show("score")
    background.set("blue")

when action.down("right"):
    move(Player, 6)

when action.down("left"):
    move(Player, -6)

when action.down("up"):
    move_up(Player, 6)

when action.down("down"):
    move_down(Player, 6)

when Player touches Coin:
    score += 1
    teleport(Coin, random(60, 420), random(60, 300))
    sound.play("coin")
"#,
        "dialogue" => r#"when start:
    ask("What is your adventure name?")
    variable.show("score")

when message("answered"):
    say_for(Player, text.join("Welcome, ", get_answer()), 2)
    score = 100
"#,
        "motion" => r#"when start:
    set_rotation_style(Player, "left-right")
    glide(Player, 2, 380, 280)

every 3 seconds:
    go_to(Player, "random")
"#,
        "lists" => r#"when start:
    list.add("inventory", "iron_sword")
    list.add("inventory", "wooden_shield")
    list.add("inventory", "health_potion")
    list.show("inventory")
"#,
        _ => r#"when start:
    score = 0
    background.set("night")
    variable.show("score")

when action.down("right"):
    move(Player, 5)

when action.down("left"):
    move(Player, -5)

when action.down("up"):
    move_up(Player, 5)

when action.down("down"):
    move_down(Player, 5)

when action.press("jump"):
    jump(Player, 12)
"#,
    }
}
