import {
  type InvokeArgs,
  type InvokeOptions,
  invoke,
} from "@tauri-apps/api/core";

type Command<Args extends InvokeArgs | undefined, Out> = Args extends undefined
  ? (args?: Args, options?: InvokeOptions) => Promise<Out>
  : (args: Args, options?: InvokeOptions) => Promise<Out>;

export function createCommand<Args extends InvokeArgs | undefined, Out>(
  command: string,
): Command<Args, Out> {
  return (async (input?: Args, options?: InvokeOptions) => {
    return await invoke<Out>(command, input, options);
  }) as Command<Args, Out>;
}
