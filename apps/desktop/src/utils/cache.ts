import { type MemoizeCache, memoize } from "es-toolkit/function";

// biome-ignore lint/suspicious/noExplicitAny:
export function memoizeAsync<F extends (...args: any) => Promise<any>>(
  fn: F,
  options?: {
    getCacheKey?: (args: Parameters<F>[0]) => unknown;
  },
): ((...args: Parameters<F>) => ReturnType<F> | Awaited<ReturnType<F>>) & {
  // biome-ignore lint/suspicious/noExplicitAny:
  cache: MemoizeCache<any, ReturnType<F> | Promise<ReturnType<F>>>;
} {
  // @ts-ignore
  return memoize(fn, {
    cache: new PromiseCache(),
    ...options,
  });
}

export class PromiseCache<K, T> implements MemoizeCache<K, T | Promise<T>> {
  private cache = new Map<K, T | Promise<T>>();

  set(key: K, value: T | Promise<T>): void {
    this.cache.set(key, value);
    if (value instanceof Promise) {
      value
        .then((resolved) => {
          const current = this.cache.get(key);
          current === value && this.cache.set(key, resolved);
        })
        .catch(() => {
          const current = this.cache.get(key);
          current === value && this.cache.delete(key);
        });
    }
  }
  get(key: K): T | Promise<T> | undefined {
    return this.cache.get(key);
  }
  has(key: K): boolean {
    return this.cache.has(key);
  }
  delete(key: K): boolean {
    return this.cache.delete(key);
  }
  clear(): void {
    this.cache.clear();
  }
  get size(): number {
    return this.cache.size;
  }
}
