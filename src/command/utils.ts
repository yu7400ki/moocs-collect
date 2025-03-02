import { type InvokeArgs, invoke } from "@tauri-apps/api/core";

export function createCommand<Args extends InvokeArgs, Out>(command: string) {
  return async (input: Args) => {
    return await invoke<Out>(command, input);
  };
}
