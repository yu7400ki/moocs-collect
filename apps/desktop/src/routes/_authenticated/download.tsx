import { DownloadPage } from "@/features/download/pages/download";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_authenticated/download")({
  component: DownloadPage,
});
