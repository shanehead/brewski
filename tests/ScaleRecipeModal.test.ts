import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { tick } from "svelte";
import ScaleRecipeModal from "$lib/components/ScaleRecipeModal.svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("$lib/stores/error", () => ({ ipc: vi.fn((p) => p) }));
vi.mock("$app/navigation", () => ({ goto: vi.fn() }));

let currentUnits = "metric";

vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => {
      fn({ units: currentUnits });
      return () => {};
    }),
  },
}));

const mockInvoke = vi.mocked((await import("@tauri-apps/api/core")).invoke);
const { goto } = await import("$app/navigation");
const mockGoto = vi.mocked(goto);

beforeEach(() => {
  mockInvoke.mockReset();
  mockGoto.mockReset();
  currentUnits = "metric";
});

describe("ScaleRecipeModal", () => {
  it("pre-fills input with current batch size in liters (metric)", () => {
    render(ScaleRecipeModal, {
      props: { recipeId: "r1", currentBatchSizeL: 23.0, onClose: vi.fn() },
    });
    const input = screen.getByRole("spinbutton") as HTMLInputElement;
    expect(input.value).toBe("23");
    expect(screen.getByText("L")).toBeTruthy();
  });

  it("pre-fills input with current batch size in gallons (imperial)", async () => {
    currentUnits = "imperial";
    render(ScaleRecipeModal, {
      props: { recipeId: "r1", currentBatchSizeL: 23.0, onClose: vi.fn() },
    });
    const input = screen.getByRole("spinbutton") as HTMLInputElement;
    // 23L * 0.264172 ≈ 6.08 gal
    expect(parseFloat(input.value)).toBeCloseTo(6.08, 1);
    expect(screen.getByText("gal")).toBeTruthy();
  });

  it("calls scale_recipe with liters and navigates to new recipe on confirm", async () => {
    const user = userEvent.setup();
    mockInvoke.mockResolvedValue({ id: "new-recipe-id", name: "My IPA (scaled)" });

    render(ScaleRecipeModal, {
      props: { recipeId: "r1", currentBatchSizeL: 23.0, onClose: vi.fn() },
    });

    const input = screen.getByRole("spinbutton");
    await user.clear(input);
    await user.type(input, "46");

    await user.click(screen.getByRole("button", { name: /scale/i }));
    await tick();
    await tick();

    expect(mockInvoke).toHaveBeenCalledWith("scale_recipe", {
      recipeId: "r1",
      newBatchSizeL: 46,
    });
    expect(mockGoto).toHaveBeenCalledWith("/recipe/new-recipe-id");
  });

  it("calls onClose when Cancel is clicked", async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(ScaleRecipeModal, {
      props: { recipeId: "r1", currentBatchSizeL: 23.0, onClose },
    });
    await user.click(screen.getByRole("button", { name: /cancel/i }));
    expect(onClose).toHaveBeenCalled();
  });
});
