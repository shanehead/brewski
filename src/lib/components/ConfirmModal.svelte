<script lang="ts">
  let {
    message,
    confirmLabel = "Confirm",
    dangerous = false,
    onconfirm,
    oncancel,
  }: {
    message: string;
    confirmLabel?: string;
    dangerous?: boolean;
    onconfirm: () => void;
    oncancel: () => void;
  } = $props();
</script>

<div class="fixed inset-0 flex items-center justify-center" style="z-index: 1000;">
  <div class="absolute inset-0" style="background: rgba(0,0,0,0.4);" role="none" onclick={oncancel} onkeydown={oncancel}></div>
  <div class="p-4 rounded relative bg-bg-elevated border border-border" style="z-index: 1001; min-width: 320px;">
    <div class="text-sm mb-3 text-text-primary">{message}</div>
    <div class="flex justify-end gap-2">
      <button
        onclick={oncancel}
        class="px-3 py-1 rounded text-sm bg-bg-surface text-text-primary border border-border"
       
      >Cancel</button>
      <button
        onclick={onconfirm}
        class="px-3 py-1 rounded text-sm"
        style="background: {dangerous ? '#dc2626' : 'var(--color-accent)'}; color: #fff;"
      >{confirmLabel}</button>
    </div>
  </div>
</div>
