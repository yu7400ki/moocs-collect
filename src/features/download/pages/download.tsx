import { Tabs } from "@/components/ui/tabs";
import { Completed } from "../components/completed";
import { InQueue } from "../components/in-queue";

export function DownloadPage() {
  const options = [
    { id: "in-queue", label: "処理待ち" },
    { id: "completed", label: "完了" },
  ];

  return (
    <Tabs.Root defaultValue="in-queue">
      <Tabs.List>
        {options.map((option) => (
          <Tabs.Trigger
            key={option.id}
            value={option.id}
            py="0"
            px="6"
            fontWeight="normal"
          >
            {option.label}
          </Tabs.Trigger>
        ))}
        <Tabs.Indicator />
      </Tabs.List>
      <Tabs.Content value="in-queue" overflowY="auto" pt="0">
        <InQueue />
      </Tabs.Content>
      <Tabs.Content value="completed" overflowY="auto" pt="0">
        <Completed />
      </Tabs.Content>
    </Tabs.Root>
  );
}
