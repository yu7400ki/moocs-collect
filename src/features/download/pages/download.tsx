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
      <Tabs.Content value="in-queue" overflowY="auto" pt="0" h="full">
        <InQueue />
      </Tabs.Content>
      <Tabs.Content value="completed" overflowY="auto" pt="0" h="full">
        <Completed />
      </Tabs.Content>
    </Tabs.Root>
  );
}
