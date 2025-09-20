import { Field } from "@ark-ui/react";
import { useAtom, useAtomValue } from "jotai";
import { CircleAlertIcon, FilterIcon, SearchIcon } from "lucide-react";
import { useMemo } from "react";
import { css, cx } from "styled-system/css";
import { Box, Circle } from "styled-system/jsx";
import { input } from "styled-system/recipes";
import { IconButton } from "@/components/ui/icon-button";
import { Popover } from "@/components/ui/popover";
import { Spinner } from "@/components/ui/spinner";
import { Tooltip } from "@/components/ui/tooltip";
import { TreeSelect, type TreeSelectNode } from "@/components/ui/tree-select";
import {
  facetFilterAtom,
  groupedRecordedCoursesAtom,
  isSearchingAtom,
  searchErrorAtom,
  searchQueryAtom,
} from "../atoms/search";

export function SearchForm() {
  const [currentQuery, setQuery] = useAtom(searchQueryAtom);
  const isSearching = useAtomValue(isSearchingAtom);
  const error = useAtomValue(searchErrorAtom);
  const recordedCourses = useAtomValue(groupedRecordedCoursesAtom);
  const [selected, setSelected] = useAtom(facetFilterAtom);

  const filter = useMemo<TreeSelectNode[]>(() => {
    const result: TreeSelectNode[] = [];
    for (const [year, courses] of recordedCourses.entries()) {
      result.push({
        id: `/${year}`,
        label: `${year}年度`,
        children: courses.map((course) => ({
          id: `/${year}/${course.slug}`,
          label: course.name,
        })),
      });
    }
    return result;
  }, [recordedCourses]);

  return (
    <Box w="full" display="grid" gridTemplateColumns="1fr auto" gap="2">
      <Field.Root>
        <div
          className={cx(
            input(),
            css({
              px: 0,
              display: "grid",
              gridTemplateColumns: "auto 1fr auto",
              gridTemplateRows: "1fr",
              alignItems: "center",
              _focusWithin: {
                borderColor: "colorPalette.default",
                boxShadow: "0 0 0 1px var(--colors-color-palette-default)",
              },
              "&:has(:invalid, [data-invalid], [aria-invalid=true])": {
                borderColor: "fg.error",
                _focusWithin: {
                  borderColor: "fg.error",
                  boxShadow: "0 0 0 1px var(--colors-border-error)",
                },
              },
              "& :where(svg)": {
                fontSize: "1.1em",
                width: "1.1em",
                height: "1.1em",
              },
            }),
          )}
        >
          <SearchIcon
            className={css({
              gridColumn: "1",
              gridRow: "1",
              ml: 3,
              color: "fg.muted",
              pointerEvents: "none",
            })}
          />
          <Field.Input
            value={currentQuery}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="スライド内容を検索..."
            className={css({
              w: "full",
              h: "full",
              gridColumn: "1 / -1",
              gridRow: "1",
              px: 10,
              _focus: {
                outline: "none",
              },
            })}
            aria-invalid={error !== null}
          />
          {isSearching ? (
            <Spinner
              size="sm"
              mr="3"
              gridColumn="3"
              gridRow="1"
              pointerEvents="none"
            />
          ) : error !== null ? (
            <Tooltip.Root>
              <Tooltip.Trigger
                asChild
                accentColor="red"
                color="fg.error"
                mr="2"
                gridColumn="3"
                gridRow="1"
              >
                <CircleAlertIcon />
              </Tooltip.Trigger>
              <Tooltip.Positioner>
                <Tooltip.Arrow>
                  <Tooltip.ArrowTip />
                </Tooltip.Arrow>
                <Tooltip.Content>{error}</Tooltip.Content>
              </Tooltip.Positioner>
            </Tooltip.Root>
          ) : null}
        </div>
      </Field.Root>
      <Popover.Root
        positioning={{
          placement: "bottom-start",
        }}
      >
        <Popover.Trigger asChild>
          <IconButton variant="outline" position="relative">
            <FilterIcon />
            {selected.length > 0 && (
              <Circle
                colorPalette="cyan"
                position="absolute"
                top="-1"
                right="-1"
                width="2"
                height="2"
                bg="colorPalette.default"
              />
            )}
          </IconButton>
        </Popover.Trigger>
        <Popover.Positioner>
          <Popover.Content p="2">
            <TreeSelect
              data={filter}
              value={selected}
              onValueChange={setSelected}
            />
          </Popover.Content>
        </Popover.Positioner>
      </Popover.Root>
    </Box>
  );
}
