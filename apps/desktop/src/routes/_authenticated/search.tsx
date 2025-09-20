import { SearchPage } from "@/features/search/pages/search";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_authenticated/search")({
  component: SearchPage,
});
