import { clientOnly } from "@solidjs/start";
import type { Component } from "solid-js";

const Button = clientOnly(() => import(`../components/Button`));

const HomePage: Component = () => (
  <main class="flex min-h-screen items-center justify-center flex-col">
    <h1 class="text-3xl font-semibold">Todos</h1>
    <Button onClick={() => alert(`todo`)}>Fetch todos</Button>
  </main>
);

export default HomePage;
