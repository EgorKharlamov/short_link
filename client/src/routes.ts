import {lazy} from "solid-js";
import {useRoutes} from "@solidjs/router";

const routes = [
  {
    path: "/",
    component: lazy(() => import("./Home.js")),
  },
  {
    path: "/:id",
    component: lazy(() => import("./Link.js")),
  },
];

export const Routes = useRoutes(routes)
