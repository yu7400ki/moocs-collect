"use client";

import { type RouteDefinition, Router } from "@funstack/router";

// Strip trailing slash so concatenation works: "/base" + "/path" = "/base/path"
const BASE = import.meta.env.BASE_URL.replace(/\/$/, "");

// RouteDefinition is opaque but shares the same runtime shape as the internal type.
function prefixRoutes(routes: RouteDefinition[]): RouteDefinition[] {
  return (routes as any[]).map((r) => ({
    ...r,
    ...(r.path !== undefined && { path: BASE + r.path }),
    ...(r.children && { children: prefixRoutes(r.children) }),
  }));
}

export function Client({
  routes,
  ssrPath,
}: {
  routes: RouteDefinition[];
  ssrPath?: string;
}) {
  return (
    <Router
      routes={prefixRoutes(routes)}
      fallback="static"
      ssr={ssrPath !== undefined ? { path: BASE + ssrPath } : undefined}
    />
  );
}
