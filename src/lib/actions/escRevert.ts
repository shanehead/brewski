// Clears an input's value when Escape is pressed (and stops propagation so a
// parent overlay's Escape handler does not also fire). If the value is already
// empty, lets Escape propagate so the parent can handle it (e.g. close a modal).
export function escClear(node: HTMLInputElement) {
  const onKey: EventListener = (evt: Event) => {
    const e = evt as KeyboardEvent;
    if (e.key !== "Escape") return;
    if (node.value !== "") {
      e.stopPropagation();
      node.value = "";
      node.dispatchEvent(new Event("input", { bubbles: true }));
    }
  };
  node.addEventListener("keydown", onKey);
  return {
    destroy() { node.removeEventListener("keydown", onKey); },
  };
}

// Reverts an input/textarea/select to its value at focus-time when Escape is
// pressed, blurs it, and stops the event from bubbling so a parent overlay's
// Escape handler does not also fire.
export function escRevert(node: HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement) {
  let original = node.value;
  const onFocus: EventListener = () => { original = node.value; };
  const onKey: EventListener = (evt: Event) => {
    const e = evt as KeyboardEvent;
    if (e.key !== "Escape") return;
    e.stopPropagation();
    if (node.value !== original) {
      node.value = original;
      // Sync Svelte's bind:value (input) without triggering change/blur saves.
      node.dispatchEvent(new Event("input", { bubbles: true }));
    }
    node.blur();
  };
  node.addEventListener("focus", onFocus);
  node.addEventListener("keydown", onKey);
  return {
    destroy() {
      node.removeEventListener("focus", onFocus);
      node.removeEventListener("keydown", onKey);
    },
  };
}
