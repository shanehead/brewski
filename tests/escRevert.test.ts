import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { escRevert } from "$lib/actions/escRevert";

function makeInput(initialValue = ""): HTMLInputElement {
  const el = document.createElement("input");
  el.type = "text";
  el.value = initialValue;
  document.body.appendChild(el);
  return el;
}

function fireKeydown(el: HTMLElement, key: string): KeyboardEvent {
  const e = new KeyboardEvent("keydown", { key, bubbles: true, cancelable: true });
  el.dispatchEvent(e);
  return e;
}

function fireFocus(el: HTMLElement) {
  el.dispatchEvent(new FocusEvent("focus", { bubbles: true }));
}

describe("escRevert action", () => {
  let el: HTMLInputElement;
  let blurSpy: ReturnType<typeof vi.spyOn>;

  beforeEach(() => {
    el = makeInput("original");
    blurSpy = vi.spyOn(el, "blur");
  });

  afterEach(() => {
    el.remove();
    vi.restoreAllMocks();
  });

  it("reverts to the value at focus-time when Escape is pressed after typing", () => {
    const action = escRevert(el);

    fireFocus(el); // captures "original"
    el.value = "changed";

    fireKeydown(el, "Escape");

    expect(el.value).toBe("original");
    expect(blurSpy).toHaveBeenCalledOnce();

    action.destroy();
  });

  it("dispatches an input event on revert to keep bind:value in sync", () => {
    const action = escRevert(el);
    const inputListener = vi.fn();
    el.addEventListener("input", inputListener);

    fireFocus(el);
    el.value = "changed";

    fireKeydown(el, "Escape");

    expect(inputListener).toHaveBeenCalledOnce();

    el.removeEventListener("input", inputListener);
    action.destroy();
  });

  it("does NOT dispatch an input event when value is unchanged", () => {
    const action = escRevert(el);
    const inputListener = vi.fn();
    el.addEventListener("input", inputListener);

    fireFocus(el);
    // do not change el.value

    fireKeydown(el, "Escape");

    expect(el.value).toBe("original");
    expect(blurSpy).toHaveBeenCalledOnce();
    expect(inputListener).not.toHaveBeenCalled();

    el.removeEventListener("input", inputListener);
    action.destroy();
  });

  it("stops the keydown event from propagating so a parent overlay does not also receive it", () => {
    const action = escRevert(el);
    const windowListener = vi.fn();
    window.addEventListener("keydown", windowListener);

    fireFocus(el);
    fireKeydown(el, "Escape");

    expect(windowListener).not.toHaveBeenCalled();

    window.removeEventListener("keydown", windowListener);
    action.destroy();
  });

  it("does not react to non-Escape keys", () => {
    const action = escRevert(el);

    fireFocus(el);
    el.value = "changed";

    fireKeydown(el, "Enter");

    expect(el.value).toBe("changed");
    expect(blurSpy).not.toHaveBeenCalled();

    action.destroy();
  });

  it("captures a new baseline on each focus", () => {
    const action = escRevert(el);

    // First edit: commit it (simulate blur without escaping)
    fireFocus(el);
    el.value = "first edit";
    // don't escape — user committed the value

    // Second focus captures the new value
    fireFocus(el);
    el.value = "second edit";

    fireKeydown(el, "Escape");

    expect(el.value).toBe("first edit");

    action.destroy();
  });

  it("removes event listeners on destroy", () => {
    const action = escRevert(el);
    action.destroy();

    const windowListener = vi.fn();
    window.addEventListener("keydown", windowListener);

    fireFocus(el);
    el.value = "changed";
    fireKeydown(el, "Escape");

    // After destroy the keydown handler is gone, so the event should NOT be
    // stopped — the window listener WILL receive it.
    expect(windowListener).toHaveBeenCalledOnce();

    window.removeEventListener("keydown", windowListener);
    // blur should NOT have been called (handler removed)
    expect(blurSpy).not.toHaveBeenCalled();
  });

  it("works with a textarea element", () => {
    const textarea = document.createElement("textarea");
    textarea.value = "initial text";
    document.body.appendChild(textarea);
    const blurSpyTA = vi.spyOn(textarea, "blur");

    const action = escRevert(textarea);
    fireFocus(textarea);
    textarea.value = "modified";
    fireKeydown(textarea, "Escape");

    expect(textarea.value).toBe("initial text");
    expect(blurSpyTA).toHaveBeenCalledOnce();

    textarea.remove();
    action.destroy();
  });
});
