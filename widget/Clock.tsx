import GLib from "gi://GLib?version=2.0";
import { createExternal, createState } from "gnim";
import BarComponent from "./BarComponent";

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
    <BarComponent>
      <label label={time((t) => t.format("%H:%M") ?? "00:00")} />
    </BarComponent>
  );
}
