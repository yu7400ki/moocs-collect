import { Tabs } from "@/components/ui/tabs";
import { useAtomValue } from "jotai";
import { useState } from "react";
import { queueAtom } from "../atoms/queue";
import { Completed } from "../components/completed";
import { Errors } from "../components/error";
import { InQueue } from "../components/in-queue";

const panels = {
  "in-queue": InQueue,
  completed: Completed,
  error: Errors,
};

export function DownloadPage() {
  const options = [
    { id: "in-queue", label: "処理待ち" },
    { id: "completed", label: "完了" },
  ];
  const { error } = useAtomValue(queueAtom);
  if (error.size > 0) {
    options.push({ id: "error", label: "エラー" });
  }
  const [_panel, setPanel] = useState("in-queue");

  let panel = _panel;
  if (!options.some((option) => option.id === panel)) {
    panel = options[0].id;
  }

  return (
    <Tabs.Root value={panel} onValueChange={(e) => setPanel(e.value)}>
      <Tabs.List px="4" gap="4">
        {options.map((option) => (
          <Tabs.Trigger
            key={option.id}
            value={option.id}
            py="0"
            px="4"
            fontWeight="normal"
          >
            {option.label}
          </Tabs.Trigger>
        ))}
        <Tabs.Indicator />
      </Tabs.List>
      {Object.entries(panels).map(([key, Component]) => (
        <Tabs.Content key={key} value={key} overflowY="auto" pt="0" h="full">
          <Component />
        </Tabs.Content>
      ))}
    </Tabs.Root>
  );
}
