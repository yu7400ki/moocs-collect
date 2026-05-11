"use client";

import { type RouteDefinition, Router } from "@funstack/router";
import { base } from "./utils/base-path";

// RouteDefinition is opaque but shares the same runtime shape as the internal type.
function prefixRoutes(routes: RouteDefinition[]): RouteDefinition[] {
  return (routes as any[]).map((r) => ({
    ...r,
    ...(r.path !== undefined && { path: base + r.path }),
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
      ssr={ssrPath !== undefined ? { path: base + ssrPath } : undefined}
    />
  );
}
