import Preview from "./Preview.svelte";

let myElement: HTMLElement | null = document?.getElementById("preview");
let myTarget: Document | Element | ShadowRoot = myElement as
  | Document
  | Element
  | ShadowRoot;
const preview = new Preview({
  target: myTarget,
});

export default preview;
