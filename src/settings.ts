import Settings from "./Settings.svelte";
let myElement: HTMLElement | null = document?.getElementById("settings");
let myTarget: Document | Element | ShadowRoot = myElement as
  | Document
  | Element
  | ShadowRoot;
const settings = new Settings({
  target: myTarget,
});

export default settings;
