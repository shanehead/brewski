<script lang="ts">
  import { escRevert } from "$lib/actions/escRevert";

  let {
    value,
    decimals = 2,
    oncommit,
    id,
    step,
    min,
    placeholder,
    class: className,
    style,
    disabled = false,
  }: {
    value: number | null;
    decimals?: number;
    oncommit: (v: number | null) => void;
    id?: string;
    step?: number | string;
    min?: number | string;
    placeholder?: string;
    class?: string;
    style?: string;
    disabled?: boolean;
  } = $props();

  const display = $derived(value != null ? value.toFixed(decimals) : "");
</script>

<input
  {id}
  type="number"
  inputmode="decimal"
  {step}
  {min}
  {placeholder}
  {disabled}
  class={className}
  {style}
  value={display}
  use:escRevert
  onblur={(e) => {
    const v = parseFloat((e.target as HTMLInputElement).value);
    oncommit(isNaN(v) ? null : v);
  }}
/>
