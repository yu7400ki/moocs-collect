import { createFileRoute } from "@tanstack/react-router";
import { SettingsPage } from "@/features/settings/pages/settings";

export const Route = createFileRoute("/_authenticated/settings")({
  component: SettingsPage,
});
