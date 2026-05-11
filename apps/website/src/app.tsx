import { route } from "@funstack/router/server";
import { Client } from "./client";
import Empty from "./pages/empty";
import Home from "./pages/home";

export const routes = [
  route({
    path: "/",
    component: <Home />,
  }),
  route({
    path: "/guide",
    component: <Empty />,
  }),
];

export default function App({ ssrPath }: { ssrPath?: string }) {
  return <Client routes={routes} ssrPath={ssrPath} />;
}
