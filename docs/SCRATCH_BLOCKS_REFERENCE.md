# Scratch Blocks Reference & scratch-lang Mapping

This document provides a comprehensive mapping of every single Scratch block from the official Harvard Scratch reference (*"All Blocks of Scratch"*, Creative Coding Using Scratch V1) into **scratch-lang** (`.sch`) text-based equivalents.

---

## Table of Contents
1. [Motion Blocks](#1-motion-blocks)
2. [Looks Blocks](#2-looks-blocks)
3. [Sound & Music Blocks](#3-sound--music-blocks)
4. [Pen Blocks](#4-pen-blocks)
5. [Data & Variables](#5-data--variables)
6. [Events & Broadcasts](#6-events--broadcasts)
7. [Control & Clones](#7-control--clones)
8. [Sensing Blocks](#8-sensing-blocks)
9. [Operators & Math](#9-operators--math)

---

## 1. Motion Blocks

| Original Scratch Block | scratch-lang Function / Syntax | Return Type | Category | Description |
| :--- | :--- | :--- | :--- | :--- |
| `move (10) steps` | `move(target, x, [y])` | `Void` | Motion | Moves object by delta pixels. |
| `turn right (15) degrees` | `turn_right(target, degrees)` | `Void` | Motion | Rotates target clockwise by degrees. |
| `turn left (15) degrees` | `turn_left(target, degrees)` | `Void` | Motion | Rotates target counter-clockwise by degrees. |
| `point in direction (90)` | `point_in_direction(target, degrees)` | `Void` | Motion | Sets absolute facing angle (0: left, 90: up, 180: right, 270: down). |
| `point towards [mouse-pointer / sprite]` | `point_towards(target, other)` | `Void` | Motion | Rotates target to face towards mouse or another entity. |
| `go to x: (0) y: (0)` | `teleport(target, x, y)` | `Void` | Motion | Sets absolute (x, y) coordinates instantly. |
| `go to [mouse-pointer / random]` | `go_to(target, destination)` | `Void` | Motion | Moves to mouse pointer or random position. |
| `glide (1) secs to x: (0) y: (0)` | `glide(target, seconds, x, y)` | `Void` | Motion | Smoothly interpolates to (x, y) over duration. |
| `change x by (10)` | `change_x(target, dx)` | `Void` | Motion | Increases or decreases x coordinate by delta. |
| `set x to (0)` | `set_x(target, x)` | `Void` | Motion | Sets x coordinate directly. |
| `change y by (10)` | `change_y(target, dy)` | `Void` | Motion | Increases or decreases y coordinate by delta. |
| `set y to (0)` | `set_y(target, y)` | `Void` | Motion | Sets y coordinate directly. |
| `if on edge, bounce` | `bounce_on_edge(target)` | `Void` | Motion | Inverts velocity if colliding with screen boundaries. |
| `set rotation style [left-right / all around / don't]` | `set_rotation_style(target, style)` | `Void` | Motion | Controls rotation rendering constraint. |
| `x position` | `x_position(target)` | `Number` | Motion | Returns current x position of target. |
| `y position` | `y_position(target)` | `Number` | Motion | Returns current y position of target. |
| `direction` | `get_direction(target)` | `Number` | Motion | Returns current angle in degrees. |

---

## 2. Looks Blocks

| Original Scratch Block | scratch-lang Function / Syntax | Return Type | Category | Description |
| :--- | :--- | :--- | :--- | :--- |
| `say [Hello!] for (2) secs` | `say_for(target, text, seconds)` | `Void` | Looks | Displays speech bubble above sprite for duration. |
| `say [Hello!]` | `say(target, text)` | `Void` | Looks | Displays speech bubble above sprite indefinitely. |
| `think [Hmm...] for (2) secs` | `think_for(target, text, seconds)` | `Void` | Looks | Displays thought bubble above sprite for duration. |
| `think [Hmm...]` | `think(target, text)` | `Void` | Looks | Displays thought bubble above sprite indefinitely. |
| `show` | `show(target)` | `Void` | Looks | Makes target sprite visible on stage. |
| `hide` | `hide(target)` | `Void` | Looks | Hides target sprite from stage. |
| `switch costume to [costume1]` | `switch_costume(target, name)` | `Void` | Looks | Sets active costume/sprite graphic. |
| `next costume` | `next_costume(target)` | `Void` | Looks | Cycles to next costume in sprite animation list. |
| `switch backdrop to [backdrop1]` | `background.set(name)` | `Void` | Scene | Switches stage background image/color. |
| `next backdrop` | `next_backdrop()` | `Void` | Scene | Cycles to next backdrop in scene list. |
| `change [effect] effect by (25)` | `change_effect(target, effect, delta)` | `Void` | Looks | Alters visual shader/filter effect. |
| `set [effect] effect to (0)` | `set_effect(target, effect, value)` | `Void` | Looks | Sets shader/filter effect (color, ghost, brightness). |
| `clear graphic effects` | `clear_effects(target)` | `Void` | Looks | Clears all active graphical filters on sprite. |
| `change size by (10)` | `change_size(target, delta)` | `Void` | Looks | Modifies scale percentage by delta. |
| `set size to (100) %` | `set_size(target, percent)` | `Void` | Looks | Sets absolute scale percentage. |
| `go to front` | `go_to_front(target)` | `Void` | Looks | Brings sprite to highest render layer. |
| `go back (1) layers` | `go_back_layers(target, count)` | `Void` | Looks | Pushes sprite back by layer count. |
| `costume #` | `get_costume_number(target)` | `Number` | Looks | Returns index of active costume. |
| `size` | `get_size(target)` | `Number` | Looks | Returns current scale percentage. |

---

## 3. Sound & Music Blocks

| Original Scratch Block | scratch-lang Function / Syntax | Return Type | Category | Description |
| :--- | :--- | :--- | :--- | :--- |
| `play sound [meow]` | `sound.play(name)` | `Void` | Audio | Plays sound effect asynchronously. |
| `play sound [meow] until done` | `sound.play_until_done(name)` | `Void` | Audio | Plays sound effect and waits until finished. |
| `stop all sounds` | `sound.stop_all()` | `Void` | Audio | Halts all active audio channels. |
| `play drum (1) for (0.25) beats` | `music.play_drum(drum, beats)` | `Void` | Audio | Triggers drum hit sound for specified duration. |
| `rest for (0.25) beats` | `music.rest(beats)` | `Void` | Audio | Pauses audio output for beat interval. |
| `play note (60) for (0.5) beats` | `music.play_note(note, beats)` | `Void` | Audio | Synthesizes MIDI musical note. |
| `set instrument to (1)` | `music.set_instrument(id)` | `Void` | Audio | Sets MIDI instrument sound preset. |
| `change volume by (-10)` | `sound.change_volume(delta)` | `Void` | Audio | Adjusts master or channel volume by delta. |
| `set volume to (100) %` | `sound.set_volume(percent)` | `Void` | Audio | Sets audio volume (0 - 100). |
| `volume` | `sound.get_volume()` | `Number` | Audio | Returns current volume percentage. |
| `change tempo by (20)` | `music.change_tempo(delta)` | `Void` | Audio | Increases or decreases BPM. |
| `set tempo to (60) bpm` | `music.set_tempo(bpm)` | `Void` | Audio | Sets audio tempo in beats per minute. |
| `tempo` | `music.get_tempo()` | `Number` | Audio | Returns active tempo BPM. |

---

## 4. Pen Blocks

| Original Scratch Block | scratch-lang Function / Syntax | Return Type | Category | Description |
| :--- | :--- | :--- | :--- | :--- |
| `clear` | `pen.clear()` | `Void` | Custom | Clears all canvas vector trails. |
| `stamp` | `pen.stamp(target)` | `Void` | Custom | Stamps sprite texture onto stage background. |
| `pen down` | `pen.down(target)` | `Void` | Custom | Lowers pen to draw trails as sprite moves. |
| `pen up` | `pen.up(target)` | `Void` | Custom | Raises pen to stop drawing trails. |
| `set pen color to [color]` | `pen.set_color(color)` | `Void` | Custom | Sets pen stroke color. |
| `change pen color by (10)` | `pen.change_color(delta)` | `Void` | Custom | Modifies pen hue / color value. |
| `set pen size to (1)` | `pen.set_size(size)` | `Void` | Custom | Sets pen line stroke thickness. |
| `change pen size by (1)` | `pen.change_size(delta)` | `Void` | Custom | Adjusts pen line stroke thickness. |

---

## 5. Data & Variables

| Original Scratch Block | scratch-lang Function / Syntax | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `set [var] to (0)` | `var = 0` | Native | Variable assignment statement. |
| `change [var] by (1)` | `var += 1` | Native | In-place compound addition assignment. |
| `show variable [var]` | `variable.show(name)` | `Void` | Shows variable display overlay on screen. |
| `hide variable [var]` | `variable.hide(name)` | `Void` | Hides variable display overlay on screen. |

---

## 6. Events & Broadcasts

| Original Scratch Block | scratch-lang Function / Syntax | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `when green flag clicked` | `when start:` | Event | Runs when game initializes or scene boots. |
| `when [space] key pressed` | `when action.press("jump"):` | Event | Triggered when key is pressed down. |
| `when this sprite clicked` | `when click(target):` | Event | Triggered when player clicks with mouse on entity. |
| `when backdrop switches to [b1]` | `when scene.switched("b1"):` | Event | Fires when scene transitions. |
| `when loudness > (10)` | `when loudness > 10:` | Event | Audio volume threshold trigger. |
| `broadcast [msg]` | `broadcast(message)` | `Void` | Dispatches global custom event string. |
| `broadcast [msg] and wait` | `broadcast_and_wait(message)` | `Void` | Dispatches event and awaits handler completion. |
| `when I receive [msg]` | `when message("msg"):` | Event | Handler for broadcast messages. |

---

## 7. Control & Clones

| Original Scratch Block | scratch-lang Function / Syntax | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `wait (1) secs` | `wait(seconds)` | `Void` | Non-blocking coroutine wait delay. |
| `repeat (10)` | `repeat 10:` | Block | Loop statement executing body N times. |
| `forever` | `when update:` | Event | Loop running every frame at 60 FPS. |
| `if <condition> then` | `if <condition>:` | Block | Conditional branching statement. |
| `stop [all / this script]` | `stop_all()` / `return` | Statement | Halts game execution or exits function early. |
| `create clone of [myself]` | `clone(target)` | `Void` | Instantiates a dynamic clone of the entity. |
| `when I start as a clone` | `when clone(target):` | Event | Lifecycle handler for cloned instance. |
| `delete this clone` | `delete_clone(target)` | `Void` | Removes cloned entity from game world. |

---

## 8. Sensing Blocks

| Original Scratch Block | scratch-lang Function / Syntax | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `touching [sprite]?` | `touching(target, other)` | `Boolean` | AABB collider overlap check. |
| `touching color [color]?` | `touching_color(target, color)` | `Boolean` | Color-based collision check. |
| `ask [question] and wait` | `ask(question)` | `Void` | Displays user prompt input dialogue. |
| `answer` | `get_answer()` | `String` | Returns text input entered by user. |
| `key [space] pressed?` | `key_pressed(key)` | `Boolean` | Polls if physical key or action is held down. |
| `mouse down?` | `mouse_down()` | `Boolean` | Checks if left mouse button is pressed. |
| `mouse x` | `mouse_x()` | `Number` | Returns current screen mouse X coordinate. |
| `mouse y` | `mouse_y()` | `Number` | Returns current screen mouse Y coordinate. |
| `distance to [target]` | `distance_to(target, other)` | `Number` | Euclidean distance $\sqrt{\Delta x^2 + \Delta y^2}$. |
| `timer` | `get_timer()` | `Number` | Elapsed seconds since game start or reset. |
| `reset timer` | `reset_timer()` | `Void` | Resets game timer to zero. |
| `[x position] of [Sprite1]` | `property_of(target, property)` | `Number/String`| Property introspection query. |

---

## 9. Operators & Math

| Original Scratch Block | scratch-lang Function / Syntax | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `() + ()` | `a + b` | `Number` | Addition. |
| `() - ()` | `a - b` | `Number` | Subtraction. |
| `() * ()` | `a * b` | `Number` | Multiplication. |
| `() / ()` | `a / b` | `Number` | Division. |
| `pick random (1) to (10)` | `random(min, max)` | `Number` | Generates uniform random integer or float. |
| `() < ()`, `() = ()`, `() > ()` | `<`, `==`, `>`, `<=`, `>=` | `Boolean` | Comparison operators. |
| `<> and <>`, `<> or <>`, `not <>` | `and`, `or`, `not` | `Boolean` | Logical operators. |
| `join [hello] [world]` | `text.join(a, b)` | `String` | Concatenates two strings together. |
| `letter (1) of [world]` | `text.letter_at(s, index)` | `String` | Returns 1-indexed character in string. |
| `length of [world]` | `text.length(s)` | `Number` | Returns number of characters in string. |
| `[apple] contains [a]?` | `text.contains(s, sub)` | `Boolean` | Checks if substring exists in text. |
| `() mod ()` | `a % b` or `math.mod(a, b)` | `Number` | Modulo remainder. |
| `round ()` | `math.round(n)` | `Number` | Rounds decimal to nearest whole integer. |
| `[abs / sqrt / sin / cos] of ()` | `math.abs(n)`, `math.sqrt(n)`, etc. | `Number` | Standard trigonometric & math library. |
