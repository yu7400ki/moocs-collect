import { Spinner } from "@/components/ui/spinner";
import { Tooltip } from "@/components/ui/tooltip";
import { Field } from "@ark-ui/react";
import { useAtom, useAtomValue } from "jotai";
import { CircleAlertIcon, SearchIcon } from "lucide-react";
import { css, cx } from "styled-system/css";
import { Box } from "styled-system/jsx";
import { input } from "styled-system/recipes";
import {
  isSearchingAtom,
  searchErrorAtom,
  searchQueryAtom,
} from "../atoms/search";

export function SearchForm() {
  const [currentQuery, setQuery] = useAtom(searchQueryAtom);
  const isSearching = useAtomValue(isSearchingAtom);
  const error = useAtomValue(searchErrorAtom);

  return (
    <Box w="full">
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
    </Box>
  );
}
