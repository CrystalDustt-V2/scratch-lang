import React, { useState } from 'react';
import { Copy, Check } from 'lucide-react';

export default function LanguageTour() {
  const [copiedSection, setCopiedSection] = useState(null);

  const copyCode = (code, id) => {
    navigator.clipboard.writeText(code);
    setCopiedSection(id);
    setTimeout(() => setCopiedSection(null), 2000);
  };

  return (
    <div className="docs-tour-container" style={{ maxWidth: '860px' }}>
      <div className="docs-category-hero">
        <h1>scratch-lang Language Guide</h1>
        <p>
          scratch-lang (<code>.sch</code>) is a typed, indented, text-first language that translates the visual semantics of Scratch 3.0 blocks into clean, high-performance code that compiles directly to native machine code or bytecode.
        </p>
      </div>

      {/* 1. Event System */}
      <section className="doc-func-entry">
        <h2 className="doc-func-title" style={{ fontSize: '1.5rem', marginBottom: '0.5rem' }}>
          1. Event Triggers (Hat Blocks)
        </h2>
        <p className="doc-func-desc">
          Top-level execution in scratch-lang begins with the <code>when</code> keyword followed by an event expression and an indented block.
        </p>

        <div className="doc-example-box">
          <div className="doc-example-header">
            <span>events.sch</span>
            <button
              type="button"
              onClick={() =>
                copyCode(
                  `when start:
    teleport("Player", 0, 0)
    say_for("Player", "Game Started!", 2)

when update:
    // Runs every single frame (60 FPS)
    point_towards("Turret", "mouse")

when action.down("up"):
    move_up("Player", 5)

when action.press("space"):
    jump("Player", 12)

when Player touches Enemy:
    damage("Player", 1)
    play_sound("hit")`,
                  'events'
                )
              }
              className="doc-copy-btn"
              style={{ position: 'static' }}
            >
              {copiedSection === 'events' ? <Check size={13} /> : <Copy size={13} />}
              <span>{copiedSection === 'events' ? 'Copied' : 'Copy'}</span>
            </button>
          </div>
          <pre className="doc-example-code">
            <code>{`when start:
    teleport("Player", 0, 0)
    say_for("Player", "Game Started!", 2)

when update:
    // Runs every single frame (60 FPS)
    point_towards("Turret", "mouse")

when action.down("up"):
    move_up("Player", 5)

when action.press("space"):
    jump("Player", 12)

when Player touches Enemy:
    damage("Player", 1)
    play_sound("hit")`}</code>
          </pre>
        </div>
      </section>

      {/* 2. Control Flow */}
      <section className="doc-func-entry">
        <h2 className="doc-func-title" style={{ fontSize: '1.5rem', marginBottom: '0.5rem' }}>
          2. Control Flow (C-Blocks)
        </h2>
        <p className="doc-func-desc">
          scratch-lang uses Python-style whitespace indentation for branch blocks and loops.
        </p>

        <div className="doc-example-box">
          <div className="doc-example-header">
            <span>control_flow.sch</span>
            <button
              type="button"
              onClick={() =>
                copyCode(
                  `if health > 0:
    move("Player", 10)
else:
    broadcast("game_over")

repeat 10:
    move_right("Player", 5)
    wait(0.1)

while distance_to("Player", "Goal") > 5:
    point_towards("Player", "Goal")
    move("Player", 2)`,
                  'control'
                )
              }
              className="doc-copy-btn"
              style={{ position: 'static' }}
            >
              {copiedSection === 'control' ? <Check size={13} /> : <Copy size={13} />}
              <span>{copiedSection === 'control' ? 'Copied' : 'Copy'}</span>
            </button>
          </div>
          <pre className="doc-example-code">
            <code>{`if health > 0:
    move("Player", 10)
else:
    broadcast("game_over")

repeat 10:
    move_right("Player", 5)
    wait(0.1)

while distance_to("Player", "Goal") > 5:
    point_towards("Player", "Goal")
    move("Player", 2)`}</code>
          </pre>
        </div>
      </section>

      {/* 3. Variables & Lists */}
      <section className="doc-func-entry">
        <h2 className="doc-func-title" style={{ fontSize: '1.5rem', marginBottom: '0.5rem' }}>
          3. Variables &amp; Dynamic Lists
        </h2>
        <p className="doc-func-desc">
          Variables can be initialized and manipulated using standard operators or Scratch-compatible block primitives. Lists support 1-based indexing conforming to Scratch conventions.
        </p>

        <div className="doc-example-box">
          <div className="doc-example-header">
            <span>data.sch</span>
            <button
              type="button"
              onClick={() =>
                copyCode(
                  `// Variable assignment
score = 0
player_name = "Hero"

// List manipulation
list.add("inventory", "Sword")
list.add("inventory", "Shield")
list.add("inventory", "Potion")

if list.contains("inventory", "Potion"):
    say_for("Player", "I have a healing potion!", 2)
    list.delete("inventory", 3)`,
                  'data'
                )
              }
              className="doc-copy-btn"
              style={{ position: 'static' }}
            >
              {copiedSection === 'data' ? <Check size={13} /> : <Copy size={13} />}
              <span>{copiedSection === 'data' ? 'Copied' : 'Copy'}</span>
            </button>
          </div>
          <pre className="doc-example-code">
            <code>{`// Variable assignment
score = 0
player_name = "Hero"

// List manipulation
list.add("inventory", "Sword")
list.add("inventory", "Shield")
list.add("inventory", "Potion")

if list.contains("inventory", "Potion"):
    say_for("Player", "I have a healing potion!", 2)
    list.delete("inventory", 3)`}</code>
          </pre>
        </div>
      </section>

      {/* 4. Complete Game Example */}
      <section className="doc-func-entry">
        <h2 className="doc-func-title" style={{ fontSize: '1.5rem', marginBottom: '0.5rem' }}>
          4. Complete Top-Down Game Controller
        </h2>
        <p className="doc-func-desc">
          Here is a complete, runnable 4-way 2D game script utilizing the new directional movement primitives (<code>move_up</code>, <code>move_down</code>, <code>move_left</code>, <code>move_right</code>) with sprite bounds and collision events.
        </p>

        <div className="doc-example-box">
          <div className="doc-example-header">
            <span>main.sch</span>
            <button
              type="button"
              onClick={() =>
                copyCode(
                  `// Scene Initialization
when start:
    teleport("Player", 0, 0)
    set_rotation_style("Player", "left-right")
    say_for("Player", "Use Arrow Keys or WASD to move!", 2)

// 4-Way Directional Movement
when action.down("up"):
    move_up("Player", 5)

when action.down("down"):
    move_down("Player", 5)

when action.down("left"):
    move_left("Player", 5)

when action.down("right"):
    move_right("Player", 5)

// Collision & Boundary Logic
when Player touches Coin:
    score += 10
    play_sound("coin")
    teleport("Coin", random_number(-200, 200), random_number(-150, 150))

when update:
    bounce_on_edge("Player")`,
                  'game'
                )
              }
              className="doc-copy-btn"
              style={{ position: 'static' }}
            >
              {copiedSection === 'game' ? <Check size={13} /> : <Copy size={13} />}
              <span>{copiedSection === 'game' ? 'Copied' : 'Copy'}</span>
            </button>
          </div>
          <pre className="doc-example-code">
            <code>{`// Scene Initialization
when start:
    teleport("Player", 0, 0)
    set_rotation_style("Player", "left-right")
    say_for("Player", "Use Arrow Keys or WASD to move!", 2)

// 4-Way Directional Movement
when action.down("up"):
    move_up("Player", 5)

when action.down("down"):
    move_down("Player", 5)

when action.down("left"):
    move_left("Player", 5)

when action.down("right"):
    move_right("Player", 5)

// Collision & Boundary Logic
when Player touches Coin:
    score += 10
    play_sound("coin")
    teleport("Coin", random_number(-200, 200), random_number(-150, 150))

when update:
    bounce_on_edge("Player")`}</code>
          </pre>
        </div>
      </section>
    </div>
  );
}
