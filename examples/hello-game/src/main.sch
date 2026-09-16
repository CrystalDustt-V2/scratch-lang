when start:
    score = 0
    background.set("white")

when action.down("right"):
    move(Player, 5)

when action.down("left"):
    move(Player, -5)

when action.press("jump"):
    jump(Player, 12)
