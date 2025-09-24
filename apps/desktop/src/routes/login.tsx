import { createFileRoute } from "@tanstack/react-router";
import { Login } from "@/features/auth/pages/login";

export const Route = createFileRoute("/login")({
  component: Login,
});
