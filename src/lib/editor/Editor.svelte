<script lang="ts">
	import { compileSource } from "./compiler";
	import MdRender from "./render/MdRender.svelte";

    export let source: string;
    if (!source) source = "";

    let selection: [number, number] = [0, 0]

    $: compiledData = compileSource(source, selection);

    function insertCharacterIntoSelection(char: string) {
        source = source.substring(0, selection[0]) + char + source.substring(selection[1]);
        selection = [selection[0]+1, selection[0]+1];
    }

    function deleteCharacterFromSelection(direction: 1 | -1) {
        if (selection[0] < selection[1]) {
            source = source.substring(0, selection[0]) + source.substring(selection[1]);
            selection = [selection[0], selection[0]];
        } else if (selection[0] > 0 && direction == -1) {
            source = source.substring(0, selection[0]-1) + source.substring(selection[1]);
            selection = [selection[0]-1, selection[0]-1];
        } else if (selection[0] < source.length && direction == 1) {
            source = source.substring(0, selection[0]) + source.substring(selection[1]+1);
            selection = [selection[0], selection[0]];
        }
    }

    function moveLeftRight(direction: 1 | -1) {
        if (selection[0] == selection[1]) {
            let newSelection = selection[0] + direction;
            newSelection = Math.min(newSelection, source.length);
            newSelection = Math.max(newSelection, 0);
            selection = [newSelection, newSelection];
            console.log("here-we-go", selection);
        } else {
            //TODO resolve longer selections
        }
    }

    function onClick(event: MouseEvent) {
        let target = event.target! as HTMLDivElement;
        target.focus();
    }

    function onKeydown(event: KeyboardEvent) {
        if (event.key.length == 1 && !event.altKey && !event.ctrlKey) {
            // Regular character
            insertCharacterIntoSelection(event.key);
        } else if (event.key == "Enter") {
            insertCharacterIntoSelection("\n");
        } else if (event.key == "Backspace") {
            // TODO SHIFT/CTRL
            deleteCharacterFromSelection(-1);
        } else if (event.key == "Delete") {
            // TODO SHIFT/CTRL
            deleteCharacterFromSelection(1);
        } else if (event.key == "ArrowLeft") {
            // TODO SHIFT/CTRL
            moveLeftRight(-1);
        } else if (event.key == "ArrowRight") {
            // TODO SHIFT/CTRL
            moveLeftRight(1);
        }

        // TODO arrow keys/ctrl+vim motions
    }
</script>

<!-- This is temporary and I don't want to be yelled at -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="themable-editor w-full h-full outline-none" tabindex="-1" on:keydown={onKeydown} on:click={onClick}>
    <div class="max-w-full mb-10">
        <MdRender {compiledData} />
    </div>
</div>