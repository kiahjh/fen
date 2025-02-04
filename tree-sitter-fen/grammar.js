/**
 * @file An SDL for type-safe cross-language http communication
 * @author Miciah Henderson <miciahjohnhenderson@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: `fen`,

  rules: {
    // TODO: add the actual grammar rules
    source_file: ($) => `hello`,
  },
});
