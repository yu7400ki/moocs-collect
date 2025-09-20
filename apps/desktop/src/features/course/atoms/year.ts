import { getArchiveYears } from "@/command/get-archive-years";
import { unwrapPromise } from "@/utils/atom";
import { atom } from "jotai";

const internalAvailableYearsAtom = atom(async () => {
  return await getArchiveYears();
});

export const availableYearsAtom = unwrapPromise(internalAvailableYearsAtom);

export const yearAtom = atom<number | undefined>(undefined);
