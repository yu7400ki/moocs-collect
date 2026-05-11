import type { RouteDefinition } from "@funstack/router";
import type { EntryDefinition } from "@funstack/static/entries";
import App, { routes } from "./app";

function collectPaths(routeDefs: RouteDefinition[], prefix: string): string[] {
  const paths: string[] = [];
  for (const r of routeDefs) {
    const routePath = r.path;
    if (routePath === undefined) {
      if (r.children) {
        paths.push(...collectPaths(r.children, prefix));
      }
    } else if (routePath.includes(":")) {
      // parameterized routes cannot be statically generated
    } else if (r.children) {
      paths.push(...collectPaths(r.children, prefix + routePath));
    } else {
      const fullPath = routePath === "/" ? prefix || "/" : prefix + routePath;
      paths.push(fullPath);
    }
  }
  return paths;
}

function toEntry(path: string): { ssrPath: string; outputPath: string } {
  if (path === "/") {
    return { ssrPath: "/", outputPath: "index.html" };
  }
  const stripped = path.slice(1);
  return { ssrPath: path, outputPath: `${stripped}.html` };
}

export default function getEntries(): EntryDefinition[] {
  const paths = collectPaths(routes, "");
  return paths.map((path) => {
    const { ssrPath, outputPath } = toEntry(path);
    return {
      path: outputPath,
      root: () => import("./root"),
      app: <App ssrPath={ssrPath} />,
    };
  });
}
