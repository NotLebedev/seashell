import GLib from "gi://GLib?version=2.0";
import { createExternal, createState } from "gnim";
import BarWidget from "../BarWidget";
import app from "ags/gtk4/app";

export default function Clock() {
  createState(GLib.DateTime.new_now_local);

  const time = createExternal(GLib.DateTime.new_now_local(), (set) => {
    const interval = setInterval(
      () => set(() => GLib.DateTime.new_now_local()),
      1000,
    );

    return () => clearInterval(interval);
  });

  return (
    <BarWidget>
      <button class="invisible" onClicked={() => app.toggle_window("calendar")}>
        <label label={time((t) => t.format("%H:%M") ?? "00:00")} />
      </button>
    </BarWidget>
  );
}
