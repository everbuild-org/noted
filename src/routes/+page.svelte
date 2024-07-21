<script lang="ts">
  import Editor from "$lib/editor/Editor.svelte";

  let source = `# welcome to noted!
_The note-taking app for the 21<sup>st</sup> century._

We hope, you have a great time here in noted! This a test file and any modifications to it won't be saved. To get you started, please see the following syntactic examples:

- *Italic*
- _Italic_
- **Bold**
- ***Bold and Italic***
- \`Inline Code\`
- ~Strikethrough~

\`\`\`
There are also
multiline
preformatted
code blocks
\`\`\`

[Link](https://github.com/everbuild.org/noted)

Her-e we-h go-! Mari-o! :laugh:
`
</script>

<div class="max-h-screen overflow-auto">
  <div class="themable-editor-wrapper w-full h-full max-w-2xl mx-auto pt-4">
    <Editor bind:source/>
  </div>
</div>
