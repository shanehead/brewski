export function wrapSelection(
  text: string,
  selStart: number,
  selEnd: number,
  before: string,
  after: string
): { value: string; selStart: number; selEnd: number } {
  const selected = text.slice(selStart, selEnd);
  const value = text.slice(0, selStart) + before + selected + after + text.slice(selEnd);
  return {
    value,
    selStart: selStart + before.length,
    selEnd: selStart + before.length + selected.length,
  };
}

export function insertLinePrefix(
  text: string,
  cursorPos: number,
  prefix: string
): { value: string; selStart: number; selEnd: number } {
  const lineStart = text.lastIndexOf('\n', cursorPos - 1) + 1;
  const value = text.slice(0, lineStart) + prefix + text.slice(lineStart);
  const newPos = lineStart + prefix.length;
  return { value, selStart: newPos, selEnd: newPos };
}

export function insertBlock(
  text: string,
  cursorPos: number,
  block: string
): { value: string; selStart: number; selEnd: number } {
  const before = text.slice(0, cursorPos);
  const after = text.slice(cursorPos);
  const newLine = before.endsWith('\n') || before === '' ? '' : '\n';
  const insertion = newLine + block + '\n';
  const value = before + insertion + after;
  const newPos = cursorPos + insertion.length;
  return { value, selStart: newPos, selEnd: newPos };
}
