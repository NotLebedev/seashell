import app from "ags/gtk4/app";
import style from "./style.scss";
import Bar from "./widget/Bar";
import { createBinding, createState, For, This } from "ags";
import Menu from "./widget/Menu";
import { Gtk } from "ags/gtk4";

function main() {
  const monitors = createBinding(app, "monitors");

  // Allow any menu to force the bar to be displayed
  const [forceDisplay, setForceDisplay] = createState(false);

  function CalendarMenu() {
    return (
      <Menu name="calendar" setForceDisplay={setForceDisplay}>
        <Gtk.Calendar></Gtk.Calendar>
      </Menu>
    );
  }

  CalendarMenu();

  return (
    <For each={monitors}>
      {(monitor) => (
        <This this={app}>
          <Bar gdkmonitor={monitor} forceDisplayed={forceDisplay}></Bar>
        </This>
      )}
    </For>
  );
}

app.start({
  css: style,
  main,
});
