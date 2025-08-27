import app from "ags/gtk4/app";
import style from "./style.scss";
import Bar from "./widget/Bar";
import { createBinding, For, This } from "ags";

function main() {
  const monitors = createBinding(app, "monitors");

  return (
    <For each={monitors}>
      {(monitor) => (
        <This this={app}>
          <Bar gdkmonitor={monitor}></Bar>
        </This>
      )}
    </For>
  );
}

app.start({
  css: style,
  main,
});
