<script lang="ts">
	import type { StyledTextElement } from "$lib/editor/compiler";
	import RawText from "./RawText.svelte";

    export let element: StyledTextElement;
</script>

<span 
    class:font-bold={element.bold} 
    class:underline={element.underline} 
    class:line-through={element.strikethrough} 
    class:italic={element.italic}
><RawText text={element.rawText} startPosition={element.elementStart} /></span>