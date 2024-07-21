<script lang="ts">
	import type { CompilationResult } from "../compiler";
	import BlockRender from "./BlockRender.svelte";

    export let compiledData: CompilationResult;
</script>

{#each compiledData as block}
    <div class="min-h-[1rem]">
        <BlockRender {block} />
    </div>
{/each}