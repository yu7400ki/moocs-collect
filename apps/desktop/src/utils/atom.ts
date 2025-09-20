import { type Atom, type SetStateAction, atom } from "jotai";
import { loadable } from "jotai/utils";

export function unwrapPromise<Value>(promiseAtom: Atom<Value>) {
  const loadableAtom = loadable(promiseAtom);
  return atom<Value | Awaited<Value>>((get) => {
    const value = get(promiseAtom);
    const loadedValue = get(loadableAtom);
    return loadedValue.state === "hasData" ? loadedValue.data : value;
  });
}

export function atomWithDebounce<T>(
  initialValue: T,
  delayMilliseconds = 500,
  shouldDebounceOnReset = false,
) {
  const prevTimeoutAtom = atom<ReturnType<typeof setTimeout> | undefined>(
    undefined,
  );

  // DO NOT EXPORT currentValueAtom as using this atom to set state can cause
  // inconsistent state between currentValueAtom and debouncedValueAtom
  const _currentValueAtom = atom(initialValue);
  const isDebouncingAtom = atom(false);

  const debouncedValueAtom = atom(
    initialValue,
    (get, set, update: SetStateAction<T>) => {
      clearTimeout(get(prevTimeoutAtom));

      const prevValue = get(_currentValueAtom);
      const nextValue =
        typeof update === "function"
          ? (update as (prev: T) => T)(prevValue)
          : update;

      const onDebounceStart = () => {
        set(_currentValueAtom, nextValue);
        set(isDebouncingAtom, true);
      };

      const onDebounceEnd = () => {
        set(debouncedValueAtom, nextValue);
        set(isDebouncingAtom, false);
      };

      onDebounceStart();

      if (!shouldDebounceOnReset && nextValue === initialValue) {
        onDebounceEnd();
        return;
      }

      const nextTimeoutId = setTimeout(() => {
        onDebounceEnd();
      }, delayMilliseconds);

      // set previous timeout atom in case it needs to get cleared
      set(prevTimeoutAtom, nextTimeoutId);
    },
  );

  // exported atom setter to clear timeout if needed
  const clearTimeoutAtom = atom(null, (get, set, _arg) => {
    clearTimeout(get(prevTimeoutAtom));
    set(isDebouncingAtom, false);
  });

  return {
    currentValueAtom: atom((get) => get(_currentValueAtom)),
    isDebouncingAtom,
    clearTimeoutAtom,
    debouncedValueAtom,
  };
}
