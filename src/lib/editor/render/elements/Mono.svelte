<script lang="ts">
	import type { MonospaceElement } from "$lib/editor/compiler";
	import RawText from "./RawText.svelte";

    export let element: MonospaceElement;
</script>

<RawText text={element.rawText} />
