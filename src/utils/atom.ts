import { type Atom, atom } from "jotai";
import { loadable } from "jotai/utils";

export function unwrapPromise<Value>(promiseAtom: Atom<Value>) {
  const loadableAtom = loadable(promiseAtom);
  return atom((get) => {
    const value = get(promiseAtom);
    const loadedValue = get(loadableAtom);
    return loadedValue.state === "hasData" ? loadedValue.data : value;
  });
}
