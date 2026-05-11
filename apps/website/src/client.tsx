"use client";

import { type RouteDefinition, Router } from "@funstack/router";

export function Client({
  routes,
  ssrPath,
}: {
  routes: RouteDefinition[];
  ssrPath?: string;
}) {
  return (
    <Router
      routes={routes}
      fallback="static"
      ssr={ssrPath ? { path: ssrPath } : undefined}
    />
  );
}
