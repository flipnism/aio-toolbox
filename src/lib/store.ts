import { listen } from "@tauri-apps/api/event";
type Subscriber<T> = (value: T) => void;
type Updater<T> = (value: T) => T;

export function writable<T>(value: T) {
  const subscribers = new Set<Subscriber<T>>();
  function set(newValue: T) {
    value = newValue;
    subscribers.forEach((subscriber) => subscriber(value));
  }
  function update(updater: Updater<T>) {
    set(updater(value));
  }
  function subscribe(subscriber: Subscriber<T>) {
    subscribers.add(subscriber);
    return () => {
      subscribers.delete(subscriber);
    };
  }
  return { set, update, subscribe };
}

export const layer_index = writable(0);
export const tags = writable([]);
export const global_settings = writable([]);

export const color_set = [
  ["#c1111f", "#fff"],
  ["#fc8500", "#0f1a20"],
  ["#7227b6", "#fff"],
];

listen("mod-event", (e) => {
  const result: any = e.payload;
  layer_index.update(() => result.layer_index);
});
