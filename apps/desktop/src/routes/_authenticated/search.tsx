import { createFileRoute } from "@tanstack/react-router";
import { SearchPage } from "@/features/search/pages/search";

export const Route = createFileRoute("/_authenticated/search")({
  component: SearchPage,
});
