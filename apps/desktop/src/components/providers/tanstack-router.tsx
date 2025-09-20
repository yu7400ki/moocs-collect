import { routeTree } from "@/routeTree.gen";
import { RouterProvider, createRouter } from "@tanstack/react-router";

// Create a new router instance
export const router = createRouter({ routeTree });

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

export function TanstackRouterProvider() {
  return <RouterProvider router={router} />;
}
