<script lang="ts">
	import type { ComponentType, SvelteComponent } from 'svelte';
	import type { MarkdownTextElement, MarkdownTextElementKind } from '../compiler';
	import Text from './elements/Text.svelte';
	import Mono from './elements/Mono.svelte';
	import Cursor from './elements/Cursor.svelte';
	import ListDot from './elements/ListDot.svelte';
	import Header from './elements/Header.svelte';
	import InlineCode from './elements/InlineCode.svelte';

	export let element: MarkdownTextElement;

	const kindResolver: Map<
		MarkdownTextElementKind,
		ComponentType<SvelteComponent<{element: any}>>
	> = new Map();

    kindResolver.set("text", Text);
    kindResolver.set("mono", Mono);
    kindResolver.set("cursor", Cursor);
    kindResolver.set("listdot", ListDot);
    kindResolver.set("header", Header);
	kindResolver.set("inlinecode", InlineCode);
</script>

{#if kindResolver.has(element.kind)}
    <svelte:component this={kindResolver.get(element.kind)} {element} />
{:else}
	<span class="text-red-400 text-sm"> Unresolved tag: {JSON.stringify(element)}</span>
{/if}
