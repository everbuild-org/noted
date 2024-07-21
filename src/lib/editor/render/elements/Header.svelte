<script lang="ts">
	import type { Header } from "$lib/editor/compiler";
	import BlockRender from "../BlockRender.svelte";

    export let element: Header;

    const headers = ["h1", "h2", "h3", "h4", "h5", "h6"];
    const classes = ["text-4xl", "text-2xl", "text-lg", "text-md", "text-sm", "text-xs"]
    $: elementName = headers[element.depth-1];
    $: elementClass = classes[element.depth-1]; 
</script>

<svelte:element this={elementName} class="themable-{elementName} {elementClass} font-bold pb-2">
    <BlockRender block={element.blocks} />
</svelte:element>
