import { purgeIndex as purgeIndexCommand } from "@/command/purge-index";

export async function purgeIndex() {
  return await purgeIndexCommand();
}
