<script lang="ts">
    import {compileSource} from "./compiler";
    import MdRender from "./render/MdRender.svelte";

    export let source: string;
    if (!source) source = "";

    let selection: [number, number] = [0, 0]

    $: compiledData = compileSource(source, selection);

    function generateNonce() {
        return Math.random().toFixed(5).replace(".", "")
    }

    const nonce = generateNonce();

    function insertCharacterIntoSelection(char: string) {
        source = source.substring(0, selection[0]) + char + source.substring(selection[1]);
        selection = [selection[0] + 1, selection[0] + 1];
    }

    function deleteCharacterFromSelection(direction: 1 | -1) {
        if (selection[0] < selection[1]) {
            source = source.substring(0, selection[0]) + source.substring(selection[1]);
            selection = [selection[0], selection[0]];
        } else if (selection[0] > 0 && direction == -1) {
            source = source.substring(0, selection[0] - 1) + source.substring(selection[1]);
            selection = [selection[0] - 1, selection[0] - 1];
        } else if (selection[0] < source.length && direction == 1) {
            source = source.substring(0, selection[0]) + source.substring(selection[1] + 1);
            selection = [selection[0], selection[0]];
        }
    }

    function moveLeftRight(direction: 1 | -1) {
        if (selection[0] == selection[1]) {
            let newSelection = selection[0] + direction;
            newSelection = Math.min(newSelection, source.length);
            newSelection = Math.max(newSelection, 0);
            selection = [newSelection, newSelection];
        } else {
            //TODO resolve longer selections
        }
    }

    function simpleCursorMoveUpDown(direction: 1 | -1) {
        let prev = 0;
        let next = source.length;
        let current = 0;
        for (let i = 0; i < source.length; i++) {
            if (i < selection[0]) {
                if (source[i] !== "\n") continue;
                prev = current;
                current = i;
            } else {
                if (source[i] !== "\n") continue;

                next = i;
                break;
            }
        }

        if (direction < 0) {
            selection = [prev, prev];
        } else {
            next = Math.min(next+1, source.length);
            selection = [next, next];
        }
    }

    function visualCursorMoveUpDown(direction: 1 | -1) {
        let currentElement = document.querySelector(`#editor-${nonce} [data-pos="${selection[0]}"]`)
        if (!currentElement) {
            simpleCursorMoveUpDown(direction)
            return
        }

        let rect = currentElement.getBoundingClientRect()
        let elements = document.elementsFromPoint(rect.x, rect.y+rect.height+5).filter((el) => Object.getOwnPropertyNames((el as HTMLElement).dataset).includes("pos"))
        if (elements.length == 0) {
            simpleCursorMoveUpDown(direction)
            return;
        }

        let element = elements[0] as HTMLElement;
        let pos = parseInt(element.dataset.pos as string);

        if (selection[0] == selection[1]) {
            selection = [pos-1, pos-1]
        } else {
            // TODO selections
        }
    }

    function moveUpDown(direction: 1 | -1) {
        if (selection[0] == selection[1]) {
            visualCursorMoveUpDown(direction)
        } else {
            //TODO selections
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
        } else if (event.key == "ArrowUp") {
            // TODO SHIFT
            moveUpDown(-1);
        } else if (event.key == "ArrowDown") {
            // TODO SHIFT
            moveUpDown(1);
        }

        // TODO arrow keys/ctrl+vim motions
    }
</script>

<!-- This is temporary and I don't want to be yelled at -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="themable-editor w-full h-full outline-none border border-red-500" tabindex="-1" on:keydown={onKeydown} on:click={onClick} id="editor-{nonce}">
    <div class="max-w-full mb-10">
        <MdRender {compiledData}/>
    </div>
</div>