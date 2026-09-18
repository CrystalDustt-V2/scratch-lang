export const FUNCTIONS_DATA = [
  // ==================== 1. MOTION (18 Blocks) ====================
  {
    id: 'motion-move',
    name: 'move',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_movesteps',
    syntax: 'move(target, steps, [dy])',
    returnType: 'Void',
    description: 'Moves the specified sprite forward in the direction it is facing by the given number of steps, or by explicit (dx, dy) deltas.',
    parameters: [
      { name: 'target', type: 'String', description: 'Name of the sprite entity to move', required: true },
      { name: 'steps', type: 'Number', description: 'Distance in steps or horizontal displacement dx', required: true },
      { name: 'dy', type: 'Number', description: 'Optional vertical displacement delta', required: false, default: '0' }
    ],
    example: 'when start:\n    move("Player", 10)\n    move("Player", 5, -2)',
    lspSnippet: 'move("${1:Player}", ${2:10})',
    notes: 'If pen is down for this sprite, an automatic PenStroke trail is recorded into the world pen buffer.'
  },
  {
    id: 'motion-move-up',
    name: 'move_up',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_moveup',
    syntax: 'move_up(target, [steps])',
    returnType: 'Void',
    description: 'Moves the sprite upward along the vertical Y-axis (+Y). Ideal for 2D top-down games.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'steps', type: 'Number', description: 'Distance in pixels to move upward', required: false, default: '5' }
    ],
    example: 'when action.down("up"):\n    move_up("Player", 5)',
    lspSnippet: 'move_up("${1:Player}", ${2:5})',
    notes: 'Increases sprite Y coordinate. Default distance is 5 pixels.'
  },
  {
    id: 'motion-up',
    name: 'up',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_moveup_alias',
    syntax: 'up(target, [steps])',
    returnType: 'Void',
    description: 'Convenience alias for move_up: Moves sprite upward along the vertical Y-axis (+Y).',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'steps', type: 'Number', description: 'Distance in pixels to move upward', required: false, default: '5' }
    ],
    example: 'when action.down("up"):\n    up("Player", 5)',
    lspSnippet: 'up("${1:Player}", ${2:5})',
    notes: 'Identical to move_up(target, steps).'
  },
  {
    id: 'motion-move-down',
    name: 'move_down',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_movedown',
    syntax: 'move_down(target, [steps])',
    returnType: 'Void',
    description: 'Moves the sprite downward along the vertical Y-axis (-Y). Ideal for 2D top-down games.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'steps', type: 'Number', description: 'Distance in pixels to move downward', required: false, default: '5' }
    ],
    example: 'when action.down("down"):\n    move_down("Player", 5)',
    lspSnippet: 'move_down("${1:Player}", ${2:5})',
    notes: 'Decreases sprite Y coordinate. Default distance is 5 pixels.'
  },
  {
    id: 'motion-down',
    name: 'down',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_movedown_alias',
    syntax: 'down(target, [steps])',
    returnType: 'Void',
    description: 'Convenience alias for move_down: Moves sprite downward along the vertical Y-axis (-Y).',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'steps', type: 'Number', description: 'Distance in pixels to move downward', required: false, default: '5' }
    ],
    example: 'when action.down("down"):\n    down("Player", 5)',
    lspSnippet: 'down("${1:Player}", ${2:5})',
    notes: 'Identical to move_down(target, steps).'
  },
  {
    id: 'motion-move-left',
    name: 'move_left',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_moveleft',
    syntax: 'move_left(target, [steps])',
    returnType: 'Void',
    description: 'Moves the sprite leftward along the horizontal X-axis (-X).',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'steps', type: 'Number', description: 'Distance in pixels to move leftward', required: false, default: '5' }
    ],
    example: 'when action.down("left"):\n    move_left("Player", 5)',
    lspSnippet: 'move_left("${1:Player}", ${2:5})',
    notes: 'Decreases sprite X coordinate. Default distance is 5 pixels.'
  },
  {
    id: 'motion-left',
    name: 'left',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_moveleft_alias',
    syntax: 'left(target, [steps])',
    returnType: 'Void',
    description: 'Convenience alias for move_left: Moves sprite leftward along the horizontal X-axis (-X).',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'steps', type: 'Number', description: 'Distance in pixels to move leftward', required: false, default: '5' }
    ],
    example: 'when action.down("left"):\n    left("Player", 5)',
    lspSnippet: 'left("${1:Player}", ${2:5})',
    notes: 'Identical to move_left(target, steps).'
  },
  {
    id: 'motion-move-right',
    name: 'move_right',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_moveright',
    syntax: 'move_right(target, [steps])',
    returnType: 'Void',
    description: 'Moves the sprite rightward along the horizontal X-axis (+X).',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'steps', type: 'Number', description: 'Distance in pixels to move rightward', required: false, default: '5' }
    ],
    example: 'when action.down("right"):\n    move_right("Player", 5)',
    lspSnippet: 'move_right("${1:Player}", ${2:5})',
    notes: 'Increases sprite X coordinate. Default distance is 5 pixels.'
  },
  {
    id: 'motion-right',
    name: 'right',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_moveright_alias',
    syntax: 'right(target, [steps])',
    returnType: 'Void',
    description: 'Convenience alias for move_right: Moves sprite rightward along the horizontal X-axis (+X).',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'steps', type: 'Number', description: 'Distance in pixels to move rightward', required: false, default: '5' }
    ],
    example: 'when action.down("right"):\n    right("Player", 5)',
    lspSnippet: 'right("${1:Player}", ${2:5})',
    notes: 'Identical to move_right(target, steps).'
  },
  {
    id: 'motion-jump',
    name: 'jump',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_jump',
    syntax: 'jump(target, [force])',
    returnType: 'Void',
    description: 'Applies an instantaneous upward jump impulse force to the target sprite (for platformers with gravity).',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'force', type: 'Number', description: 'Upward jump impulse magnitude', required: false, default: '10' }
    ],
    example: 'when action.press("jump"):\n    jump("Player", 12)',
    lspSnippet: 'jump("${1:Player}", ${2:10})',
    notes: 'Sets vertical velocity component for physics-enabled sprites.'
  },
  {
    id: 'motion-stop',
    name: 'stop',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_stop',
    syntax: 'stop(target)',
    returnType: 'Void',
    description: 'Immediately halts sprite movement by resetting its velocity to zero.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'when action.release("right"):\n    stop("Player")',
    lspSnippet: 'stop("${1:Player}")',
    notes: 'Resets velocity to (0, 0).'
  },
  {
    id: 'motion-turn-right',
    name: 'turn_right',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_turnright',
    syntax: 'turn_right(target, degrees)',
    returnType: 'Void',
    description: 'Rotates the sprite clockwise by the specified angle in degrees.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'degrees', type: 'Number', description: 'Clockwise angle rotation in degrees', required: true }
    ],
    example: 'when action.press("right"):\n    turn_right("Player", 15)',
    lspSnippet: 'turn_right("${1:Player}", ${2:15})',
    notes: 'Modulates rotation around normalized 0-360 range.'
  },
  {
    id: 'motion-turn-left',
    name: 'turn_left',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_turnleft',
    syntax: 'turn_left(target, degrees)',
    returnType: 'Void',
    description: 'Rotates the sprite counter-clockwise by the specified angle in degrees.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'degrees', type: 'Number', description: 'Counter-clockwise angle in degrees', required: true }
    ],
    example: 'when action.press("left"):\n    turn_left("Player", 15)',
    lspSnippet: 'turn_left("${1:Player}", ${2:15})',
    notes: 'Subtracts degrees from current transform heading.'
  },
  {
    id: 'motion-go-to',
    name: 'go_to',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_goto',
    syntax: 'go_to(target, destination)',
    returnType: 'Void',
    description: 'Instantly moves the sprite to another target sprite, the mouse cursor, or a random position.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity to move', required: true },
      { name: 'destination', type: 'String', description: '"mouse", "random", or another sprite entity name', required: true }
    ],
    example: 'when start:\n    go_to("Cat", "random")\n    go_to("Follower", "mouse")',
    lspSnippet: 'go_to("${1:Player}", "${2|mouse,random,Sprite1|}")',
    notes: 'Pen stroke trail is drawn if pen is down.'
  },
  {
    id: 'motion-teleport',
    name: 'teleport',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_gotoxy',
    syntax: 'teleport(target, x, y)',
    returnType: 'Void',
    description: 'Instantly positions the sprite at the exact Cartesian coordinates (x, y).',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'x', type: 'Number', description: 'Horizontal coordinate (-240 to 240)', required: true },
      { name: 'y', type: 'Number', description: 'Vertical coordinate (-180 to 180)', required: true }
    ],
    example: 'when start:\n    teleport("Player", 0, 0)',
    lspSnippet: 'teleport("${1:Player}", ${2:0}, ${3:0})',
    notes: 'Origin (0, 0) is at stage center.'
  },
  {
    id: 'motion-glide-to',
    name: 'glide_to',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_glideto',
    syntax: 'glide_to(target, seconds, destination)',
    returnType: 'Void',
    description: 'Smoothly interpolates sprite position towards a named target, mouse, or random position over a duration.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'seconds', type: 'Number', description: 'Duration in seconds', required: true },
      { name: 'destination', type: 'String', description: '"mouse", "random", or sprite name', required: true }
    ],
    example: 'when click("Button"):\n    glide_to("Cat", 1.5, "mouse")',
    lspSnippet: 'glide_to("${1:Player}", ${2:1.0}, "${3:mouse}")',
    notes: 'Managed via active GlideTween linear interpolation fibers.'
  },
  {
    id: 'motion-glide',
    name: 'glide',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_glidesecstoxy',
    syntax: 'glide(target, seconds, x, y)',
    returnType: 'Void',
    description: 'Smoothly glides the sprite to exact coordinates (x, y) over the specified duration in seconds.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'seconds', type: 'Number', description: 'Duration in seconds', required: true },
      { name: 'x', type: 'Number', description: 'Destination X coordinate', required: true },
      { name: 'y', type: 'Number', description: 'Destination Y coordinate', required: true }
    ],
    example: 'when start:\n    glide("Cat", 2.0, 100, 50)',
    lspSnippet: 'glide("${1:Player}", ${2:1.0}, ${3:100}, ${4:50})',
    notes: 'Non-blocking in coroutine fibers with smooth 60 FPS delta tick updates.'
  },
  {
    id: 'motion-point-in-direction',
    name: 'point_in_direction',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_pointindirection',
    syntax: 'point_in_direction(target, degrees)',
    returnType: 'Void',
    description: 'Sets the sprite orientation to the specified angle: 0 (up), 90 (right), 180 (down), -90 (left).',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'degrees', type: 'Number', description: 'Heading angle in Scratch convention degrees', required: true }
    ],
    example: 'when action.press("up"):\n    point_in_direction("Player", 0)',
    lspSnippet: 'point_in_direction("${1:Player}", ${2:90})',
    notes: 'Follows Scratch convention: 0 is North/Up, 90 is East/Right.'
  },
  {
    id: 'motion-point-towards',
    name: 'point_towards',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_pointtowards',
    syntax: 'point_towards(target, other)',
    returnType: 'Void',
    description: 'Orients the sprite to face towards the mouse cursor or another named sprite.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'other', type: 'String', description: '"mouse" or target sprite name', required: true }
    ],
    example: 'when update:\n    point_towards("Turret", "mouse")',
    lspSnippet: 'point_towards("${1:Player}", "${2:mouse}")',
    notes: 'Calculates atan2(dy, dx) automatically and sets rotation heading.'
  },
  {
    id: 'motion-change-x',
    name: 'change_x',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_changexby',
    syntax: 'change_x(target, dx)',
    returnType: 'Void',
    description: 'Modifies the horizontal position of the sprite by dx.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'dx', type: 'Number', description: 'Horizontal offset delta', required: true }
    ],
    example: 'when action.press("d"):\n    change_x("Player", 10)',
    lspSnippet: 'change_x("${1:Player}", ${2:10})',
    notes: 'Increments transform.x.'
  },
  {
    id: 'motion-set-x',
    name: 'set_x',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_setx',
    syntax: 'set_x(target, x)',
    returnType: 'Void',
    description: 'Directly sets the horizontal coordinate x of the sprite.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'x', type: 'Number', description: 'New X position', required: true }
    ],
    example: 'when start:\n    set_x("Player", -200)',
    lspSnippet: 'set_x("${1:Player}", ${2:0})',
    notes: 'Overwrites transform.x directly.'
  },
  {
    id: 'motion-change-y',
    name: 'change_y',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_changeyby',
    syntax: 'change_y(target, dy)',
    returnType: 'Void',
    description: 'Modifies the vertical position of the sprite by dy.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'dy', type: 'Number', description: 'Vertical offset delta', required: true }
    ],
    example: 'when action.press("w"):\n    change_y("Player", 10)',
    lspSnippet: 'change_y("${1:Player}", ${2:10})',
    notes: 'Increments transform.y.'
  },
  {
    id: 'motion-set-y',
    name: 'set_y',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_sety',
    syntax: 'set_y(target, y)',
    returnType: 'Void',
    description: 'Directly sets the vertical coordinate y of the sprite.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'y', type: 'Number', description: 'New Y position', required: true }
    ],
    example: 'when start:\n    set_y("Player", 100)',
    lspSnippet: 'set_y("${1:Player}", ${2:0})',
    notes: 'Overwrites transform.y directly.'
  },
  {
    id: 'motion-bounce-on-edge',
    name: 'bounce_on_edge',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_ifonedgebounce',
    syntax: 'bounce_on_edge(target)',
    returnType: 'Void',
    description: 'If the sprite touches the stage boundary, it bounces off and inverts its heading angle.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'when update:\n    move("Ball", 5)\n    bounce_on_edge("Ball")',
    lspSnippet: 'bounce_on_edge("${1:Player}")',
    notes: 'Clamps to stage boundaries [-240..240, -180..180].'
  },
  {
    id: 'motion-set-rotation-style',
    name: 'set_rotation_style',
    category: 'motion',
    shape: 'Stack',
    opcode: 'motion_setrotationstyle',
    syntax: 'set_rotation_style(target, style)',
    returnType: 'Void',
    description: 'Configures sprite visual rotation behavior: "all-around", "left-right", or "don\'t rotate".',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'style', type: 'String', description: '"all-around" | "left-right" | "don\'t rotate"', required: true }
    ],
    example: 'when start:\n    set_rotation_style("Player", "left-right")',
    lspSnippet: 'set_rotation_style("${1:Player}", "${2|all-around,left-right,don\'t rotate|}")',
    notes: 'In "left-right", flips horizontal scale when pointing leftward.'
  },
  {
    id: 'motion-x-position',
    name: 'x_position',
    category: 'motion',
    shape: 'Reporter',
    opcode: 'motion_xposition',
    syntax: 'x_position(target)',
    returnType: 'Number',
    description: 'Returns the current horizontal coordinate x of the sprite.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'if x_position("Player") > 200:\n    teleport("Player", -200, 0)',
    lspSnippet: 'x_position("${1:Player}")',
    notes: 'Returns f64 coordinate centered on stage.'
  },
  {
    id: 'motion-y-position',
    name: 'y_position',
    category: 'motion',
    shape: 'Reporter',
    opcode: 'motion_yposition',
    syntax: 'y_position(target)',
    returnType: 'Number',
    description: 'Returns the current vertical coordinate y of the sprite.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'if y_position("Player") < -170:\n    say("Player", "Fell out!")',
    lspSnippet: 'y_position("${1:Player}")',
    notes: 'Returns f64 coordinate centered on stage.'
  },
  {
    id: 'motion-get-direction',
    name: 'get_direction',
    category: 'motion',
    shape: 'Reporter',
    opcode: 'motion_direction',
    syntax: 'get_direction(target)',
    returnType: 'Number',
    description: 'Returns the current heading direction angle of the sprite in degrees.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'let dir = get_direction("Player")',
    lspSnippet: 'get_direction("${1:Player}")',
    notes: 'Returns Scratch convention angle (-180 to 180 or 0 to 360).'
  },

  // ==================== 2. LOOKS (21 Blocks) ====================
  {
    id: 'looks-say-for',
    name: 'say_for',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_sayforsecs',
    syntax: 'say_for(target, message, seconds)',
    returnType: 'Void',
    description: 'Displays a speech bubble above the sprite with the message for the given duration in seconds.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'message', type: 'String', description: 'Text to display in bubble', required: true },
      { name: 'seconds', type: 'Number', description: 'Duration in seconds', required: true }
    ],
    example: 'when start:\n    say_for("Cat", "Hello World!", 2)',
    lspSnippet: 'say_for("${1:Player}", "${2:Hello!}", ${3:2})',
    notes: 'Bubble dismisses automatically after timer expires.'
  },
  {
    id: 'looks-say',
    name: 'say',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_say',
    syntax: 'say(target, message)',
    returnType: 'Void',
    description: 'Displays a speech bubble above the sprite permanently until cleared or replaced.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'message', type: 'String', description: 'Text to display (empty string clears bubble)', required: true }
    ],
    example: 'say("Cat", "Score: 100")\nsay("Cat", "") # Clears bubble',
    lspSnippet: 'say("${1:Player}", "${2:Hello!}")',
    notes: 'Pass empty string to dismiss.'
  },
  {
    id: 'looks-think-for',
    name: 'think_for',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_thinkforsecs',
    syntax: 'think_for(target, message, seconds)',
    returnType: 'Void',
    description: 'Displays a thought cloud bubble above the sprite for a specified duration.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'message', type: 'String', description: 'Thought message text', required: true },
      { name: 'seconds', type: 'Number', description: 'Duration in seconds', required: true }
    ],
    example: 'think_for("Cat", "Hmm... what to do?", 3)',
    lspSnippet: 'think_for("${1:Player}", "${2:Hmm...}", ${3:2})',
    notes: 'Renders thought bubble style in GUI preview.'
  },
  {
    id: 'looks-think',
    name: 'think',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_think',
    syntax: 'think(target, message)',
    returnType: 'Void',
    description: 'Displays a persistent thought cloud bubble above the sprite.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'message', type: 'String', description: 'Thought message text', required: true }
    ],
    example: 'think("Cat", "Thinking deeply...")',
    lspSnippet: 'think("${1:Player}", "${2:Hmm...}")',
    notes: 'Pass empty string to remove.'
  },
  {
    id: 'looks-switch-costume',
    name: 'switch_costume',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_switchcostumeto',
    syntax: 'switch_costume(target, costume_name_or_number)',
    returnType: 'Void',
    description: 'Changes the active costume of the sprite to a specified name or 1-based index.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'costume', type: 'String|Number', description: 'Costume name or 1-based index', required: true }
    ],
    example: 'switch_costume("Player", "walk_2")',
    lspSnippet: 'switch_costume("${1:Player}", "${2:costume2}")',
    notes: 'Resolves against sprite costume manifest.'
  },
  {
    id: 'looks-next-costume',
    name: 'next_costume',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_nextcostume',
    syntax: 'next_costume(target)',
    returnType: 'Void',
    description: 'Cycles to the sprite\'s next costume, wrapping around to the first costume at the end.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'when update:\n    wait(0.1)\n    next_costume("Player")',
    lspSnippet: 'next_costume("${1:Player}")',
    notes: 'Increments costume_index modulo costumes length.'
  },
  {
    id: 'looks-switch-backdrop',
    name: 'switch_backdrop',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_switchbackdropto',
    syntax: 'switch_backdrop(backdrop_name)',
    returnType: 'Void',
    description: 'Switches the stage background/backdrop to the specified asset name or index.',
    parameters: [
      { name: 'backdrop_name', type: 'String|Number', description: 'Name or index of backdrop', required: true }
    ],
    example: 'when scene.switched("level2"):\n    switch_backdrop("castle_room")',
    lspSnippet: 'switch_backdrop("${1:backdrop1}")',
    notes: 'Also aliases with background.set(name).'
  },
  {
    id: 'looks-switch-backdrop-and-wait',
    name: 'switch_backdrop_and_wait',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_switchbackdroptoandwait',
    syntax: 'switch_backdrop_and_wait(backdrop_name)',
    returnType: 'Void',
    description: 'Switches the backdrop and dispatches backdrop transition events, waiting for handlers.',
    parameters: [
      { name: 'backdrop_name', type: 'String', description: 'Name of backdrop', required: true }
    ],
    example: 'switch_backdrop_and_wait("game_over_screen")',
    lspSnippet: 'switch_backdrop_and_wait("${1:backdrop2}")',
    notes: 'Triggers when scene.switched events.'
  },
  {
    id: 'looks-next-backdrop',
    name: 'next_backdrop',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_nextbackdrop',
    syntax: 'next_backdrop()',
    returnType: 'Void',
    description: 'Advances the stage to the next backdrop in the project scenes array.',
    parameters: [],
    example: 'when action.press("space"):\n    next_backdrop()',
    lspSnippet: 'next_backdrop()',
    notes: 'Cycles active_backdrop_index sequentially.'
  },
  {
    id: 'looks-change-size',
    name: 'change_size',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_changesizeby',
    syntax: 'change_size(target, delta)',
    returnType: 'Void',
    description: 'Changes sprite scale percentage by the given delta (e.g. +10 or -10).',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'delta', type: 'Number', description: 'Scale percentage delta', required: true }
    ],
    example: 'change_size("Player", 10)',
    lspSnippet: 'change_size("${1:Player}", ${2:10})',
    notes: 'Clamped to minimum size of 5%.'
  },
  {
    id: 'looks-set-size',
    name: 'set_size',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_setsizeto',
    syntax: 'set_size(target, percent)',
    returnType: 'Void',
    description: 'Sets absolute sprite scale percentage (100 is 1.0x normal size).',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'percent', type: 'Number', description: 'Scale percentage (100 = 100%)', required: true }
    ],
    example: 'set_size("Player", 150)',
    lspSnippet: 'set_size("${1:Player}", ${2:100})',
    notes: 'Updates transform.scale_x and transform.scale_y.'
  },
  {
    id: 'looks-change-effect',
    name: 'change_effect',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_changeeffectby',
    syntax: 'change_effect(target, effect, delta)',
    returnType: 'Void',
    description: 'Adjusts a visual shader effect on the sprite (color, ghost, brightness, whirl, pixelate).',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'effect', type: 'String', description: '"color" | "ghost" | "brightness" | "whirl" | "pixelate"', required: true },
      { name: 'delta', type: 'Number', description: 'Amount to change effect value by', required: true }
    ],
    example: 'change_effect("Ghost", "ghost", 25)',
    lspSnippet: 'change_effect("${1:Player}", "${2|color,ghost,brightness,whirl,pixelate|}", ${3:25})',
    notes: 'Ghost at 100 makes the sprite fully transparent.'
  },
  {
    id: 'looks-set-effect',
    name: 'set_effect',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_seteffectto',
    syntax: 'set_effect(target, effect, value)',
    returnType: 'Void',
    description: 'Directly sets a visual shader effect parameter value.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'effect', type: 'String', description: '"color" | "ghost" | "brightness" | "whirl" | "pixelate"', required: true },
      { name: 'value', type: 'Number', description: 'Absolute effect value', required: true }
    ],
    example: 'set_effect("Player", "ghost", 50)',
    lspSnippet: 'set_effect("${1:Player}", "${2|color,ghost,brightness,whirl,pixelate|}", ${3:0})',
    notes: 'Effects persist until cleared or modified.'
  },
  {
    id: 'looks-clear-effects',
    name: 'clear_effects',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_cleargraphiceffects',
    syntax: 'clear_effects(target)',
    returnType: 'Void',
    description: 'Resets all active visual shader effects to default values for the sprite.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'clear_effects("Player")',
    lspSnippet: 'clear_effects("${1:Player}")',
    notes: 'Resets ghost opacity to 0 and color modulation to baseline.'
  },
  {
    id: 'looks-show',
    name: 'show',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_show',
    syntax: 'show(target)',
    returnType: 'Void',
    description: 'Makes the sprite visible on stage (visible = true).',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'when start:\n    show("Player")',
    lspSnippet: 'show("${1:Player}")',
    notes: 'Enables rendering in stage pipeline.'
  },
  {
    id: 'looks-hide',
    name: 'hide',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_hide',
    syntax: 'hide(target)',
    returnType: 'Void',
    description: 'Hides the sprite from the stage (visible = false). Colliders are still tracked.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'when click("Coin"):\n    hide("Coin")',
    lspSnippet: 'hide("${1:Player}")',
    notes: 'Sprite remains in memory but will not render on screen.'
  },
  {
    id: 'looks-go-to-front',
    name: 'go_to_front',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_gotofrontback',
    syntax: 'go_to_front(target)',
    returnType: 'Void',
    description: 'Moves the sprite to the highest z-index rendering layer.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'go_to_front("HUD")',
    lspSnippet: 'go_to_front("${1:Player}")',
    notes: 'Guarantees the sprite is drawn in front of all other sprites.'
  },
  {
    id: 'looks-go-to-back',
    name: 'go_to_back',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_gotofrontback',
    syntax: 'go_to_back(target)',
    returnType: 'Void',
    description: 'Moves the sprite to the lowest z-index rendering layer behind other sprites.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'go_to_back("BackgroundScenery")',
    lspSnippet: 'go_to_back("${1:Player}")',
    notes: 'Renders immediately above the stage backdrop.'
  },
  {
    id: 'looks-go-forward-layers',
    name: 'go_forward_layers',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_goforwardbackwardlayers',
    syntax: 'go_forward_layers(target, layers)',
    returnType: 'Void',
    description: 'Shifts the sprite forward by a number of z-index layers.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'layers', type: 'Number', description: 'Number of layers forward', required: true }
    ],
    example: 'go_forward_layers("Player", 2)',
    lspSnippet: 'go_forward_layers("${1:Player}", ${2:1})',
    notes: 'Increments layer depth.'
  },
  {
    id: 'looks-go-back-layers',
    name: 'go_back_layers',
    category: 'looks',
    shape: 'Stack',
    opcode: 'looks_goforwardbackwardlayers',
    syntax: 'go_back_layers(target, layers)',
    returnType: 'Void',
    description: 'Shifts the sprite backward by a number of z-index layers.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'layers', type: 'Number', description: 'Number of layers backward', required: true }
    ],
    example: 'go_back_layers("Shadow", 1)',
    lspSnippet: 'go_back_layers("${1:Player}", ${2:1})',
    notes: 'Decrements layer depth.'
  },
  {
    id: 'looks-get-costume-number',
    name: 'get_costume_number',
    category: 'looks',
    shape: 'Reporter',
    opcode: 'looks_costumenumbername',
    syntax: 'get_costume_number(target)',
    returnType: 'Number',
    description: 'Returns the active costume 1-based index of the sprite.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'if get_costume_number("Player") == 1:\n    say("Player", "Idle")',
    lspSnippet: 'get_costume_number("${1:Player}")',
    notes: '1-indexed per Scratch convention.'
  },
  {
    id: 'looks-get-costume-name',
    name: 'get_costume_name',
    category: 'looks',
    shape: 'Reporter',
    opcode: 'looks_costumenumbername',
    syntax: 'get_costume_name(target)',
    returnType: 'String',
    description: 'Returns the active costume string name/label of the sprite.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'let name = get_costume_name("Player")',
    lspSnippet: 'get_costume_name("${1:Player}")',
    notes: 'Reads costume_name from entity state.'
  },
  {
    id: 'looks-get-backdrop-number',
    name: 'get_backdrop_number',
    category: 'looks',
    shape: 'Reporter',
    opcode: 'looks_backdropnumbername',
    syntax: 'get_backdrop_number()',
    returnType: 'Number',
    description: 'Returns the active 1-based index number of the current stage backdrop.',
    parameters: [],
    example: 'if get_backdrop_number() == 2:\n    start_boss_fight()',
    lspSnippet: 'get_backdrop_number()',
    notes: 'Returns active_backdrop_index + 1.'
  },
  {
    id: 'looks-get-backdrop-name',
    name: 'get_backdrop_name',
    category: 'looks',
    shape: 'Reporter',
    opcode: 'looks_backdropnumbername',
    syntax: 'get_backdrop_name()',
    returnType: 'String',
    description: 'Returns the string name of the currently active stage backdrop.',
    parameters: [],
    example: 'if get_backdrop_name() == "game_over":\n    stop_all()',
    lspSnippet: 'get_backdrop_name()',
    notes: 'Returns world.background or backdrop list entry.'
  },
  {
    id: 'looks-get-size',
    name: 'get_size',
    category: 'looks',
    shape: 'Reporter',
    opcode: 'looks_size',
    syntax: 'get_size(target)',
    returnType: 'Number',
    description: 'Returns current sprite scaling percent (100 = 1.0x normal size).',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'let sz = get_size("Player")',
    lspSnippet: 'get_size("${1:Player}")',
    notes: 'Derived from entity transform.scale_x * 100.'
  },

  // ==================== 3. SOUND (9 Blocks) ====================
  {
    id: 'sound-play-until-done',
    name: 'sound.play_until_done',
    category: 'sound',
    shape: 'Stack',
    opcode: 'sound_playuntildone',
    syntax: 'sound.play_until_done(sound_name)',
    returnType: 'Void',
    description: 'Plays the audio sound and blocks/suspends the executing fiber until audio duration finishes.',
    parameters: [
      { name: 'sound_name', type: 'String', description: 'Sound asset name', required: true }
    ],
    example: 'sound.play_until_done("meow")\nsay("Cat", "Done meowing!")',
    lspSnippet: 'sound.play_until_done("${1:meow}")',
    notes: 'Yields execution fiber in runtime scheduler.'
  },
  {
    id: 'sound-play',
    name: 'sound.play',
    category: 'sound',
    shape: 'Stack',
    opcode: 'sound_play',
    syntax: 'sound.play(sound_name)',
    returnType: 'Void',
    description: 'Fires asynchronous sound playback channel without blocking code execution.',
    parameters: [
      { name: 'sound_name', type: 'String', description: 'Sound asset name', required: true }
    ],
    example: 'when click("Coin"):\n    sound.play("coin_pickup")',
    lspSnippet: 'sound.play("${1:sound_name}")',
    notes: 'Dispatches to audio playback thread immediately.'
  },
  {
    id: 'sound-stop-all',
    name: 'sound.stop_all',
    category: 'sound',
    shape: 'Stack',
    opcode: 'sound_stopallsounds',
    syntax: 'sound.stop_all()',
    returnType: 'Void',
    description: 'Halts all active sound playback channels immediately.',
    parameters: [],
    example: 'when start:\n    sound.stop_all()',
    lspSnippet: 'sound.stop_all()',
    notes: 'Stops background music and all sound effect streams.'
  },
  {
    id: 'sound-change-effect',
    name: 'sound.change_effect',
    category: 'sound',
    shape: 'Stack',
    opcode: 'sound_changeeffectby',
    syntax: 'sound.change_effect(effect, delta)',
    returnType: 'Void',
    description: 'Adjusts audio playback DSP parameter (pitch or pan).',
    parameters: [
      { name: 'effect', type: 'String', description: '"pitch" | "pan"', required: true },
      { name: 'delta', type: 'Number', description: 'Amount to adjust effect by', required: true }
    ],
    example: 'sound.change_effect("pitch", 10)',
    lspSnippet: 'sound.change_effect("${1|pitch,pan|}", ${2:10})',
    notes: 'Pitch 10 represents 10 semitones or 100 cents.'
  },
  {
    id: 'sound-set-effect',
    name: 'sound.set_effect',
    category: 'sound',
    shape: 'Stack',
    opcode: 'sound_seteffectto',
    syntax: 'sound.set_effect(effect, value)',
    returnType: 'Void',
    description: 'Directly sets an audio DSP parameter (pitch or pan).',
    parameters: [
      { name: 'effect', type: 'String', description: '"pitch" | "pan"', required: true },
      { name: 'value', type: 'Number', description: 'New parameter value (pan: -100 left to 100 right)', required: true }
    ],
    example: 'sound.set_effect("pan", -100) # Full left stereo',
    lspSnippet: 'sound.set_effect("${1|pitch,pan|}", ${2:0})',
    notes: 'Stored in world.sound_pitch and world.sound_pan.'
  },
  {
    id: 'sound-clear-effects',
    name: 'sound.clear_effects',
    category: 'sound',
    shape: 'Stack',
    opcode: 'sound_cleareffects',
    syntax: 'sound.clear_effects()',
    returnType: 'Void',
    description: 'Resets all audio DSP effects (pitch and pan) to neutral baseline (0.0).',
    parameters: [],
    example: 'sound.clear_effects()',
    lspSnippet: 'sound.clear_effects()',
    notes: 'Resets pitch to 0 and center-pans audio.'
  },
  {
    id: 'sound-change-volume',
    name: 'sound.change_volume',
    category: 'sound',
    shape: 'Stack',
    opcode: 'sound_changevolumeby',
    syntax: 'sound.change_volume(delta)',
    returnType: 'Void',
    description: 'Adjusts master audio playback volume by delta.',
    parameters: [
      { name: 'delta', type: 'Number', description: 'Volume delta (clamped 0 to 100%)', required: true }
    ],
    example: 'sound.change_volume(-10)',
    lspSnippet: 'sound.change_volume(${1:-10})',
    notes: 'Clamped between 0.0 and 100.0.'
  },
  {
    id: 'sound-set-volume',
    name: 'sound.set_volume',
    category: 'sound',
    shape: 'Stack',
    opcode: 'sound_setvolumeto',
    syntax: 'sound.set_volume(percent)',
    returnType: 'Void',
    description: 'Sets absolute master volume percentage (0 to 100).',
    parameters: [
      { name: 'percent', type: 'Number', description: 'Volume percent (0 to 100)', required: true }
    ],
    example: 'sound.set_volume(80)',
    lspSnippet: 'sound.set_volume(${1:100})',
    notes: 'Stored in world variable __volume.'
  },
  {
    id: 'sound-get-volume',
    name: 'sound.get_volume',
    category: 'sound',
    shape: 'Reporter',
    opcode: 'sound_volume',
    syntax: 'sound.get_volume()',
    returnType: 'Number',
    description: 'Returns the current audio volume percentage (0 to 100).',
    parameters: [],
    example: 'let vol = sound.get_volume()',
    lspSnippet: 'sound.get_volume()',
    notes: 'Defaults to 100.0%.'
  },

  // ==================== 4. EVENTS (8 Blocks) ====================
  {
    id: 'events-when-start',
    name: 'when start:',
    category: 'events',
    shape: 'Hat',
    opcode: 'event_whenflagclicked',
    syntax: 'when start:\n    <statements>',
    returnType: 'Void',
    description: 'Triggered when the green flag is clicked or when the project starts execution.',
    parameters: [],
    example: 'when start:\n    teleport("Player", 0, 0)\n    say("Player", "Ready!")',
    lspSnippet: 'when start:\n    ${1:teleport("Player", 0, 0)}',
    notes: 'First lifecycle event dispatched on runtime boot.'
  },
  {
    id: 'events-when-action-press',
    name: 'when action.press:',
    category: 'events',
    shape: 'Hat',
    opcode: 'event_whenkeypressed',
    syntax: 'when action.press("key"):\n    <statements>',
    returnType: 'Void',
    description: 'Triggered once when a physical keyboard key or input action is pressed down (edge-triggered).',
    parameters: [
      { name: 'key', type: 'String', description: '"space", "up", "down", "left", "right", or letter/digit', required: true }
    ],
    example: 'when action.press("space"):\n    change_y("Player", 50)',
    lspSnippet: 'when action.press("${1:space}"):\n    ${2:pass}',
    notes: 'Keys: space, up, down, left, right, any, a-z, 0-9.'
  },
  {
    id: 'events-when-click',
    name: 'when click:',
    category: 'events',
    shape: 'Hat',
    opcode: 'event_whenthisspriteclicked',
    syntax: 'when click(target):\n    <statements>',
    returnType: 'Void',
    description: 'Triggered when the mouse clicks within the bounding box / collider of the target sprite.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name or "stage"', required: true }
    ],
    example: 'when click("PlayButton"):\n    broadcast("start_game")',
    lspSnippet: 'when click("${1:Player}"):\n    ${2:pass}',
    notes: 'Performs AABB point intersection with mouse_pos.'
  },
  {
    id: 'events-when-scene-switched',
    name: 'when scene.switched:',
    category: 'events',
    shape: 'Hat',
    opcode: 'event_whenbackdropswitchesto',
    syntax: 'when scene.switched("backdrop_name"):\n    <statements>',
    returnType: 'Void',
    description: 'Triggered when the stage backdrop switches to the specified backdrop scene.',
    parameters: [
      { name: 'backdrop_name', type: 'String', description: 'Target backdrop name', required: true }
    ],
    example: 'when scene.switched("game_over"):\n    hide("Player")',
    lspSnippet: 'when scene.switched("${1:backdrop1}"):\n    ${2:pass}',
    notes: 'Dispatched on switch_backdrop calls.'
  },
  {
    id: 'events-when-greater-than',
    name: 'when loudness >',
    category: 'events',
    shape: 'Hat',
    opcode: 'event_whengreaterthan',
    syntax: 'when loudness > threshold:\n    <statements>',
    returnType: 'Void',
    description: 'Triggered when audio loudness or timer exceeds the threshold value.',
    parameters: [
      { name: 'threshold', type: 'Number', description: 'Threshold value', required: true }
    ],
    example: 'when loudness > 50:\n    say("Cat", "Too loud!")',
    lspSnippet: 'when loudness > ${1:50}:\n    ${2:pass}',
    notes: 'Polled continuously during runtime ticks.'
  },
  {
    id: 'events-when-message',
    name: 'when message:',
    category: 'events',
    shape: 'Hat',
    opcode: 'event_whenbroadcastreceived',
    syntax: 'when message("event_name"):\n    <statements>',
    returnType: 'Void',
    description: 'Subscribes to global broadcast messages sent across sprites.',
    parameters: [
      { name: 'event_name', type: 'String', description: 'Broadcast message identifier', required: true }
    ],
    example: 'when message("game_over"):\n    stop_all()',
    lspSnippet: 'when message("${1:msg}"):\n    ${2:pass}',
    notes: 'Spawns coroutine fibers for all matching subscribers.'
  },
  {
    id: 'events-broadcast',
    name: 'broadcast',
    category: 'events',
    shape: 'Stack',
    opcode: 'event_broadcast',
    syntax: 'broadcast(message_name)',
    returnType: 'Void',
    description: 'Sends a broadcast signal to all subscribing sprites asynchronously without waiting.',
    parameters: [
      { name: 'message_name', type: 'String', description: 'Broadcast message string', required: true }
    ],
    example: 'broadcast("score_point")',
    lspSnippet: 'broadcast("${1:message1}")',
    notes: 'Enqueues message into world broadcast queue.'
  },
  {
    id: 'events-broadcast-and-wait',
    name: 'broadcast_and_wait',
    category: 'events',
    shape: 'Stack',
    opcode: 'event_broadcastandwait',
    syntax: 'broadcast_and_wait(message_name)',
    returnType: 'Void',
    description: 'Sends a broadcast signal and pauses current fiber until all receivers finish execution.',
    parameters: [
      { name: 'message_name', type: 'String', description: 'Broadcast message string', required: true }
    ],
    example: 'broadcast_and_wait("reset_level")',
    lspSnippet: 'broadcast_and_wait("${1:message1}")',
    notes: 'Awaits completion of receiver coroutines.'
  },

  // ==================== 5. CONTROL (11 Blocks) ====================
  {
    id: 'control-wait',
    name: 'wait',
    category: 'control',
    shape: 'Stack',
    opcode: 'control_wait',
    syntax: 'wait(seconds)',
    returnType: 'Void',
    description: 'Suspends execution of the current script fiber for the specified number of seconds.',
    parameters: [
      { name: 'seconds', type: 'Number', description: 'Duration in seconds', required: true }
    ],
    example: 'wait(0.5)',
    lspSnippet: 'wait(${1:1})',
    notes: 'Yields thread cleanly without halting engine loop.'
  },
  {
    id: 'control-repeat',
    name: 'repeat',
    category: 'control',
    shape: 'C-Block',
    opcode: 'control_repeat',
    syntax: 'repeat count:\n    <statements>',
    returnType: 'Void',
    description: 'Executes the enclosed block of code a fixed number of times.',
    parameters: [
      { name: 'count', type: 'Number', description: 'Number of repetitions', required: true }
    ],
    example: 'repeat 10:\n    move("Player", 5)\n    wait(0.05)',
    lspSnippet: 'repeat ${1:10}:\n    ${2:pass}',
    notes: 'Compiles to bounded loop block in IR.'
  },
  {
    id: 'control-update-forever',
    name: 'when update / forever',
    category: 'control',
    shape: 'C-Block',
    opcode: 'control_forever',
    syntax: 'when update:\n    <statements>',
    returnType: 'Void',
    description: 'Runs repeatedly on every simulation tick (60 FPS game loop).',
    parameters: [],
    example: 'when update:\n    if key_pressed("right"):\n        change_x("Player", 5)',
    lspSnippet: 'when update:\n    ${1:pass}',
    notes: 'Idiomatic scratch-lang syntax for Scratch\'s forever block.'
  },
  {
    id: 'control-if',
    name: 'if',
    category: 'control',
    shape: 'C-Block',
    opcode: 'control_if',
    syntax: 'if condition:\n    <statements>',
    returnType: 'Void',
    description: 'Conditionally executes the enclosed block if the condition evaluates to true.',
    parameters: [
      { name: 'condition', type: 'Boolean', description: 'Boolean conditional expression', required: true }
    ],
    example: 'if touching("Player", "Lava"):\n    say("Player", "Ouch!")',
    lspSnippet: 'if ${1:condition}:\n    ${2:pass}',
    notes: 'Supports complex boolean operators and / or / not.'
  },
  {
    id: 'control-if-else',
    name: 'if ... else',
    category: 'control',
    shape: 'C-Block',
    opcode: 'control_if_else',
    syntax: 'if condition:\n    <then_statements>\nelse:\n    <else_statements>',
    returnType: 'Void',
    description: 'Executes one block if condition is true, or alternate block if false.',
    parameters: [
      { name: 'condition', type: 'Boolean', description: 'Conditional expression', required: true }
    ],
    example: 'if score >= 100:\n    say("Player", "Winner!")\nelse:\n    say("Player", "Keep trying!")',
    lspSnippet: 'if ${1:condition}:\n    ${2:pass}\nelse:\n    ${3:pass}',
    notes: 'Indentation indicates branch nesting.'
  },
  {
    id: 'control-wait-until',
    name: 'wait_until',
    category: 'control',
    shape: 'Stack',
    opcode: 'control_wait_until',
    syntax: 'wait_until(condition)',
    returnType: 'Void',
    description: 'Pauses execution of the fiber until the given boolean expression evaluates to true.',
    parameters: [
      { name: 'condition', type: 'Boolean', description: 'Condition to await', required: true }
    ],
    example: 'wait_until(key_pressed("space"))',
    lspSnippet: 'wait_until(${1:key_pressed("space")})',
    notes: 'Re-evaluated each tick before resuming fiber.'
  },
  {
    id: 'control-while-not',
    name: 'while not / repeat until',
    category: 'control',
    shape: 'C-Block',
    opcode: 'control_repeat_until',
    syntax: 'while not condition:\n    <statements>',
    returnType: 'Void',
    description: 'Repeats the enclosed block until the condition becomes true (loops while false).',
    parameters: [
      { name: 'condition', type: 'Boolean', description: 'Exit condition', required: true }
    ],
    example: 'while not touching("Player", "Goal"):\n    move("Player", 2)',
    lspSnippet: 'while not ${1:condition}:\n    ${2:pass}',
    notes: 'Equates to Scratch repeat until block.'
  },
  {
    id: 'control-stop-all',
    name: 'stop_all',
    category: 'control',
    shape: 'Cap',
    opcode: 'control_stop',
    syntax: 'stop_all()',
    returnType: 'Cap',
    description: 'Stops all running fibers, audio playback, and simulation updates across the entire project.',
    parameters: [],
    example: 'when start:\n    if lives <= 0:\n        stop_all()',
    lspSnippet: 'stop_all()',
    notes: 'Terminates all active coroutines.'
  },
  {
    id: 'control-when-clone',
    name: 'when clone:',
    category: 'control',
    shape: 'Hat',
    opcode: 'control_start_as_clone',
    syntax: 'when clone(target):\n    <statements>',
    returnType: 'Void',
    description: 'Lifecycle hook executed by a newly spawned clone instance.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name being cloned', required: true }
    ],
    example: 'when clone("Bullet"):\n    show("Bullet")\n    repeat 20:\n        move("Bullet", 10)\n    delete_clone("Bullet")',
    lspSnippet: 'when clone("${1:Player}"):\n    ${2:pass}',
    notes: 'Executed in the context of the newly spawned clone.'
  },
  {
    id: 'control-clone',
    name: 'clone',
    category: 'control',
    shape: 'Stack',
    opcode: 'control_create_clone_of',
    syntax: 'clone(target)',
    returnType: 'Void',
    description: 'Creates a duplicate clone of the target sprite with copied position, scale, and costume.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name or "myself"', required: true }
    ],
    example: 'clone("Coin")',
    lspSnippet: 'clone("${1:myself}")',
    notes: 'Spawns an independent entity instance in the world.'
  },
  {
    id: 'control-delete-clone',
    name: 'delete_clone',
    category: 'control',
    shape: 'Cap',
    opcode: 'control_delete_this_clone',
    syntax: 'delete_clone(target)',
    returnType: 'Cap',
    description: 'Destroys the current clone instance and frees its resources from the simulation world.',
    parameters: [
      { name: 'target', type: 'String', description: 'Clone entity name', required: true }
    ],
    example: 'delete_clone("Coin")',
    lspSnippet: 'delete_clone("${1:myself}")',
    notes: 'Removes the entity from the active world entities list.'
  },

  // ==================== 6. SENSING (18 Blocks) ====================
  {
    id: 'sensing-touching',
    name: 'touching',
    category: 'sensing',
    shape: 'Boolean',
    opcode: 'sensing_touchingobject',
    syntax: 'touching(target, other)',
    returnType: 'Boolean',
    description: 'Checks if target sprite is colliding with another sprite, "mouse", or stage "edge".',
    parameters: [
      { name: 'target', type: 'String', description: 'Source sprite name', required: true },
      { name: 'other', type: 'String', description: '"mouse", "edge", or sprite name', required: true }
    ],
    example: 'if touching("Player", "Enemy"):\n    say("Player", "Hit!")',
    lspSnippet: 'touching("${1:Player}", "${2|mouse,edge,Sprite1|}")',
    notes: 'Uses SAT and AABB intersection algorithms.'
  },
  {
    id: 'sensing-touching-color',
    name: 'touching_color',
    category: 'sensing',
    shape: 'Boolean',
    opcode: 'sensing_touchingcolor',
    syntax: 'touching_color(target, hex_color)',
    returnType: 'Boolean',
    description: 'Checks if the target sprite intersects with a specific hex color on stage.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'hex_color', type: 'String', description: 'Hex color string (e.g. "#FF0000")', required: true }
    ],
    example: 'if touching_color("Player", "#00FF00"):\n    say("Player", "On Grass")',
    lspSnippet: 'touching_color("${1:Player}", "${2:#ff0000}")',
    notes: 'Evaluates pixel boundary color test.'
  },
  {
    id: 'sensing-color-touching-color',
    name: 'color_touching_color',
    category: 'sensing',
    shape: 'Boolean',
    opcode: 'sensing_coloristouchingcolor',
    syntax: 'color_touching_color(color1, color2)',
    returnType: 'Boolean',
    description: 'Checks if a sprite pixel of color1 is touching color2 on the backdrop or another sprite.',
    parameters: [
      { name: 'color1', type: 'String', description: 'First hex color', required: true },
      { name: 'color2', type: 'String', description: 'Second hex color', required: true }
    ],
    example: 'if color_touching_color("#000000", "#FFFFFF"):\n    bounce()',
    lspSnippet: 'color_touching_color("${1:#000000}", "${2:#ffffff}")',
    notes: 'Advanced collision detection used in platformers.'
  },
  {
    id: 'sensing-distance-to',
    name: 'distance_to',
    category: 'sensing',
    shape: 'Reporter',
    opcode: 'sensing_distanceto',
    syntax: 'distance_to(target, other)',
    returnType: 'Number',
    description: 'Calculates the Euclidean distance in pixels between the target sprite and other sprite or "mouse".',
    parameters: [
      { name: 'target', type: 'String', description: 'Source sprite name', required: true },
      { name: 'other', type: 'String', description: '"mouse" or target sprite name', required: true }
    ],
    example: 'if distance_to("Player", "Enemy") < 50:\n    say("Enemy", "I see you!")',
    lspSnippet: 'distance_to("${1:Player}", "${2:mouse}")',
    notes: 'Returns sqrt(dx^2 + dy^2).'
  },
  {
    id: 'sensing-ask',
    name: 'ask',
    category: 'sensing',
    shape: 'Stack',
    opcode: 'sensing_askandwait',
    syntax: 'ask(question)',
    returnType: 'Void',
    description: 'Displays a question prompt input modal on screen and waits for user text submission.',
    parameters: [
      { name: 'question', type: 'String', description: 'Prompt text question', required: true }
    ],
    example: 'ask("What is your name?")\nsay("Cat", text.join("Hello, ", get_answer()))',
    lspSnippet: 'ask("${1:What\'s your name?}")',
    notes: 'Result is retrievable via get_answer().'
  },
  {
    id: 'sensing-get-answer',
    name: 'get_answer',
    category: 'sensing',
    shape: 'Reporter',
    opcode: 'sensing_answer',
    syntax: 'get_answer()',
    returnType: 'String',
    description: 'Returns the user\'s response from the last completed ask() modal prompt.',
    parameters: [],
    example: 'let name = get_answer()',
    lspSnippet: 'get_answer()',
    notes: 'Stored in world variable __answer.'
  },
  {
    id: 'sensing-key-pressed',
    name: 'key_pressed',
    category: 'sensing',
    shape: 'Boolean',
    opcode: 'sensing_keypressed',
    syntax: 'key_pressed(key_name)',
    returnType: 'Boolean',
    description: 'Continuously checks if a keyboard key is currently being held down.',
    parameters: [
      { name: 'key_name', type: 'String', description: '"space", "up", "down", "left", "right", "any", "a"-"z", "0"-"9"', required: true }
    ],
    example: 'if key_pressed("up"):\n    change_y("Player", 5)',
    lspSnippet: 'key_pressed("${1:space}")',
    notes: 'Queried from world input state tables.'
  },
  {
    id: 'sensing-mouse-down',
    name: 'mouse_down',
    category: 'sensing',
    shape: 'Boolean',
    opcode: 'sensing_mousedown',
    syntax: 'mouse_down()',
    returnType: 'Boolean',
    description: 'Returns true if the primary left mouse button is currently pressed down.',
    parameters: [],
    example: 'if mouse_down():\n    teleport("Crosshair", mouse_x(), mouse_y())',
    lspSnippet: 'mouse_down()',
    notes: 'Continuous polling boolean reporter.'
  },
  {
    id: 'sensing-mouse-x',
    name: 'mouse_x',
    category: 'sensing',
    shape: 'Reporter',
    opcode: 'sensing_mousex',
    syntax: 'mouse_x()',
    returnType: 'Number',
    description: 'Returns current stage horizontal coordinate x of the mouse cursor (-240 to 240).',
    parameters: [],
    example: 'let mx = mouse_x()',
    lspSnippet: 'mouse_x()',
    notes: 'Stage coordinates centered at origin (0, 0).'
  },
  {
    id: 'sensing-mouse-y',
    name: 'mouse_y',
    category: 'sensing',
    shape: 'Reporter',
    opcode: 'sensing_mousey',
    syntax: 'mouse_y()',
    returnType: 'Number',
    description: 'Returns current stage vertical coordinate y of the mouse cursor (-180 to 180).',
    parameters: [],
    example: 'let my = mouse_y()',
    lspSnippet: 'mouse_y()',
    notes: 'Stage coordinates centered at origin (0, 0).'
  },
  {
    id: 'sensing-set-drag-mode',
    name: 'set_drag_mode',
    category: 'sensing',
    shape: 'Stack',
    opcode: 'sensing_setdragmode',
    syntax: 'set_drag_mode(target, mode)',
    returnType: 'Void',
    description: 'Sets whether a sprite can be dragged by the user in the stage canvas.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true },
      { name: 'mode', type: 'String', description: '"draggable" | "not draggable"', required: true }
    ],
    example: 'set_drag_mode("Piece", "draggable")',
    lspSnippet: 'set_drag_mode("${1:Player}", "${2|draggable,not draggable|}")',
    notes: 'Sets entity.draggable flag.'
  },
  {
    id: 'sensing-loudness',
    name: 'loudness / get_loudness',
    category: 'sensing',
    shape: 'Reporter',
    opcode: 'sensing_loudness',
    syntax: 'get_loudness()',
    returnType: 'Number',
    description: 'Returns microphone audio loudness level on a scale from 0 to 100.',
    parameters: [],
    example: 'if get_loudness() > 40:\n    say("Cat", "Loud sound!")',
    lspSnippet: 'get_loudness()',
    notes: 'Returns 0.0 if microphone is inactive.'
  },
  {
    id: 'sensing-get-timer',
    name: 'get_timer',
    category: 'sensing',
    shape: 'Reporter',
    opcode: 'sensing_timer',
    syntax: 'get_timer()',
    returnType: 'Number',
    description: 'Returns elapsed time in seconds with millisecond precision since simulation started or timer was reset.',
    parameters: [],
    example: 'let time_elapsed = get_timer()',
    lspSnippet: 'get_timer()',
    notes: 'High-precision monotonic clock query.'
  },
  {
    id: 'sensing-reset-timer',
    name: 'reset_timer',
    category: 'sensing',
    shape: 'Stack',
    opcode: 'sensing_resettimer',
    syntax: 'reset_timer()',
    returnType: 'Void',
    description: 'Resets the simulation timer baseline to 0.0 seconds.',
    parameters: [],
    example: 'when start:\n    reset_timer()',
    lspSnippet: 'reset_timer()',
    notes: 'Resets world timer start offset.'
  },
  {
    id: 'sensing-property-of',
    name: 'property_of',
    category: 'sensing',
    shape: 'Reporter',
    opcode: 'sensing_of',
    syntax: 'property_of(target, property_name)',
    returnType: 'Any',
    description: 'Introspects a property ("x position", "y position", "direction", "costume number", "costume name", "size", "volume") or variable of an entity.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name or "Stage"', required: true },
      { name: 'property_name', type: 'String', description: 'Property or variable identifier', required: true }
    ],
    example: 'let enemy_x = property_of("Boss", "x position")',
    lspSnippet: 'property_of("${1:Sprite1}", "${2|x position,y position,direction,costume number,costume name,size,volume|}")',
    notes: 'Equates to Scratch ([property] of [Sprite]) sensing block.'
  },
  {
    id: 'sensing-current',
    name: 'current',
    category: 'sensing',
    shape: 'Reporter',
    opcode: 'sensing_current',
    syntax: 'current(unit)',
    returnType: 'Number',
    description: 'Returns system clock value for "year", "month", "date", "dayofweek", "hour", "minute", or "second".',
    parameters: [
      { name: 'unit', type: 'String', description: '"year" | "month" | "date" | "dayofweek" | "hour" | "minute" | "second"', required: true }
    ],
    example: 'say("Cat", text.join("Current Hour: ", current("hour")))',
    lspSnippet: 'current("${1|year,month,date,dayofweek,hour,minute,second|}")',
    notes: 'Queried from host system local time.'
  },
  {
    id: 'sensing-days-since-2000',
    name: 'days_since_2000',
    category: 'sensing',
    shape: 'Reporter',
    opcode: 'sensing_dayssince2000',
    syntax: 'days_since_2000()',
    returnType: 'Number',
    description: 'Returns high-precision floating point days elapsed since Jan 1, 2000 00:00:00 UTC.',
    parameters: [],
    example: 'let days = days_since_2000()',
    lspSnippet: 'days_since_2000()',
    notes: 'Scratch standard astronomical clock offset calculation.'
  },
  {
    id: 'sensing-get-username',
    name: 'get_username',
    category: 'sensing',
    shape: 'Reporter',
    opcode: 'sensing_username',
    syntax: 'get_username()',
    returnType: 'String',
    description: 'Returns the active system OS username or logged-in profile name.',
    parameters: [],
    example: 'say("Cat", text.join("Welcome, ", get_username()))',
    lspSnippet: 'get_username()',
    notes: 'Resolves from USERNAME / USER environment or profile config.'
  },

  // ==================== 7. OPERATORS & MATH (18 Blocks) ====================
  {
    id: 'op-add',
    name: '+ (Addition)',
    category: 'operators',
    shape: 'Reporter',
    opcode: 'operator_add',
    syntax: 'a + b',
    returnType: 'Number',
    description: 'Calculates the sum of two numbers (or string concatenation fallback).',
    parameters: [
      { name: 'a', type: 'Number', description: 'First operand', required: true },
      { name: 'b', type: 'Number', description: 'Second operand', required: true }
    ],
    example: 'let sum = 10 + 25',
    lspSnippet: '${1:a} + ${2:b}',
    notes: 'IEEE-754 64-bit float math.'
  },
  {
    id: 'op-sub',
    name: '- (Subtraction)',
    category: 'operators',
    shape: 'Reporter',
    opcode: 'operator_subtract',
    syntax: 'a - b',
    returnType: 'Number',
    description: 'Subtracts operand b from operand a.',
    parameters: [
      { name: 'a', type: 'Number', description: 'Minuend', required: true },
      { name: 'b', type: 'Number', description: 'Subtrahend', required: true }
    ],
    example: 'let diff = 100 - 35',
    lspSnippet: '${1:a} - ${2:b}',
    notes: 'IEEE-754 64-bit float math.'
  },
  {
    id: 'op-mul',
    name: '* (Multiplication)',
    category: 'operators',
    shape: 'Reporter',
    opcode: 'operator_multiply',
    syntax: 'a * b',
    returnType: 'Number',
    description: 'Multiplies two numbers together.',
    parameters: [
      { name: 'a', type: 'Number', description: 'Multiplier', required: true },
      { name: 'b', type: 'Number', description: 'Multiplicand', required: true }
    ],
    example: 'let area = width * height',
    lspSnippet: '${1:a} * ${2:b}',
    notes: 'IEEE-754 64-bit float math.'
  },
  {
    id: 'op-div',
    name: '/ (Division)',
    category: 'operators',
    shape: 'Reporter',
    opcode: 'operator_divide',
    syntax: 'a / b',
    returnType: 'Number',
    description: 'Divides operand a by operand b. Division by zero returns 0.0 per Scratch specification.',
    parameters: [
      { name: 'a', type: 'Number', description: 'Numerator', required: true },
      { name: 'b', type: 'Number', description: 'Denominator', required: true }
    ],
    example: 'let speed = distance / time',
    lspSnippet: '${1:a} / ${2:b}',
    notes: 'Safe division by zero guard built-in.'
  },
  {
    id: 'op-random',
    name: 'random / pick random',
    category: 'operators',
    shape: 'Reporter',
    opcode: 'operator_random',
    syntax: 'random(min, max)',
    returnType: 'Number',
    description: 'Returns a random integer (if both inputs are integers) or random float within [min, max].',
    parameters: [
      { name: 'min', type: 'Number', description: 'Lower bound', required: true },
      { name: 'max', type: 'Number', description: 'Upper bound', required: true }
    ],
    example: 'let dice = random(1, 6)',
    lspSnippet: 'random(${1:1}, ${2:10})',
    notes: 'Uniform distribution PRNG.'
  },
  {
    id: 'op-gt',
    name: '> (Greater Than)',
    category: 'operators',
    shape: 'Boolean',
    opcode: 'operator_gt',
    syntax: 'a > b',
    returnType: 'Boolean',
    description: 'Returns true if operand a is strictly greater than operand b (numeric or alphabetical).',
    parameters: [
      { name: 'a', type: 'Any', description: 'Left operand', required: true },
      { name: 'b', type: 'Any', description: 'Right operand', required: true }
    ],
    example: 'if score > high_score:\n    high_score = score',
    lspSnippet: '${1:a} > ${2:b}',
    notes: 'Case-insensitive string comparison if operands are text.'
  },
  {
    id: 'op-lt',
    name: '< (Less Than)',
    category: 'operators',
    shape: 'Boolean',
    opcode: 'operator_lt',
    syntax: 'a < b',
    returnType: 'Boolean',
    description: 'Returns true if operand a is strictly less than operand b.',
    parameters: [
      { name: 'a', type: 'Any', description: 'Left operand', required: true },
      { name: 'b', type: 'Any', description: 'Right operand', required: true }
    ],
    example: 'if lives < 1:\n    stop_all()',
    lspSnippet: '${1:a} < ${2:b}',
    notes: 'Case-insensitive string comparison if operands are text.'
  },
  {
    id: 'op-eq',
    name: '== (Equals)',
    category: 'operators',
    shape: 'Boolean',
    opcode: 'operator_equals',
    syntax: 'a == b',
    returnType: 'Boolean',
    description: 'Returns true if both operands are equivalent (numbers, booleans, or case-insensitive strings).',
    parameters: [
      { name: 'a', type: 'Any', description: 'Left operand', required: true },
      { name: 'b', type: 'Any', description: 'Right operand', required: true }
    ],
    example: 'if get_answer() == "Secret":\n    say("Door", "Unlocked")',
    lspSnippet: '${1:a} == ${2:b}',
    notes: 'Scratch-compatible loose and case-insensitive comparison.'
  },
  {
    id: 'op-and',
    name: 'and (Conjunction)',
    category: 'operators',
    shape: 'Boolean',
    opcode: 'operator_and',
    syntax: 'a and b',
    returnType: 'Boolean',
    description: 'Short-circuiting logical AND: returns true if both expressions evaluate to true.',
    parameters: [
      { name: 'a', type: 'Boolean', description: 'First condition', required: true },
      { name: 'b', type: 'Boolean', description: 'Second condition', required: true }
    ],
    example: 'if key_pressed("up") and on_ground:\n    jump()',
    lspSnippet: '${1:cond1} and ${2:cond2}',
    notes: 'Short-circuits if left expression is false.'
  },
  {
    id: 'op-or',
    name: 'or (Disjunction)',
    category: 'operators',
    shape: 'Boolean',
    opcode: 'operator_or',
    syntax: 'a or b',
    returnType: 'Boolean',
    description: 'Short-circuiting logical OR: returns true if at least one expression evaluates to true.',
    parameters: [
      { name: 'a', type: 'Boolean', description: 'First condition', required: true },
      { name: 'b', type: 'Boolean', description: 'Second condition', required: true }
    ],
    example: 'if touching("Player", "Wall") or touching("Player", "Door"):\n    stop()',
    lspSnippet: '${1:cond1} or ${2:cond2}',
    notes: 'Short-circuits if left expression is true.'
  },
  {
    id: 'op-not',
    name: 'not (Negation)',
    category: 'operators',
    shape: 'Boolean',
    opcode: 'operator_not',
    syntax: 'not condition',
    returnType: 'Boolean',
    description: 'Logical inversion: returns true if condition is false, and false if condition is true.',
    parameters: [
      { name: 'condition', type: 'Boolean', description: 'Condition to negate', required: true }
    ],
    example: 'if not touching("Player", "Lava"):\n    move("Player", 5)',
    lspSnippet: 'not ${1:condition}',
    notes: 'Inverts truth value.'
  },
  {
    id: 'op-join',
    name: 'text.join / join',
    category: 'operators',
    shape: 'Reporter',
    opcode: 'operator_join',
    syntax: 'text.join(str1, str2)',
    returnType: 'String',
    description: 'Concatenates two string or numeric values together.',
    parameters: [
      { name: 'str1', type: 'Any', description: 'First string/value', required: true },
      { name: 'str2', type: 'Any', description: 'Second string/value', required: true }
    ],
    example: 'let greeting = text.join("Score: ", score)',
    lspSnippet: 'text.join("${1:apple}", "${2:banana}")',
    notes: 'Auto-converts numbers to string representation.'
  },
  {
    id: 'op-letter-at',
    name: 'text.letter_at',
    category: 'operators',
    shape: 'Reporter',
    opcode: 'operator_letter_of',
    syntax: 'text.letter_at(text, index)',
    returnType: 'String',
    description: 'Returns the single character at the 1-based index of the string (UTF-8 safe).',
    parameters: [
      { name: 'text', type: 'String', description: 'Source text', required: true },
      { name: 'index', type: 'Number', description: '1-based character position', required: true }
    ],
    example: 'let first_char = text.letter_at("Scratch", 1) # "S"',
    lspSnippet: 'text.letter_at("${1:apple}", ${2:1})',
    notes: 'Returns empty string if index is out of bounds.'
  },
  {
    id: 'op-length',
    name: 'text.length',
    category: 'operators',
    shape: 'Reporter',
    opcode: 'operator_length',
    syntax: 'text.length(text)',
    returnType: 'Number',
    description: 'Returns the total number of characters in the string.',
    parameters: [
      { name: 'text', type: 'String', description: 'Source string', required: true }
    ],
    example: 'let count = text.length("Player")',
    lspSnippet: 'text.length("${1:apple}")',
    notes: 'Counts Unicode codepoints.'
  },
  {
    id: 'op-contains',
    name: 'text.contains',
    category: 'operators',
    shape: 'Boolean',
    opcode: 'operator_contains',
    syntax: 'text.contains(source, substring)',
    returnType: 'Boolean',
    description: 'Performs case-insensitive substring search returning true if substring is found.',
    parameters: [
      { name: 'source', type: 'String', description: 'String to search within', required: true },
      { name: 'substring', type: 'String', description: 'Substring to look for', required: true }
    ],
    example: 'if text.contains(get_answer(), "yes"):\n    start_quest()',
    lspSnippet: 'text.contains("${1:apple}", "${2:a}")',
    notes: 'Case-insensitive per Scratch specification.'
  },
  {
    id: 'op-mod',
    name: 'math.mod / %',
    category: 'operators',
    shape: 'Reporter',
    opcode: 'operator_mod',
    syntax: 'math.mod(n, m) or n % m',
    returnType: 'Number',
    description: 'Calculates mathematical floored modulo: ((a % b) + b) % b, correctly handling negative dividends.',
    parameters: [
      { name: 'n', type: 'Number', description: 'Dividend', required: true },
      { name: 'm', type: 'Number', description: 'Divisor', required: true }
    ],
    example: 'let rem = math.mod(10, 3) # 1',
    lspSnippet: 'math.mod(${1:10}, ${2:3})',
    notes: 'Floored modulo matches Scratch conventions.'
  },
  {
    id: 'op-round',
    name: 'math.round',
    category: 'operators',
    shape: 'Reporter',
    opcode: 'operator_round',
    syntax: 'math.round(n)',
    returnType: 'Number',
    description: 'Rounds number to nearest integer using half-away-from-zero rounding.',
    parameters: [
      { name: 'n', type: 'Number', description: 'Float value', required: true }
    ],
    example: 'let rounded = math.round(3.7) # 4',
    lspSnippet: 'math.round(${1:3.7})',
    notes: 'Half-away-from-zero rounding.'
  },
  {
    id: 'op-mathop',
    name: 'math.<function>',
    category: 'operators',
    shape: 'Reporter',
    opcode: 'operator_mathop',
    syntax: 'math.<func>(n)',
    returnType: 'Number',
    description: 'Advanced math functions: math.abs, math.sqrt, math.sin, math.cos, math.tan, math.asin, math.acos, math.atan, math.ln, math.log, math.exp, math.pow10.',
    parameters: [
      { name: 'n', type: 'Number', description: 'Input numeric value', required: true }
    ],
    example: 'let hypotenuse = math.sqrt(dx * dx + dy * dy)\nlet angle = math.atan(dy / dx)',
    lspSnippet: 'math.${1|abs,sqrt,sin,cos,tan,asin,acos,atan,ln,log,exp,pow10|}(${2:0})',
    notes: 'Trigonometric functions expect degrees per Scratch convention.'
  },

  // ==================== 8. VARIABLES & DATA (5 Blocks) ====================
  {
    id: 'var-ref',
    name: '<variable_name>',
    category: 'variables',
    shape: 'Reporter',
    opcode: 'data_variable',
    syntax: 'variable_name',
    returnType: 'Any',
    description: 'Reads the stored value of a declared global or local variable.',
    parameters: [],
    example: 'say("Player", score)',
    lspSnippet: '${1:variable}',
    notes: 'Variables can store Numbers, Strings, Booleans, or Lists.'
  },
  {
    id: 'var-set',
    name: '= (Set Variable)',
    category: 'variables',
    shape: 'Stack',
    opcode: 'data_setvariableto',
    syntax: 'variable = value',
    returnType: 'Void',
    description: 'Assigns a new value to a variable.',
    parameters: [
      { name: 'variable', type: 'Identifier', description: 'Variable name', required: true },
      { name: 'value', type: 'Any', description: 'Value expression', required: true }
    ],
    example: 'score = 0\nplayer_name = "Sonic"',
    lspSnippet: '${1:var} = ${2:0}',
    notes: 'Creates variable entry if not already present.'
  },
  {
    id: 'var-change',
    name: '+= (Change Variable By)',
    category: 'variables',
    shape: 'Stack',
    opcode: 'data_changevariableby',
    syntax: 'variable += delta',
    returnType: 'Void',
    description: 'Adds delta to the current numeric value of the variable.',
    parameters: [
      { name: 'variable', type: 'Identifier', description: 'Variable name', required: true },
      { name: 'delta', type: 'Number', description: 'Numeric increment delta', required: true }
    ],
    example: 'score += 10',
    lspSnippet: '${1:var} += ${2:1}',
    notes: 'Compound assignment shorthand for variable = variable + delta.'
  },
  {
    id: 'var-show',
    name: 'variable.show',
    category: 'variables',
    shape: 'Stack',
    opcode: 'data_showvariable',
    syntax: 'variable.show(name)',
    returnType: 'Void',
    description: 'Shows the variable on-screen monitor HUD on the stage.',
    parameters: [
      { name: 'name', type: 'String', description: 'Variable name', required: true }
    ],
    example: 'variable.show("score")',
    lspSnippet: 'variable.show("${1:score}")',
    notes: 'Enables HUD monitor display in stage.'
  },
  {
    id: 'var-hide',
    name: 'variable.hide',
    category: 'variables',
    shape: 'Stack',
    opcode: 'data_hidevariable',
    syntax: 'variable.hide(name)',
    returnType: 'Void',
    description: 'Hides the variable on-screen monitor HUD from the stage.',
    parameters: [
      { name: 'name', type: 'String', description: 'Variable name', required: true }
    ],
    example: 'variable.hide("secret_seed")',
    lspSnippet: 'variable.hide("${1:score}")',
    notes: 'Disables HUD monitor display in stage.'
  },

  // ==================== 9. LISTS (12 Blocks) ====================
  {
    id: 'list-to-string',
    name: 'list.to_string',
    category: 'lists',
    shape: 'Reporter',
    opcode: 'data_listcontents',
    syntax: 'list.to_string(list_name)',
    returnType: 'String',
    description: 'Returns all list elements formatted into a string (joined with space if single chars, otherwise newline).',
    parameters: [
      { name: 'list_name', type: 'String', description: 'List name', required: true }
    ],
    example: 'let text = list.to_string("inventory")',
    lspSnippet: 'list.to_string("${1:list_name}")',
    notes: 'Follows Scratch 3.0 stringification rules.'
  },
  {
    id: 'list-add',
    name: 'list.add',
    category: 'lists',
    shape: 'Stack',
    opcode: 'data_addtolist',
    syntax: 'list.add(list_name, item)',
    returnType: 'Void',
    description: 'Appends a new element to the end/tail of the specified list.',
    parameters: [
      { name: 'list_name', type: 'String', description: 'List name', required: true },
      { name: 'item', type: 'Any', description: 'Value to add', required: true }
    ],
    example: 'list.add("inventory", "Sword")',
    lspSnippet: 'list.add("${1:list_name}", ${2:item})',
    notes: 'Expands list dynamically.'
  },
  {
    id: 'list-delete',
    name: 'list.delete',
    category: 'lists',
    shape: 'Stack',
    opcode: 'data_deleteoflist',
    syntax: 'list.delete(list_name, index)',
    returnType: 'Void',
    description: 'Deletes the element at the 1-based index or "last" from the list.',
    parameters: [
      { name: 'list_name', type: 'String', description: 'List name', required: true },
      { name: 'index', type: 'Number|String', description: '1-based index or "last"', required: true }
    ],
    example: 'list.delete("inventory", 1)\nlist.delete("inventory", "last")',
    lspSnippet: 'list.delete("${1:list_name}", ${2:1})',
    notes: 'Shifts subsequent elements leftward.'
  },
  {
    id: 'list-clear',
    name: 'list.clear',
    category: 'lists',
    shape: 'Stack',
    opcode: 'data_deletealloflist',
    syntax: 'list.clear(list_name)',
    returnType: 'Void',
    description: 'Removes all elements from the list, resetting its length to 0.',
    parameters: [
      { name: 'list_name', type: 'String', description: 'List name', required: true }
    ],
    example: 'list.clear("scores")',
    lspSnippet: 'list.clear("${1:list_name}")',
    notes: 'Empties the vector buffer.'
  },
  {
    id: 'list-insert',
    name: 'list.insert',
    category: 'lists',
    shape: 'Stack',
    opcode: 'data_insertatlist',
    syntax: 'list.insert(list_name, index, item)',
    returnType: 'Void',
    description: 'Inserts an item at the given 1-based index (shifts existing elements right).',
    parameters: [
      { name: 'list_name', type: 'String', description: 'List name', required: true },
      { name: 'index', type: 'Number', description: '1-based insertion position', required: true },
      { name: 'item', type: 'Any', description: 'Item to insert', required: true }
    ],
    example: 'list.insert("highscores", 1, 9999)',
    lspSnippet: 'list.insert("${1:list_name}", ${2:1}, ${3:item})',
    notes: 'Supports inserting between 1 and list.length + 1.'
  },
  {
    id: 'list-replace',
    name: 'list.replace',
    category: 'lists',
    shape: 'Stack',
    opcode: 'data_replaceitemoflist',
    syntax: 'list.replace(list_name, index, item)',
    returnType: 'Void',
    description: 'Replaces the item at the 1-based index with a new value.',
    parameters: [
      { name: 'list_name', type: 'String', description: 'List name', required: true },
      { name: 'index', type: 'Number', description: '1-based index position', required: true },
      { name: 'item', type: 'Any', description: 'New replacement value', required: true }
    ],
    example: 'list.replace("inventory", 1, "Shield")',
    lspSnippet: 'list.replace("${1:list_name}", ${2:1}, ${3:item})',
    notes: 'Overwrites existing slot.'
  },
  {
    id: 'list-item',
    name: 'list.item',
    category: 'lists',
    shape: 'Reporter',
    opcode: 'data_itemoflist',
    syntax: 'list.item(list_name, index)',
    returnType: 'Any',
    description: 'Retrieves the element at 1-based index, "last", or "random" from the list.',
    parameters: [
      { name: 'list_name', type: 'String', description: 'List name', required: true },
      { name: 'index', type: 'Number|String', description: '1-based index, "last", or "random"', required: true }
    ],
    example: 'let weapon = list.item("inventory", 1)\nlet random_loot = list.item("loot_table", "random")',
    lspSnippet: 'list.item("${1:list_name}", ${2:1})',
    notes: 'Returns Nil if index is out of bounds.'
  },
  {
    id: 'list-index-of',
    name: 'list.index_of',
    category: 'lists',
    shape: 'Reporter',
    opcode: 'data_itemnumoflist',
    syntax: 'list.index_of(list_name, item)',
    returnType: 'Number',
    description: 'Finds the 1-based index of the first occurrence of item in list. Returns 0 if not found.',
    parameters: [
      { name: 'list_name', type: 'String', description: 'List name', required: true },
      { name: 'item', type: 'Any', description: 'Item to search for', required: true }
    ],
    example: 'let idx = list.index_of("inventory", "Potion")\nif idx > 0:\n    say("Player", "Found potion!")',
    lspSnippet: 'list.index_of("${1:list_name}", ${2:item})',
    notes: 'Returns 0 when missing per Scratch convention.'
  },
  {
    id: 'list-length',
    name: 'list.length',
    category: 'lists',
    shape: 'Reporter',
    opcode: 'data_lengthoflist',
    syntax: 'list.length(list_name)',
    returnType: 'Number',
    description: 'Returns the total number of elements in the list.',
    parameters: [
      { name: 'list_name', type: 'String', description: 'List name', required: true }
    ],
    example: 'let total_items = list.length("inventory")',
    lspSnippet: 'list.length("${1:list_name}")',
    notes: 'Returns 0 for empty lists.'
  },
  {
    id: 'list-contains',
    name: 'list.contains',
    category: 'lists',
    shape: 'Boolean',
    opcode: 'data_listcontainsitem',
    syntax: 'list.contains(list_name, item)',
    returnType: 'Boolean',
    description: 'Returns true if the list contains the specified item.',
    parameters: [
      { name: 'list_name', type: 'String', description: 'List name', required: true },
      { name: 'item', type: 'Any', description: 'Item to test', required: true }
    ],
    example: 'if list.contains("inventory", "Key"):\n    say("Door", "Unlocked!")',
    lspSnippet: 'list.contains("${1:list_name}", ${2:item})',
    notes: 'Case-insensitive string comparisons.'
  },
  {
    id: 'list-show',
    name: 'list.show',
    category: 'lists',
    shape: 'Stack',
    opcode: 'data_showlist',
    syntax: 'list.show(list_name)',
    returnType: 'Void',
    description: 'Shows the on-screen scrollable table monitor for the list on stage.',
    parameters: [
      { name: 'list_name', type: 'String', description: 'List name', required: true }
    ],
    example: 'list.show("leaderboard")',
    lspSnippet: 'list.show("${1:list_name}")',
    notes: 'Enables list table HUD overlay.'
  },
  {
    id: 'list-hide',
    name: 'list.hide',
    category: 'lists',
    shape: 'Stack',
    opcode: 'data_hidelist',
    syntax: 'list.hide(list_name)',
    returnType: 'Void',
    description: 'Hides the on-screen scrollable table monitor for the list from the stage.',
    parameters: [
      { name: 'list_name', type: 'String', description: 'List name', required: true }
    ],
    example: 'list.hide("leaderboard")',
    lspSnippet: 'list.hide("${1:list_name}")',
    notes: 'Disables list table HUD overlay.'
  },

  // ==================== 10. PROCEDURES (4 Blocks) ====================
  {
    id: 'proc-def',
    name: 'fn <name>(args): (Define Procedure)',
    category: 'procedures',
    shape: 'Hat',
    opcode: 'procedures_definition',
    syntax: 'fn function_name(arg1, arg2):\n    <statements>',
    returnType: 'Void',
    description: 'Defines a custom procedure / My Block with scalar or boolean arguments.',
    parameters: [
      { name: 'function_name', type: 'Identifier', description: 'Procedure name', required: true },
      { name: 'args', type: 'Parameters', description: 'Comma-separated parameter list', required: false }
    ],
    example: 'fn jump(power):\n    change_y("Player", power)\n    wait(0.2)\n    change_y("Player", -power)',
    lspSnippet: 'fn ${1:name}(${2:args}):\n    ${3:pass}',
    notes: 'Compiles into callable function symbol.'
  },
  {
    id: 'proc-call',
    name: '<name>(args) (Call Procedure)',
    category: 'procedures',
    shape: 'Stack',
    opcode: 'procedures_call',
    syntax: 'function_name(arg1, arg2)',
    returnType: 'Void',
    description: 'Executes a user-defined procedure passing arguments to its stack frame.',
    parameters: [
      { name: 'function_name', type: 'Identifier', description: 'Procedure name', required: true },
      { name: 'args', type: 'Expressions', description: 'Arguments', required: false }
    ],
    example: 'jump(50)',
    lspSnippet: '${1:func}(${2:args})',
    notes: 'Creates local call frame.'
  },
  {
    id: 'proc-arg-reporter',
    name: 'Argument Reporter (Scalar)',
    category: 'procedures',
    shape: 'Reporter',
    opcode: 'argument_reporter_string_number',
    syntax: '<parameter_name>',
    returnType: 'Any',
    description: 'Evaluates the string or numeric argument passed into the active procedure scope.',
    parameters: [],
    example: 'fn draw_square(size):\n    repeat 4:\n        move("Pen", size)\n        turn_right("Pen", 90)',
    lspSnippet: '${1:param}',
    notes: 'Scoped to procedure body.'
  },
  {
    id: 'proc-arg-bool',
    name: 'Argument Reporter (Boolean)',
    category: 'procedures',
    shape: 'Boolean',
    opcode: 'argument_reporter_boolean',
    syntax: '<parameter_name>',
    returnType: 'Boolean',
    description: 'Evaluates a boolean condition argument passed into the procedure scope.',
    parameters: [],
    example: 'fn check_state(is_active):\n    if is_active:\n        say("Bot", "Online")',
    lspSnippet: '${1:is_valid}',
    notes: 'Scoped to procedure body.'
  },

  // ==================== 11. PEN EXTENSION (9 Blocks) ====================
  {
    id: 'pen-clear',
    name: 'pen.clear / erase all',
    category: 'pen',
    shape: 'Stack',
    opcode: 'pen_clear',
    syntax: 'pen.clear()',
    returnType: 'Void',
    description: 'Clears all vector drawing strokes and stamps from the stage canvas buffer.',
    parameters: [],
    example: 'when start:\n    pen.clear()',
    lspSnippet: 'pen.clear()',
    notes: 'Empties world.pen_strokes and world.pen_stamps.'
  },
  {
    id: 'pen-stamp',
    name: 'pen.stamp',
    category: 'pen',
    shape: 'Stack',
    opcode: 'pen_stamp',
    syntax: 'pen.stamp(target)',
    returnType: 'Void',
    description: 'Bakes the current sprite appearance, position, and rotation permanently into the background stamp buffer.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'repeat 12:\n    move("Pen", 20)\n    turn_right("Pen", 30)\n    pen.stamp("Pen")',
    lspSnippet: 'pen.stamp("${1:Player}")',
    notes: 'Renders in GUI preview via painter.rect_filled with entity transforms.'
  },
  {
    id: 'pen-down',
    name: 'pen.down',
    category: 'pen',
    shape: 'Stack',
    opcode: 'pen_penDown',
    syntax: 'pen.down(target)',
    returnType: 'Void',
    description: 'Lowers the drawing pen for the sprite; subsequent movements will automatically trace vector lines on the canvas.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'pen.set_color("#00FF00")\npen.down("Player")\nmove("Player", 100)',
    lspSnippet: 'pen.down("${1:Player}")',
    notes: 'Records PenStroke segments during movement.execute_move.'
  },
  {
    id: 'pen-up',
    name: 'pen.up',
    category: 'pen',
    shape: 'Stack',
    opcode: 'pen_penUp',
    syntax: 'pen.up(target)',
    returnType: 'Void',
    description: 'Lifts the drawing pen so the sprite can move freely without leaving vector trails.',
    parameters: [
      { name: 'target', type: 'String', description: 'Sprite entity name', required: true }
    ],
    example: 'pen.up("Player")\nteleport("Player", 0, 0)',
    lspSnippet: 'pen.up("${1:Player}")',
    notes: 'Removes sprite from active pen down set.'
  },
  {
    id: 'pen-set-color',
    name: 'pen.set_color',
    category: 'pen',
    shape: 'Stack',
    opcode: 'pen_setPenColorToColor',
    syntax: 'pen.set_color(hex_color)',
    returnType: 'Void',
    description: 'Sets the stroke color for drawing lines using a hex color string.',
    parameters: [
      { name: 'hex_color', type: 'String', description: 'Hex color string (e.g. "#4C97FF" or "#FF0000")', required: true }
    ],
    example: 'pen.set_color("#FF5500")',
    lspSnippet: 'pen.set_color("${1:#ff0000}")',
    notes: 'Stores normalized [r, g, b, a] in world.pen_color.'
  },
  {
    id: 'pen-change-param',
    name: 'pen.change_param',
    category: 'pen',
    shape: 'Stack',
    opcode: 'pen_changePenColorParamBy',
    syntax: 'pen.change_param(param, delta)',
    returnType: 'Void',
    description: 'Adjusts a pen color parameter: "color", "saturation", "brightness", or "transparency" by delta.',
    parameters: [
      { name: 'param', type: 'String', description: '"color" | "saturation" | "brightness" | "transparency"', required: true },
      { name: 'delta', type: 'Number', description: 'Parameter delta', required: true }
    ],
    example: 'pen.change_param("color", 10)',
    lspSnippet: 'pen.change_param("${1|color,saturation,brightness,transparency|}", ${2:10})',
    notes: 'Allows rainbow trails when called inside move loops.'
  },
  {
    id: 'pen-set-param',
    name: 'pen.set_param',
    category: 'pen',
    shape: 'Stack',
    opcode: 'pen_setPenColorParamTo',
    syntax: 'pen.set_param(param, value)',
    returnType: 'Void',
    description: 'Directly sets a pen color parameter: "color", "saturation", "brightness", or "transparency".',
    parameters: [
      { name: 'param', type: 'String', description: '"color" | "saturation" | "brightness" | "transparency"', required: true },
      { name: 'value', type: 'Number', description: 'Value (0 to 100)', required: true }
    ],
    example: 'pen.set_param("transparency", 20)',
    lspSnippet: 'pen.set_param("${1|color,saturation,brightness,transparency|}", ${2:50})',
    notes: 'Configures HSV/alpha parameters.'
  },
  {
    id: 'pen-change-size',
    name: 'pen.change_size',
    category: 'pen',
    shape: 'Stack',
    opcode: 'pen_changePenSizeBy',
    syntax: 'pen.change_size(delta)',
    returnType: 'Void',
    description: 'Changes stroke line thickness by delta pixels.',
    parameters: [
      { name: 'delta', type: 'Number', description: 'Thickness delta in pixels', required: true }
    ],
    example: 'pen.change_size(2)',
    lspSnippet: 'pen.change_size(${1:1})',
    notes: 'Increments world.pen_size (clamped min 1.0).'
  },
  {
    id: 'pen-set-size',
    name: 'pen.set_size',
    category: 'pen',
    shape: 'Stack',
    opcode: 'pen_setPenSizeTo',
    syntax: 'pen.set_size(size)',
    returnType: 'Void',
    description: 'Sets absolute stroke line thickness in pixels.',
    parameters: [
      { name: 'size', type: 'Number', description: 'Width in pixels (min 1.0)', required: true }
    ],
    example: 'pen.set_size(4)',
    lspSnippet: 'pen.set_size(${1:2})',
    notes: 'Renders in GUI preview stage_view.rs.'
  },

  // ==================== 12. MUSIC EXTENSION (7 Blocks) ====================
  {
    id: 'music-play-drum',
    name: 'music.play_drum',
    category: 'music',
    shape: 'Stack',
    opcode: 'music_playDrumForBeats',
    syntax: 'music.play_drum(drum_id, beats)',
    returnType: 'Void',
    description: 'Plays a percussion drum sound for the given beat duration.',
    parameters: [
      { name: 'drum_id', type: 'Number', description: 'Drum preset ID (1 to 18: Snare, Bass, Side Stick, Crash Cymbal, etc.)', required: true },
      { name: 'beats', type: 'Number', description: 'Duration in musical beats', required: true }
    ],
    example: 'music.play_drum(1, 0.25)\nmusic.play_drum(2, 0.25)',
    lspSnippet: 'music.play_drum(${1:1}, ${2:0.25})',
    notes: 'Dispatches MusicEvent into world queue.'
  },
  {
    id: 'music-rest',
    name: 'music.rest',
    category: 'music',
    shape: 'Stack',
    opcode: 'music_restForBeats',
    syntax: 'music.rest(beats)',
    returnType: 'Void',
    description: 'Pauses execution for a duration calculated from tempo BPM: beats * (60.0 / tempo).',
    parameters: [
      { name: 'beats', type: 'Number', description: 'Rest duration in beats', required: true }
    ],
    example: 'music.rest(1.0)',
    lspSnippet: 'music.rest(${1:0.25})',
    notes: 'Calculates delta sleep time dynamically based on current tempo.'
  },
  {
    id: 'music-play-note',
    name: 'music.play_note',
    category: 'music',
    shape: 'Stack',
    opcode: 'music_playNoteForBeats',
    syntax: 'music.play_note(midi_note, beats)',
    returnType: 'Void',
    description: 'Plays a synthesized musical note (MIDI pitch 60 = Middle C, 69 = A440) for beat duration.',
    parameters: [
      { name: 'midi_note', type: 'Number', description: 'MIDI note number (60 = Middle C)', required: true },
      { name: 'beats', type: 'Number', description: 'Duration in musical beats', required: true }
    ],
    example: 'music.play_note(60, 0.5) # C\nmusic.play_note(64, 0.5) # E\nmusic.play_note(67, 0.5) # G',
    lspSnippet: 'music.play_note(${1:60}, ${2:0.5})',
    notes: 'Calculates exact frequency: 440 * 2^((note - 69)/12).'
  },
  {
    id: 'music-set-instrument',
    name: 'music.set_instrument',
    category: 'music',
    shape: 'Stack',
    opcode: 'music_setInstrument',
    syntax: 'music.set_instrument(instrument_id)',
    returnType: 'Void',
    description: 'Selects the synthesizer timbre (1: Piano, 2: Electric Piano, 3: Organ, 4: Guitar, 5: Bass, etc.).',
    parameters: [
      { name: 'instrument_id', type: 'Number', description: 'Instrument ID (1 to 21)', required: true }
    ],
    example: 'music.set_instrument(1)',
    lspSnippet: 'music.set_instrument(${1:1})',
    notes: 'Sets world.music_instrument.'
  },
  {
    id: 'music-set-tempo',
    name: 'music.set_tempo',
    category: 'music',
    shape: 'Stack',
    opcode: 'music_setTempo',
    syntax: 'music.set_tempo(bpm)',
    returnType: 'Void',
    description: 'Sets tempo in Beats Per Minute (BPM) for musical duration calculations.',
    parameters: [
      { name: 'bpm', type: 'Number', description: 'Beats per minute (e.g. 60, 120)', required: true }
    ],
    example: 'music.set_tempo(120)',
    lspSnippet: 'music.set_tempo(${1:60})',
    notes: 'Updates world.music_tempo and __music_tempo variable.'
  },
  {
    id: 'music-change-tempo',
    name: 'music.change_tempo',
    category: 'music',
    shape: 'Stack',
    opcode: 'music_changeTempo',
    syntax: 'music.change_tempo(delta)',
    returnType: 'Void',
    description: 'Adjusts current tempo BPM by delta.',
    parameters: [
      { name: 'delta', type: 'Number', description: 'Tempo delta (e.g. +20 or -10)', required: true }
    ],
    example: 'music.change_tempo(20)',
    lspSnippet: 'music.change_tempo(${1:20})',
    notes: 'Clamped to minimum 20 BPM.'
  },
  {
    id: 'music-get-tempo',
    name: 'music.get_tempo',
    category: 'music',
    shape: 'Reporter',
    opcode: 'music_getTempo',
    syntax: 'music.get_tempo()',
    returnType: 'Number',
    description: 'Returns the current musical tempo in Beats Per Minute (BPM).',
    parameters: [],
    example: 'let current_bpm = music.get_tempo()',
    lspSnippet: 'music.get_tempo()',
    notes: 'Defaults to 60.0 BPM.'
  },

  // ==================== 13. TEXT TO SPEECH (3 Blocks) ====================
  {
    id: 'tts-speak',
    name: 'tts.speak',
    category: 'tts',
    shape: 'Stack',
    opcode: 'text2speech_speakAndWait',
    syntax: 'tts.speak(text)',
    returnType: 'Void',
    description: 'Speaks the given string aloud using native speech synthesis and renders toast in GUI preview.',
    parameters: [
      { name: 'text', type: 'String', description: 'Text message to speak aloud', required: true }
    ],
    example: 'tts.set_voice("alto")\ntts.speak("Welcome to Scratch Lang!")',
    lspSnippet: 'tts.speak("${1:Hello world!}")',
    notes: 'Pushes string to world.tts_speech_queue.'
  },
  {
    id: 'tts-set-voice',
    name: 'tts.set_voice',
    category: 'tts',
    shape: 'Stack',
    opcode: 'text2speech_setVoice',
    syntax: 'tts.set_voice(voice)',
    returnType: 'Void',
    description: 'Sets voice character timbre: "alto", "tenor", "squeak", "giant", or "kitten".',
    parameters: [
      { name: 'voice', type: 'String', description: '"alto" | "tenor" | "squeak" | "giant" | "kitten"', required: true }
    ],
    example: 'tts.set_voice("squeak")',
    lspSnippet: 'tts.set_voice("${1|alto,tenor,squeak,giant,kitten|}")',
    notes: 'Configures world.tts_voice.'
  },
  {
    id: 'tts-set-language',
    name: 'tts.set_language',
    category: 'tts',
    shape: 'Stack',
    opcode: 'text2speech_setLanguage',
    syntax: 'tts.set_language(language)',
    returnType: 'Void',
    description: 'Configures text-to-speech pronunciation dialect and phoneme engine.',
    parameters: [
      { name: 'language', type: 'String', description: 'Language name or ISO code (e.g. "English", "Spanish", "Japanese")', required: true }
    ],
    example: 'tts.set_language("English")',
    lspSnippet: 'tts.set_language("${1|English,Spanish,French,German,Japanese|}")',
    notes: 'Sets world.tts_language.'
  },

  // ==================== 14. TRANSLATE (2 Blocks) ====================
  {
    id: 'translate-text',
    name: 'translate.text',
    category: 'translate',
    shape: 'Reporter',
    opcode: 'translate_getTranslate',
    syntax: 'translate.text(text, target_language)',
    returnType: 'String',
    description: 'Translates a string into the target language ISO code.',
    parameters: [
      { name: 'text', type: 'String', description: 'Source text string', required: true },
      { name: 'target_language', type: 'String', description: 'Target language name or ISO code ("Spanish", "es", "fr", "ja")', required: true }
    ],
    example: 'let greeting = translate.text("Hello", "Spanish") # "Hola"',
    lspSnippet: 'translate.text("${1:Hello}", "${2:Spanish}")',
    notes: 'Supports common phrases offline with online translation hook.'
  },
  {
    id: 'translate-get-language',
    name: 'translate.get_language',
    category: 'translate',
    shape: 'Reporter',
    opcode: 'translate_getViewerLanguage',
    syntax: 'translate.get_language()',
    returnType: 'String',
    description: 'Returns the host operating system ISO language code (e.g. "en", "es", "zh").',
    parameters: [],
    example: 'let user_lang = translate.get_language()',
    lspSnippet: 'translate.get_language()',
    notes: 'Derived from system environment and locale.'
  },

  // ==================== 15. MAKEY MAKEY (2 Blocks) ====================
  {
    id: 'makey-when-key',
    name: 'when makey.key:',
    category: 'makey',
    shape: 'Hat',
    opcode: 'makeymakey_whenMakeyKeyPressed',
    syntax: 'when makey.key("key"):\n    <statements>',
    returnType: 'Void',
    description: 'Triggered when a Makey Makey tactile alligator clip contact is closed (aliased to keyboard inputs).',
    parameters: [
      { name: 'key', type: 'String', description: '"space", "up", "down", "left", "right", "w", "a", "s", "d", "f", "g"', required: true }
    ],
    example: 'when makey.key("space"):\n    play_drum(1, 0.25)',
    lspSnippet: 'when makey.key("${1:space}"):\n    ${2:pass}',
    notes: 'Mapped directly to hardware key events.'
  },
  {
    id: 'makey-when-code',
    name: 'when makey.code:',
    category: 'makey',
    shape: 'Hat',
    opcode: 'makeymakey_whenCodePressed',
    syntax: 'when makey.code("sequence"):\n    <statements>',
    returnType: 'Void',
    description: 'Triggered when an input sequence combo (e.g. "up up down down left right") is matched against the input history buffer.',
    parameters: [
      { name: 'sequence', type: 'String', description: 'Space-delimited key sequence', required: true }
    ],
    example: 'when makey.code("up up down down"):\n    say("Player", "Cheat Code Activated!")',
    lspSnippet: 'when makey.code("${1:up up down down}"):\n    ${2:pass}',
    notes: 'Maintains circular FIFO buffer of last 16 key inputs.'
  }
];
