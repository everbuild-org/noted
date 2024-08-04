<script lang="ts">
    export let text: string;
    export let startPosition: number;
</script>

{#each text.split("") as character, i}
    {#if character === " "}<span class="inline-block w-[4px]" data-pos={startPosition + i}></span>{:else}<span
            data-pos={startPosition + i}>{character}</span>{/if}
{/each}
