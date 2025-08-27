import app from "ags/gtk4/app";
import { Astal, Gdk } from "ags/gtk4";
import AutoHide from "./HideController";
import { Props } from "./util";
import { onCleanup } from "ags";
import Clock from "./Clock";

export default function Bar({
  gdkmonitor,
}: Props<{ gdkmonitor: Gdk.Monitor }>) {
  const { TOP, RIGHT } = Astal.WindowAnchor;

  let window!: Astal.Window;
  function resize() {
    window.set_default_size(1, 1);
    window.queue_resize();
  }

  return (
    <window
      visible
      resizable
      name="bar"
      class="Bar"
      gdkmonitor={gdkmonitor}
      exclusivity={Astal.Exclusivity.NORMAL}
      anchor={TOP | RIGHT}
      application={app}
      namespace="seashell"
      $={(self) => {
        window = self;
        onCleanup(() => self.destroy());
      }}
    >
      <AutoHide resizeHook={resize}>
        <box hexpand cssName="centerbox">
          <Clock />
        </box>
      </AutoHide>
    </window>
  );
}
