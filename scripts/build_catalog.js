const fs = require('fs');
const path = require('path');

const srcPath = path.join(__dirname, '..', 'website', 'src', 'data', 'functionsData.js');
const outPath = path.join(__dirname, '..', 'editors', 'vscode', 'src', 'catalog.js');

const src = fs.readFileSync(srcPath, 'utf8');
const cjsData = src.replace('export const FUNCTIONS_DATA =', 'const FUNCTIONS_DATA =');

const extraCode = `
const KEYWORDS_DATA = [
  {
    name: 'when',
    category: 'events',
    syntax: 'when <trigger>:',
    description: 'Defines a top-level event handler that executes when the specified trigger condition fires.',
    example: 'when start:\\n    move("Player", 10)',
    snippet: 'when \${1:start}:\\n    \${0}'
  },
  {
    name: 'if',
    category: 'control',
    syntax: 'if <condition>:',
    description: 'Evaluates condition. If true, runs the indented block of code.',
    example: 'if touching("Player", "Enemy"):\\n    damage("Player", 1)',
    snippet: 'if \${1:condition}:\\n    \${0}'
  },
  {
    name: 'else',
    category: 'control',
    syntax: 'else:',
    description: 'Executes alternative indented block when the preceding if-condition evaluates to false.',
    example: 'if touching("Player", "Ground"):\\n    jump("Player", 10)\\nelse:\\n    fall("Player", 5)',
    snippet: 'else:\\n    \${0}'
  },
  {
    name: 'repeat',
    category: 'control',
    syntax: 'repeat <count>:',
    description: 'Executes the indented block a specified number of times.',
    example: 'repeat 5:\\n    move("Player", 10)',
    snippet: 'repeat \${1:5}:\\n    \${0}'
  },
  {
    name: 'while',
    category: 'control',
    syntax: 'while <condition>:',
    description: 'Loops and executes the indented block while the condition expression evaluates to true.',
    example: 'while distance("Player", "Goal") > 10:\\n    move("Player", 2)',
    snippet: 'while \${1:condition}:\\n    \${0}'
  },
  {
    name: 'return',
    category: 'control',
    syntax: 'return',
    description: 'Exits early from the current event handler or function execution.',
    example: 'if is_dead("Player"):\\n    return',
    snippet: 'return'
  },
  {
    name: 'touches',
    category: 'sensing',
    syntax: '<ObjectA> touches <ObjectB>',
    description: 'Collision trigger condition checking if ObjectA collides with ObjectB.',
    example: 'when Player touches Enemy:\\n    damage("Player", 1)',
    snippet: '\${1:Player} touches \${2:Enemy}'
  }
];

const EVENTS_DATA = [
  {
    name: 'when start',
    label: 'when start:',
    detail: 'Lifecycle Event',
    description: 'Runs once when the game boots or a scene loads.',
    example: 'when start:\\n    teleport("Player", 0, 0)',
    snippet: 'when start:\\n    \${0}'
  },
  {
    name: 'when update',
    label: 'when update:',
    detail: 'Frame Loop Event',
    description: 'Runs every single frame at 60 FPS. Ideal for movement updates and game loops.',
    example: 'when update:\\n    move("Player", 2)',
    snippet: 'when update:\\n    \${0}'
  },
  {
    name: 'when action.press',
    label: 'when action.press("..."):',
    detail: 'Input Trigger Event',
    description: 'Fires once when the specified action key is first pressed down.',
    example: 'when action.press("jump"):\\n    jump("Player", 12)',
    snippet: 'when action.press("\${1|jump,left,right,up,down,action|}"):\\n    \${0}'
  },
  {
    name: 'when action.down',
    label: 'when action.down("..."):',
    detail: 'Input Hold Event',
    description: 'Fires continuously every frame while the specified action key is held down.',
    example: 'when action.down("right"):\\n    move("Player", 5)',
    snippet: 'when action.down("\${1|right,left,up,down,jump,action|}"):\\n    \${0}'
  },
  {
    name: 'when action.up',
    label: 'when action.up("..."):',
    detail: 'Input Release Event',
    description: 'Fires once when the specified action key is released.',
    example: 'when action.up("jump"):\\n    stop("Player")',
    snippet: 'when action.up("\${1|jump,left,right,up,down,action|}"):\\n    \${0}'
  },
  {
    name: 'when collision',
    label: 'when Object touches Object:',
    detail: 'Collision Event',
    description: 'Fires when two named entities or sprite groups overlap in collision.',
    example: 'when Player touches Coin:\\n    collect("Coin")\\n    add_score(10)',
    snippet: 'when \${1:Player} touches \${2:Enemy}:\\n    \${0}'
  },
  {
    name: 'every N seconds',
    label: 'every N seconds:',
    detail: 'Periodic Timer Event',
    description: 'Repeats execution periodically every N seconds.',
    example: 'every 2 seconds:\\n    spawn("Enemy", 100, 0)',
    snippet: 'every \${1:1} seconds:\\n    \${0}'
  },
  {
    name: 'after N seconds',
    label: 'after N seconds:',
    detail: 'Delayed Timer Event',
    description: 'Executes once after an initial delay of N seconds.',
    example: 'after 3 seconds:\\n    sound_play("win")',
    snippet: 'after \${1:2} seconds:\\n    \${0}'
  },
  {
    name: 'when message',
    label: 'when message("..."):',
    detail: 'Broadcast Listener Event',
    description: 'Fires when a broadcast message with the matching key is sent via broadcast("...").',
    example: 'when message("level_clear"):\\n    scene_switch("level2")',
    snippet: 'when message("\${1:message_name}"):\\n    \${0}'
  }
];

const KNOWN_OBJECTS = [
  'Player', 'Enemy', 'Coin', 'Ground', 'Hazard', 'Door', 'Goal', 'Platform', 'Bullet', 'Score', 'Camera'
];

const INPUT_ACTIONS = [
  'left', 'right', 'up', 'down', 'jump', 'action'
];

// Quick index lookup map
const FUNCTION_MAP = new Map();
for (const fn of FUNCTIONS_DATA) {
  FUNCTION_MAP.set(fn.name, fn);
  FUNCTION_MAP.set(fn.name.toLowerCase(), fn);
}

const KEYWORD_MAP = new Map();
for (const kw of KEYWORDS_DATA) {
  KEYWORD_MAP.set(kw.name, kw);
}

module.exports = {
  FUNCTIONS_DATA,
  KEYWORDS_DATA,
  EVENTS_DATA,
  KNOWN_OBJECTS,
  INPUT_ACTIONS,
  FUNCTION_MAP,
  KEYWORD_MAP,
};
`;

fs.writeFileSync(outPath, cjsData + '\n' + extraCode, 'utf8');
console.log('Successfully generated ' + outPath);
