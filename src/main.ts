import ToolbarPanel from "./ToolbarPanel.svelte";

let myElement: HTMLElement | null = document?.getElementById("app");
let myTarget: Document | Element | ShadowRoot = myElement as
  | Document
  | Element
  | ShadowRoot;
const app = new ToolbarPanel({
  target: myTarget,
});

export default app;
