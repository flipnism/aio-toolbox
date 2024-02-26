import Hotkeys from "./Hotkeys.svelte";
let myElement: HTMLElement | null = document?.getElementById("app");
let myTarget: Document | Element | ShadowRoot = myElement as
  | Document
  | Element
  | ShadowRoot;
const app = new Hotkeys({
  target: myTarget,
});

export default app;
