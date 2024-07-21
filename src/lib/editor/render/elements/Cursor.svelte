<script lang="ts">
	import type { Cursor } from "$lib/editor/compiler";

    export let element: Cursor;
    element.kind;
</script>

<span class="w-0 relative">
    <div class="themable-cursor absolute w-px top-0 left-0 bottom-0 bg-white blinking"></div>
</span>

<style>
    .blinking {
        animation: blinking 1s step-end infinite;
    }

    @keyframes blinking {
        from {
            opacity: 1;
        }

        50% {
            opacity: 0;
        }

        to {
            opacity: 0;
        }
    }
</style>