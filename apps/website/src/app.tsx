import { route } from "@funstack/router/server";
import { Client } from "./client";
import { PageLayout } from "./components/page-layout";
import Guide from "./pages/guide";
import Home from "./pages/home";

export const routes = [
  route({
    component: PageLayout,
    children: [
      route({
        path: "/",
        component: <Home />,
      }),
      route({
        path: "/guide",
        component: <Guide />,
      }),
    ],
  }),
];

export default function App({ ssrPath }: { ssrPath?: string }) {
  return <Client routes={routes} ssrPath={ssrPath} />;
}
