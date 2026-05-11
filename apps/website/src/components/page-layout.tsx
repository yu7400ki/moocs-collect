"use client";

import { Outlet } from "@funstack/router";

export function PageLayout() {
  return (
    <div className="relative min-h-screen">
      <div
        aria-hidden="true"
        className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_top,_rgba(56,189,248,0.18),_transparent_55%)]"
      />
      <div className="relative z-10 flex min-h-screen flex-col">
        <Outlet />
      </div>
    </div>
  );
}
