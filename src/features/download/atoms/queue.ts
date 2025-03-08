import type { Page } from "@/features/course/schemas/page";
import { atomWithProxy } from "jotai-valtio";
import PQueue from "p-queue";
import { proxySet } from "valtio/utils";
import { proxy } from "valtio/vanilla";

const queue = new PQueue({ concurrency: 10 });

const queueState = proxy({
  pending: proxySet<Page>(),
  running: proxySet<Page>(),
  completed: proxySet<Page>(),
});

export async function enqueue(page: Page) {
  queueState.pending.add(page);
  await queue.add(async () => {
    queueState.pending.delete(page);
    queueState.running.add(page);
    console.log("Start Processing", page);
    await new Promise((resolve) => setTimeout(resolve, 3000));
    console.log("End Processing", page);
    queueState.running.delete(page);
    queueState.completed.add(page);
  });
}

export const queueAtom = atomWithProxy(queueState, { sync: true });
