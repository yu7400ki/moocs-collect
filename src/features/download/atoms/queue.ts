import type { Page } from "@/features/course/schemas/page";
import { atom } from "jotai";
import PQueue from "p-queue";
import { downloadSlides } from "../services/download-slides";

const queue = new PQueue({ concurrency: 5 });

const pendingQueue = atom(new Set<Page>());
const runningQueue = atom(new Set<Page>());
const completedQueue = atom(new Set<Page>());
const errorQueue = atom(new Set<Page>());

export const queueAtom = atom(
  (get) => {
    return {
      pending: get(pendingQueue),
      running: get(runningQueue),
      completed: get(completedQueue),
      error: get(errorQueue),
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
      try {
        await downloadSlides(page);
        set(completedQueue, (prev) => {
          const next = new Set(prev);
          next.add(page);
          return next;
        });
      } catch (error) {
        set(errorQueue, (prev) => {
          const next = new Set(prev);
          next.add(page);
          return next;
        });
      }
      set(runningQueue, (prev) => {
        const next = new Set(prev);
        next.delete(page);
        return next;
      });
    });
  },
);
