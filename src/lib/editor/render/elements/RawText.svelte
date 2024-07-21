<script lang="ts">
    export let text: string;
</script>

{#each text.split(" ") as word, i}{#if i != 0}<span class="inline-block w-[4px] bg-red-500"></span>{/if}{word}{/each}