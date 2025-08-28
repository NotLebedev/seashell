import app from "ags/gtk4/app";
import style from "./style.scss";
import Bar from "./widget/Bar";
import { createBinding, For, This } from "ags";
import Menu from "./widget/Menu";
import { Gtk } from "ags/gtk4";

function main() {
  const monitors = createBinding(app, "monitors");

  function CalendarMenu() {
    return (
      <Menu name="calendar">
        <Gtk.Calendar></Gtk.Calendar>
      </Menu>
    );
  }

  CalendarMenu();

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
