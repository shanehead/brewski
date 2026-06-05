import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import ImagePickerModal from '$lib/components/ImagePickerModal.svelte';
import type { ImageRef } from '$lib/api';

const images: ImageRef[] = [
  { id: 'id-1', name: 'brew-day.jpg', assetUrl: 'asset://brew-day.jpg' },
  { id: 'id-2', name: 'grain-bill.jpg', assetUrl: 'asset://grain-bill.jpg' },
];

describe('ImagePickerModal', () => {
  it('renders a thumbnail for each image', () => {
    render(ImagePickerModal, { images, onInsert: vi.fn(), onClose: vi.fn() });
    expect(screen.getByAltText('brew-day.jpg')).toBeTruthy();
    expect(screen.getByAltText('grain-bill.jpg')).toBeTruthy();
  });

  it('Insert button is disabled until a photo is selected', () => {
    render(ImagePickerModal, { images, onInsert: vi.fn(), onClose: vi.fn() });
    expect(screen.getByRole('button', { name: 'Insert' })).toBeDisabled();
  });

  it('calls onInsert with the selected image then closes', async () => {
    const user = userEvent.setup();
    const onInsert = vi.fn();
    const onClose = vi.fn();
    render(ImagePickerModal, { images, onInsert, onClose });
    await user.click(screen.getByAltText('brew-day.jpg'));
    await user.click(screen.getByRole('button', { name: 'Insert' }));
    expect(onInsert).toHaveBeenCalledWith(images[0]);
    expect(onClose).toHaveBeenCalled();
  });

  it('calls onClose when Cancel is clicked', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(ImagePickerModal, { images, onInsert: vi.fn(), onClose });
    await user.click(screen.getByRole('button', { name: 'Cancel' }));
    expect(onClose).toHaveBeenCalled();
  });

  it('shows empty state and no Insert button when images is empty', () => {
    render(ImagePickerModal, { images: [], onInsert: vi.fn(), onClose: vi.fn() });
    expect(screen.getByText(/No photos yet/)).toBeTruthy();
    expect(screen.queryByRole('button', { name: 'Insert' })).toBeNull();
  });

  it('calls onClose when Escape is pressed', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(ImagePickerModal, { images, onInsert: vi.fn(), onClose });
    await user.keyboard('{Escape}');
    expect(onClose).toHaveBeenCalled();
  });

  it('calls onClose when backdrop is clicked', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    const { container } = render(ImagePickerModal, { images, onInsert: vi.fn(), onClose });
    // click the backdrop (the outermost element)
    await user.click(container.firstElementChild!);
    expect(onClose).toHaveBeenCalled();
  });
});
