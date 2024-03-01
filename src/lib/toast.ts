import {
  writable,
  type Writable,
  type Updater,
  type Subscriber,
  type Unsubscriber,
  type Invalidator,
} from "svelte/store";

interface Toast {
  id: number;
  type: string;
  dismissible: boolean;
  timeout: number;
  message?: string;
  title?: string;
}

interface ToastStore {
  subscribe: (
    run: Subscriber<Toast[]>,
    invalidate?: Invalidator<Toast[]>
  ) => Unsubscriber;
  set: (value: Toast[]) => void;
  update: (fn: Updater<Toast[]>) => void;
}

export const toasts: Writable<Toast[]> = writable([]);

export const addToast = (toast: Partial<Toast>): void => {
  // Create a unique ID so we can easily find/remove it
  // if it is dismissible/has a timeout.
  const id = Math.floor(Math.random() * 10000);

  // Setup some sensible defaults for a toast.
  const defaults: Toast = {
    id,
    type: "info",
    dismissible: true,
    timeout: 3000,
  };

  // Push the toast to the top of the list of toasts
  toasts.update((all) => [{ ...defaults, ...toast }, ...all]);

  // If toast is dismissible, dismiss it after "timeout" amount of time.
  if (toast.timeout) setTimeout(() => dismissToast(id), toast.timeout);
};

export const dismissToast = (id: number): void => {
  toasts.update((all) => all.filter((t) => t.id !== id));
};
