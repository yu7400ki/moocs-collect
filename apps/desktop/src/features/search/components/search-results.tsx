import { useAtomValue } from "jotai";
import { Box, Flex, VStack } from "styled-system/jsx";
import { Text } from "@/components/ui/text";
import { searchResultsAtom } from "../atoms/search";
import { SearchResultItem } from "./search-result-item";

export function SearchResults() {
  const entries = useAtomValue(searchResultsAtom);

  if (entries.length === 0) {
    return (
      <Box p="8" textAlign="center" color="fg.muted">
        <p>検索結果が見つかりませんでした</p>
      </Box>
    );
  }

  return (
    <Box>
      <Flex
        justify="space-between"
        align="flex-end"
        mb="4"
        borderBottom="1px solid"
        borderColor="border.subtle"
        pb="2"
      >
        <Text as="h3" fontSize="lg" fontWeight="semibold">
          検索結果
        </Text>
        <Text color="fg.muted" fontSize="sm">
          {entries.length}件見つかりました
        </Text>
      </Flex>
      <VStack gap="3" alignItems="stretch">
        {entries.map((entry) => (
          <SearchResultItem key={entry.searchResult.facet} entry={entry} />
        ))}
      </VStack>
    </Box>
  );
}
