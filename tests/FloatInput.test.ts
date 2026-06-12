import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import FloatInput from "$lib/components/FloatInput.svelte";

describe("FloatInput", () => {
  it("displays value formatted to specified decimals", () => {
    render(FloatInput, { value: 5.789, decimals: 2, oncommit: vi.fn() });
    const input = screen.getByRole("spinbutton") as HTMLInputElement;
    expect(input.value).toBe("5.79");
  });

  it("defaults to 2 decimal places", () => {
    render(FloatInput, { value: 1, oncommit: vi.fn() });
    const input = screen.getByRole("spinbutton") as HTMLInputElement;
    expect(input.value).toBe("1.00");
  });

  it("shows empty string when value is null", () => {
    render(FloatInput, { value: null, oncommit: vi.fn() });
    const input = screen.getByRole("spinbutton") as HTMLInputElement;
    expect(input.value).toBe("");
  });

  it("renders with type=number and inputmode=decimal", () => {
    render(FloatInput, { value: 1, oncommit: vi.fn() });
    const input = screen.getByRole("spinbutton");
    expect(input).toHaveAttribute("type", "number");
    expect(input).toHaveAttribute("inputmode", "decimal");
  });

  it("calls oncommit with parsed number on blur", async () => {
    const oncommit = vi.fn();
    const user = userEvent.setup();
    render(FloatInput, { value: 1, decimals: 2, oncommit });
    const input = screen.getByRole("spinbutton");
    await user.clear(input);
    await user.type(input, "3.14");
    await user.tab();
    expect(oncommit).toHaveBeenCalledWith(3.14);
  });

  it("calls oncommit with null when field is cleared and blurred", async () => {
    const oncommit = vi.fn();
    const user = userEvent.setup();
    render(FloatInput, { value: 5, decimals: 1, oncommit });
    const input = screen.getByRole("spinbutton");
    await user.clear(input);
    await user.tab();
    expect(oncommit).toHaveBeenCalledWith(null);
  });

  it("does not call oncommit with the edited value after Escape", async () => {
    const oncommit = vi.fn();
    const user = userEvent.setup();
    render(FloatInput, { value: 5, decimals: 1, oncommit });
    const input = screen.getByRole("spinbutton") as HTMLInputElement;
    await user.clear(input);
    await user.type(input, "99");
    await user.keyboard("{Escape}");
    // escRevert restores original value and triggers blur with original.
    // jsdom normalizes type="number" values, so "5.0" becomes "5".
    expect(input.value).toBe("5");
    expect(oncommit).not.toHaveBeenCalledWith(99);
  });

  it("forwards style prop to the input", () => {
    render(FloatInput, { value: 1, oncommit: vi.fn(), style: "color: red;" });
    const input = screen.getByRole("spinbutton");
    expect(input).toHaveAttribute("style", "color: red;");
  });

  it("forwards id, step, min, placeholder, class props to the input", () => {
    render(FloatInput, {
      value: 1,
      oncommit: vi.fn(),
      id: "my-input",
      step: "0.1",
      min: "0",
      placeholder: "enter value",
      class: "my-class",
    });
    const input = screen.getByRole("spinbutton");
    expect(input).toHaveAttribute("id", "my-input");
    expect(input).toHaveAttribute("step", "0.1");
    expect(input).toHaveAttribute("min", "0");
    expect(input).toHaveAttribute("placeholder", "enter value");
    expect(input).toHaveClass("my-class");
  });
});
