import { Text } from "@/components/ui/text";
import { ChevronRightIcon } from "lucide-react";
import { css } from "styled-system/css";
import { Box, HStack, VStack } from "styled-system/jsx";
import type { HighlightedText, SearchResult } from "../services/search";

interface SearchResultItemProps {
  result: SearchResult;
}

export function SearchResultItem({ result }: SearchResultItemProps) {
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
    <Box
      p="4"
      bg="bg.default"
      border="1px solid"
      borderColor="border.subtle"
      rounded="l2"
      transition="background-color 0.2s"
      _hover={{
        bg: "bg.canvas",
      }}
    >
      <VStack gap="3" alignItems="stretch">
        <VStack gap="1" alignItems="flex-start">
          <HStack gap="2" flexWrap="wrap" alignItems="center">
            <Box fontSize="xs" fontWeight="semibold">
              {result.year}年度
            </Box>
            <Box color="fg.muted" fontSize="sm">
              {result.course}
            </Box>
          </HStack>
          <HStack gap="2" fontSize="sm">
            <Box color="fg.muted">{result.lecture}</Box>
            <ChevronRightIcon
              className={css({ color: "fg.muted", w: "1em", h: "1em" })}
            />
            <Box color="fg.default">{result.page}</Box>
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
            {result.highlightedContent.length > 0
              ? renderHighlightedContent(result.highlightedContent)
              : result.contentSnippet}
          </Text>
        </Box>
      </VStack>
    </Box>
  );
}
