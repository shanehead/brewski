import { describe, it, expect } from 'vitest';
import { wrapSelection, insertLinePrefix, insertBlock } from '$lib/markdown';

describe('wrapSelection', () => {
  it('wraps selected text in before/after markers', () => {
    const result = wrapSelection('hello world', 6, 11, '**', '**');
    expect(result.value).toBe('hello **world**');
    expect(result.selStart).toBe(8);   // after opening **
    expect(result.selEnd).toBe(13);    // before closing **
  });

  it('inserts markers at cursor with no selection', () => {
    const result = wrapSelection('hello', 5, 5, '**', '**');
    expect(result.value).toBe('hello****');
    expect(result.selStart).toBe(7);   // cursor between the **
    expect(result.selEnd).toBe(7);
  });

  it('works at start of string', () => {
    const result = wrapSelection('world', 0, 5, '*', '*');
    expect(result.value).toBe('*world*');
    expect(result.selStart).toBe(1);
    expect(result.selEnd).toBe(6);
  });
});

describe('insertLinePrefix', () => {
  it('inserts prefix at start of the current line', () => {
    // cursor at pos 8, single line — lineStart is 0
    const result = insertLinePrefix('hello world', 8, '## ');
    expect(result.value).toBe('## hello world');
    expect(result.selStart).toBe(3);  // after '## '
    expect(result.selEnd).toBe(3);
  });

  it('inserts prefix at the correct line in multiline text', () => {
    // 'line two' starts at pos 9; cursor is at pos 12 (inside 'line two')
    const text = 'line one\nline two\nline three';
    const result = insertLinePrefix(text, 12, '- ');
    expect(result.value).toBe('line one\n- line two\nline three');
    expect(result.selStart).toBe(11); // 9 (lineStart) + 2 (prefix)
    expect(result.selEnd).toBe(11);
  });

  it('handles cursor at start of file', () => {
    const result = insertLinePrefix('hello', 0, '- ');
    expect(result.value).toBe('- hello');
    expect(result.selStart).toBe(2);
  });
});

describe('insertBlock', () => {
  it('inserts block with a leading newline when not at line start', () => {
    const result = insertBlock('hello', 5, '---');
    expect(result.value).toBe('hello\n---\n');
    expect(result.selStart).toBe(10);
    expect(result.selEnd).toBe(10);
  });

  it('inserts block without extra newline when already at line start', () => {
    const result = insertBlock('hello\n', 6, '---');
    expect(result.value).toBe('hello\n---\n');
    expect(result.selStart).toBe(10);
    expect(result.selEnd).toBe(10);
  });

  it('inserts block without extra newline when string is empty', () => {
    const result = insertBlock('', 0, '---');
    expect(result.value).toBe('---\n');
    expect(result.selStart).toBe(4);
  });
});
