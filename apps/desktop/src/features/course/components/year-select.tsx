import { useAtom, useAtomValue } from "jotai";
import { CheckIcon, ChevronsUpDownIcon } from "lucide-react";
import { createListCollection, Select } from "@/components/ui/select";
import { availableYearsAtom, yearAtom } from "../atoms/year";

export function YearSelect(
  props: Omit<React.ComponentProps<"div">, "defaultValue">,
) {
  const years = useAtomValue(availableYearsAtom);
  const [selectedYear, setSelectedYear] = useAtom(yearAtom);

  const collection = createListCollection({
    items: years.map((year) => ({
      value: year.toString(),
      label: `${year}年度`,
    })),
  });

  return (
    <Select.Root
      {...props}
      size="sm"
      width="auto"
      positioning={{ sameWidth: true }}
      collection={collection}
      value={selectedYear ? [selectedYear.toString()] : []}
      onValueChange={(detail) => {
        const year = detail.value[0]
          ? Number.parseInt(detail.value[0], 10)
          : undefined;
        setSelectedYear(year);
      }}
    >
      <Select.Control>
        <Select.Trigger>
          <Select.ValueText placeholder="年度を選択" />
          <ChevronsUpDownIcon />
        </Select.Trigger>
      </Select.Control>
      <Select.Positioner>
        <Select.Content>
          <Select.ItemGroup>
            {collection.items.map((item) => (
              <Select.Item key={item.value} item={item}>
                <Select.ItemText>{item.label}</Select.ItemText>
                <Select.ItemIndicator>
                  <CheckIcon />
                </Select.ItemIndicator>
              </Select.Item>
            ))}
          </Select.ItemGroup>
        </Select.Content>
      </Select.Positioner>
    </Select.Root>
  );
}
