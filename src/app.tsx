import app from "ags/gtk4/app";
import style from "./style.scss";
import Bar from "./bar/Bar";
import { createBinding, createState, For, This } from "ags";

function main() {
  const monitors = createBinding(app, "monitors");

  // Allow any menu to force the bar to be displayed
  const [forceDisplay, setForceDisplay] = createState(false);

  return (
    <For each={monitors}>
      {(monitor) => (
        <This this={app}>
          <Bar
            gdkmonitor={monitor}
            forceDisplayed={forceDisplay}
            setForceDisplay={setForceDisplay}
          />
        </This>
      )}
    </For>
  );
}

app.start({
  css: style,
  main,
});
