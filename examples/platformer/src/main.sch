when start:
    score = 0
    health = 3
    background.set("forest")

when action.down("right"):
    move(Player, 6)

when action.down("left"):
    move(Player, -6)

when action.press("jump"):
    jump(Player, 14)

when Player touches Coin:
    collect(Coin)
    score += 1
    sound.play("coin")

when Player touches Hazard:
    damage(Player, 1)
    health -= 1
    if health <= 0:
        respawn(Player)
        health = 3

when Player touches Enemy:
    damage(Player, 1)
    health -= 1
    if health <= 0:
        respawn(Player)
        health = 3

when Player touches Goal:
    sound.play("jump")
    scene.switch("level2")

every 5 seconds:
    score += 1