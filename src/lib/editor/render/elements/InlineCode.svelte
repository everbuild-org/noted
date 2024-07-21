<script lang="ts">
	import type { InlineCode } from "$lib/editor/compiler";

    export let element: InlineCode;
</script>

<span class="bg-[var(--background-card)] px-1 rounded-sm font-mono">{element.rawText}</span>