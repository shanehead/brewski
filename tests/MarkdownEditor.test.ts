import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { tick } from 'svelte';
import MarkdownEditor from '$lib/components/MarkdownEditor.svelte';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

describe('MarkdownEditor', () => {
  it('shows a textarea in Write mode by default', () => {
    render(MarkdownEditor, { value: 'hello', onchange: vi.fn() });
    expect(screen.getByRole('textbox')).toBeTruthy();
    expect(screen.getByRole('button', { name: 'Write' })).toBeTruthy();
    expect(screen.getByRole('button', { name: 'Preview' })).toBeTruthy();
  });

  it('switches to Preview and renders markdown as HTML', async () => {
    const user = userEvent.setup();
    render(MarkdownEditor, { value: '**bold text**', onchange: vi.fn() });
    await user.click(screen.getByRole('button', { name: 'Preview' }));
    await tick();
    expect(screen.queryByRole('textbox')).toBeNull();
    const preview = document.querySelector('.md-preview');
    expect(preview?.innerHTML).toContain('<strong>bold text</strong>');
  });

  it('switches back to Write from Preview', async () => {
    const user = userEvent.setup();
    render(MarkdownEditor, { value: 'hello', onchange: vi.fn() });
    await user.click(screen.getByRole('button', { name: 'Preview' }));
    await tick();
    await user.click(screen.getByRole('button', { name: 'Write' }));
    await tick();
    expect(screen.getByRole('textbox')).toBeTruthy();
  });

  it('calls onchange with the typed value on blur', async () => {
    const user = userEvent.setup();
    const onchange = vi.fn();
    render(MarkdownEditor, { value: null, onchange });
    const textarea = screen.getByRole('textbox');
    await user.click(textarea);
    await user.keyboard('my notes');
    await user.tab();
    expect(onchange).toHaveBeenCalledWith('my notes');
  });

  it('calls onchange with null when value is cleared and blurred', async () => {
    const user = userEvent.setup();
    const onchange = vi.fn();
    render(MarkdownEditor, { value: 'existing', onchange });
    const textarea = screen.getByRole('textbox') as HTMLTextAreaElement;
    await user.clear(textarea);
    await user.tab();
    expect(onchange).toHaveBeenCalledWith(null);
  });

  it('renders task list checkboxes in Preview', async () => {
    const user = userEvent.setup();
    render(MarkdownEditor, { value: '- [ ] todo\n- [x] done', onchange: vi.fn() });
    await user.click(screen.getByRole('button', { name: 'Preview' }));
    await tick();
    const checkboxes = document.querySelectorAll('.md-preview input[type="checkbox"]');
    expect(checkboxes.length).toBe(2);
    expect((checkboxes[0] as HTMLInputElement).checked).toBe(false);
    expect((checkboxes[1] as HTMLInputElement).checked).toBe(true);
  });

  it('renders null value as empty textarea with placeholder', () => {
    render(MarkdownEditor, { value: null, onchange: vi.fn(), placeholder: 'Add notes…' });
    const textarea = screen.getByRole('textbox') as HTMLTextAreaElement;
    expect(textarea.value).toBe('');
    expect(textarea.placeholder).toBe('Add notes…');
  });
});
