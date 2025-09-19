import { Text } from "@/components/ui/text";
import { openPath } from "@tauri-apps/plugin-opener";
import { ChevronRightIcon, ExternalLinkIcon } from "lucide-react";
import { useCallback, useTransition } from "react";
import { css } from "styled-system/css";
import { Box, HStack, VStack } from "styled-system/jsx";
import type { HighlightedText, SlideSearchEntry } from "../services/search";

interface SearchResultItemProps {
  entry: SlideSearchEntry;
}

export function SearchResultItem({ entry }: SearchResultItemProps) {
  const [isPending, startTransition] = useTransition();

  const handleOpenSlide = useCallback(() => {
    startTransition(() => {
      openPath(entry.downloadPath);
    });
  }, [entry.downloadPath]);

  const renderHighlightedContent = (highlights: HighlightedText[]) => {
    return highlights.map((highlight, idx) =>
      highlight.isHighlighted ? (
        <Text
          as="mark"
          key={`${highlight.text}-${idx}-${highlight.isHighlighted}`}
          bg="colorPalette.default"
          color="colorPalette.fg"
          px="1"
          py="0.5"
          mx="0.5"
          rounded="l1"
          fontWeight="medium"
        >
          {highlight.text}
        </Text>
      ) : (
        <Text
          as="span"
          key={`${highlight.text}-${idx}-${highlight.isHighlighted}`}
        >
          {highlight.text}
        </Text>
      ),
    );
  };

  return (
    <button
      type="button"
      className={css({
        p: 4,
        bg: "bg.default",
        border: "1px solid",
        borderColor: "border.subtle",
        rounded: "l2",
        transition: "background-color 0.2s",
        cursor: "pointer",
        textAlign: "left",
        _hover: {
          bg: "bg.canvas",
        },
        _focusVisible: {
          outline: "2px solid",
          outlineColor: "colorPalette.default",
          outlineOffset: "2px",
        },
      })}
      onClick={handleOpenSlide}
      disabled={isPending}
    >
      <VStack gap="3" alignItems="stretch">
        <VStack gap="1" alignItems="flex-start">
          <HStack gap="2" flexWrap="wrap" alignItems="center" w="full">
            <Box fontSize="xs" fontWeight="semibold">
              {entry.year}年度
            </Box>
            <Box color="fg.muted" fontSize="sm">
              {entry.courseName}
            </Box>
            <ExternalLinkIcon
              className={css({
                ml: "auto",
                h: "1em",
                w: "1em",
              })}
            />
          </HStack>
          <HStack gap="2" fontSize="sm">
            <Box color="fg.muted">{entry.lectureName}</Box>
            <ChevronRightIcon
              className={css({ color: "fg.muted", w: "1em", h: "1em" })}
            />
            <Box color="fg.default">{entry.pageName}</Box>
          </HStack>
        </VStack>
        <Box
          className={css({
            p: 3,
            bg: "bg.subtle",
            rounded: "l1",
            borderWidth: "1px",
          })}
        >
          <Text fontSize="sm" lineHeight="relaxed" color="fg.default">
            {entry.searchResult.highlightedContent.length > 0
              ? renderHighlightedContent(entry.searchResult.highlightedContent)
              : entry.searchResult.contentSnippet}
          </Text>
        </Box>
      </VStack>
    </button>
  );
}
