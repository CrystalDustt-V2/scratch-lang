import { FUNCTIONS_DATA } from '../../data/functionsData';
import { CLI_COMMANDS } from '../../data/cliData';
import { CATEGORIES } from '../../data/categories';

export default function handler(req, res) {
  const { category, search } = req.query;

  let results = [...FUNCTIONS_DATA];

  if (category && category !== 'all') {
    results = results.filter((fn) => fn.category.toLowerCase() === category.toLowerCase());
  }

  if (search) {
    const q = search.toLowerCase();
    results = results.filter(
      (fn) =>
        fn.name.toLowerCase().includes(q) ||
        fn.opcode.toLowerCase().includes(q) ||
        fn.description.toLowerCase().includes(q) ||
        fn.syntax.toLowerCase().includes(q)
    );
  }

  res.status(200).json({
    version: '1.0.0',
    totalFunctions: results.length,
    categories: CATEGORIES,
    functions: results,
    cliCommands: CLI_COMMANDS,
    generatedAt: new Date().toISOString(),
  });
}
