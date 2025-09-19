import type { Course } from "@/features/course/schemas/course";
import type { Lecture } from "@/features/course/schemas/lecture";
import type { Page } from "@/features/course/schemas/page";
import { recordedCoursesAtom } from "@/features/search/atoms/search";
import { atom } from "jotai";
import PQueue from "p-queue";
import { downloadSlides } from "../services/download-slides";

export type DownloadItem = Page & {
  course: Course;
  lecture: Lecture;
};

const queue = new PQueue({ concurrency: 5 });

const pendingQueue = atom(new Set<DownloadItem>());
const runningQueue = atom(new Set<DownloadItem>());
const completedQueue = atom(new Set<DownloadItem>());
const errorQueue = atom(new Set<DownloadItem>());

export const queueAtom = atom(
  (get) => {
    return {
      pending: get(pendingQueue),
      running: get(runningQueue),
      completed: get(completedQueue),
      error: get(errorQueue),
    };
  },
  (_, set, downloadItem: DownloadItem) => {
    set(pendingQueue, (prev) => {
      const next = new Set(prev);
      next.add(downloadItem);
      return next;
    });
    queue.add(async () => {
      set(pendingQueue, (prev) => {
        const next = new Set(prev);
        next.delete(downloadItem);
        return next;
      });
      set(runningQueue, (prev) => {
        const next = new Set(prev);
        next.add(downloadItem);
        return next;
      });
      try {
        await downloadSlides(downloadItem);
        set(completedQueue, (prev) => {
          const next = new Set(prev);
          next.add(downloadItem);
          return next;
        });
        set(recordedCoursesAtom);
      } catch (error) {
        set(errorQueue, (prev) => {
          const next = new Set(prev);
          next.add(downloadItem);
          return next;
        });
      }
      set(runningQueue, (prev) => {
        const next = new Set(prev);
        next.delete(downloadItem);
        return next;
      });
    });
  },
);

export const retryAtom = atom(null, (_, set, downloadItem: DownloadItem) => {
  set(errorQueue, (prev) => {
    const next = new Set(prev);
    next.delete(downloadItem);
    return next;
  });
  set(queueAtom, downloadItem);
});
