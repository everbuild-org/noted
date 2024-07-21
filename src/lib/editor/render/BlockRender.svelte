<script lang="ts">
	import type { MarkdownTextlineBlock } from "../compiler";
	import ElementRender from "./ElementRender.svelte";

    export let block: MarkdownTextlineBlock;
</script>

{#each block as element}
    <ElementRender {element} />
{/each}