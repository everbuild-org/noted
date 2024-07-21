export type Element = {
	elementStart: number;
	elementEnd: number;
	rawText: string;
};

export type StyledTextElement = {
	kind: 'text';
	bold: boolean;
	underline: boolean;
	strikethrough: boolean;
	italic: boolean;
} & Element;

export type ListElement = {
	kind: 'listdot';
} & Element;

export type MonospaceElement = {
	kind: 'mono';
} & Element;

export type Cursor = {
	kind: 'cursor';
} & Element;

export type Header = {
	kind: 'header';
	depth: number;
	blocks: MarkdownTextlineBlock;
} & Element;

export type InlineCode = {
	kind: 'inlinecode';
} & Element;

export type MarkdownTextElement =
	| StyledTextElement
	| MonospaceElement
	| Cursor
	| ListElement
	| Header
	| InlineCode;
export type MarkdownTextElementKind = MarkdownTextElement['kind'];
export type MarkdownTextlineBlock = MarkdownTextElement[];
export type Block = MarkdownTextlineBlock;
export type CompilationResult = Block[];

export function compileLine(source: string, from: number, to: number): MarkdownTextlineBlock {
	let elements: MarkdownTextlineBlock = [];

	let currentMode = {
		bold: false,
		strikethrough: false,
		underline: false,
		italic: false
	};

	let isCodeBlock = false;
    let skip = false;

	let fromPostion = 0;
	let toPosition = 0;
	let currentSegmentWithoutFormatting = '';

	// no italic _italic_

	function pushElement() {
		elements.push({
			kind: 'text',
			elementStart: from + fromPostion,
			elementEnd: from + toPosition,
			rawText: currentSegmentWithoutFormatting,
			...currentMode
		});

		currentSegmentWithoutFormatting = '';
	}

	for (let char of source) {
        if (skip) {
            skip = false;
            continue;
        }
		if (char == '_' && !isCodeBlock) {
			if (!currentMode.italic) {
				pushElement();
				currentMode.italic = true;
				toPosition++;
				fromPostion = toPosition;
			} else {
				toPosition++;
				pushElement();
				currentMode.italic = false;
				fromPostion = toPosition;
			}
		} else if (char == '~' && !isCodeBlock) {
			if (!currentMode.strikethrough) {
				pushElement();
				currentMode.strikethrough = true;
				toPosition++;
				fromPostion = toPosition;
			} else {
				toPosition++;
				pushElement();
				currentMode.strikethrough = false;
				fromPostion = toPosition;
			}
		} else if (char == '*' && !isCodeBlock) {
            console.log(source, char, toPosition, source[toPosition])
			if (toPosition + 1 < source.length && source[toPosition + 1] == '*') {
                console.log("boldthing");
                
                if (!currentMode.bold) {
					pushElement();
					currentMode.bold = true;
					toPosition+=2;
                    skip = true;
					fromPostion = toPosition;
				} else {
					toPosition+=2;
					pushElement();
                    skip = true;
					currentMode.bold = false;
					fromPostion = toPosition;
				}
			} else {
                console.log("italic");
                
				if (!currentMode.italic) {
					pushElement();
					currentMode.italic = true;
					toPosition++;
					fromPostion = toPosition;
				} else {
					toPosition++;
					pushElement();
					currentMode.italic = false;
					fromPostion = toPosition;
				}
			}
		} else if (char == '`') {
			if (!isCodeBlock) {
				pushElement();
				isCodeBlock = true;
				toPosition++;
				fromPostion = toPosition;
			} else {
				toPosition++;
				elements.push({
					kind: 'inlinecode',
					elementStart: from + fromPostion,
					elementEnd: from + toPosition,
					rawText: currentSegmentWithoutFormatting
				});

				currentSegmentWithoutFormatting = '';
				isCodeBlock = false;
				fromPostion = toPosition;
			}
		} else {
			toPosition++;
			currentSegmentWithoutFormatting += char;
		}
	}

	if (fromPostion != source.length) {
		toPosition = source.length;
		pushElement();
	}

	return elements;
}

export function compileSource(source: string, selection: [number, number]): CompilationResult {
	const [selectionStart, selectionEnd] = selection;
	let position = 0;

	let blocks: Block[] = [];

	for (let line of source.split('\n')) {
		line = line + '\n';

		let elements: MarkdownTextlineBlock = [];

		let length = line.length;
		// TODO selection
		let isSelectionInside = position <= selectionStart && selectionStart < position + length;
		//|| (position < selectionEnd && selectionEnd < position + length);

		if (isSelectionInside) {
			// Resolve cursor and build two Monospace-Elements
			// TODO selection
			let selectionStartRelative = selectionStart - position;
			let before = line.substring(0, selectionStartRelative);
			let after = line.substring(selectionStartRelative);

			elements = [
				{
					kind: 'mono',
					elementStart: position,
					elementEnd: selectionStart,
					rawText: before
				},
				{
					kind: 'cursor',
					elementStart: selectionStart,
					elementEnd: selectionStart,
					rawText: ''
				},
				{
					kind: 'mono',
					elementStart: selectionStart,
					elementEnd: position + length,
					rawText: after
				}
			];
		} else if (line.length > 1) {
			let firstCharacter = line[0];

			if (firstCharacter == '#') {
				let depth = 0;
				let chars = 0;
				for (let char of line) {
					if (depth > 6) break;
					if (char != '#' && char != ' ') break;
					if (char == '#') depth += 1;
					chars += 1;
				}

				let from = position + chars;
				let to = position + length;

				elements.push({
					kind: 'header',
					elementStart: position,
					elementEnd: to,
					rawText: line,
					depth,
					blocks: compileLine(line.substring(chars), from, to)
				});
			} else if (firstCharacter == '-' && line[1] == ' ') {
				let from = position + 2;
				let to = position + length;
				elements.push({
					kind: 'listdot',
					elementStart: position,
					elementEnd: position + 2,
					rawText: '- '
				});
				elements.push(...compileLine(line.substring(2), from, to));
			} else {
				let from = position;
				let to = position + length;
				elements = compileLine(line, from, to);
			}
		}

		blocks.push(elements);
		// TODO detect special blocks (like lists and code blocks)

		position += length;
	}

	return blocks;
}
