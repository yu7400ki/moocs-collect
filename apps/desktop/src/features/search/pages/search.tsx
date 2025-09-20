import { css } from "styled-system/css";
import { Box, Divider } from "styled-system/jsx";
import { SearchForm } from "../components/search-form";
import { SearchResults } from "../components/search-results";

export function SearchPage() {
  return (
    <main
      className={css({
        display: "grid",
        gridTemplateRows: "auto auto minmax(0, 1fr)",
      })}
    >
      <Box p="6">
        <SearchForm />
      </Box>
      <Divider orientation="horizontal" />
      <Box p="6" overflow="auto">
        <SearchResults />
      </Box>
    </main>
  );
}
