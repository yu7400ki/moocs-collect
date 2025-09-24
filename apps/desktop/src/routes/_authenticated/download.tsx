import { createFileRoute } from "@tanstack/react-router";
import { DownloadPage } from "@/features/download/pages/download";

export const Route = createFileRoute("/_authenticated/download")({
  component: DownloadPage,
});
