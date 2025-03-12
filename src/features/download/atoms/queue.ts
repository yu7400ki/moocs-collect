import type { Page } from "@/features/course/schemas/page";
import PQueue from "p-queue";
import { downloadSlides } from "../services/download-slides";
import { atom } from "jotai";

const queue = new PQueue({ concurrency: 5 });

const pendingQueue = atom(new Set<Page>());
const runningQueue = atom(new Set<Page>());
const completedQueue = atom(new Set<Page>());

export const queueAtom = atom(
  (get) => {
    return {
      pending: get(pendingQueue),
      running: get(runningQueue),
      completed: get(completedQueue),
    };
  },
  (_, set, page: Page) => {
    set(pendingQueue, (prev) => {
      const next = new Set(prev);
      next.add(page);
      return next;
    });
    queue.add(async () => {
      set(pendingQueue, (prev) => {
        const next = new Set(prev);
        next.delete(page);
        return next;
      });
      set(runningQueue, (prev) => {
        const next = new Set(prev);
        next.add(page);
        return next;
      });
      await downloadSlides(page);
      set(runningQueue, (prev) => {
        const next = new Set(prev);
        next.delete(page);
        return next;
      });
      set(completedQueue, (prev) => {
        const next = new Set(prev);
        next.add(page);
        return next;
      });
    });
  },
);
