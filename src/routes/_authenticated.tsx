import { store } from "@/components/ui/providers/jotai";
import { authenticatedAtom } from "@/features/auth/atoms/authenticated";
import { createFileRoute, redirect } from "@tanstack/react-router";

export const Route = createFileRoute("/_authenticated")({
  beforeLoad: () => {
    const authenticated = store.get(authenticatedAtom);
    if (!authenticated) {
      throw redirect({
        to: "/login",
      });
    }
  },
});
