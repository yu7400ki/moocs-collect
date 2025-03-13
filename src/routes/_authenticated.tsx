import { store } from "@/components/providers/jotai";
import { authenticatedAtom } from "@/features/auth/atoms/authenticated";
import { Outlet, createFileRoute, redirect } from "@tanstack/react-router";
import { css } from "styled-system/css";
import { Sidebar } from "./-components/sidebar";

export const Route = createFileRoute("/_authenticated")({
  beforeLoad: () => {
    const authenticated = store.get(authenticatedAtom);
    if (!authenticated) {
      throw redirect({
        to: "/login",
      });
    }
  },
  component: Component,
});

function Component() {
  return (
    <div
      className={css({
        h: "100dvh",
        display: "grid",
        gridTemplateColumns: "auto 1fr",
        p: 1,
        gap: 1,
        bg: "bg.canvas",
      })}
    >
      <Sidebar />
      <div
        className={css({
          display: "grid",
          minH: 0,
          gridTemplateRows: "minmax(0, 1fr)",
          bg: "bg.default",
          rounded: "l2",
          border: "1px solid",
          borderLeft: "1px solid",
          borderColor: "border.subtle",
          overflowX: "auto",
        })}
      >
        <Outlet />
      </div>
    </div>
  );
}
