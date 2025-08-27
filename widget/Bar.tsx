import app from "ags/gtk4/app";
import { Astal, Gdk } from "ags/gtk4";
import AutoHide from "./HideController";
import { Props } from "./util";
import { onCleanup } from "ags";

function HideButton() {
  return (
    <button onClicked={(self) => console.log(self, "clicked")}>
      <label label="Run" />
    </button>
  );
}

export default function Bar({
  gdkmonitor,
}: Props<{ gdkmonitor: Gdk.Monitor }>) {
  const { TOP, LEFT, RIGHT } = Astal.WindowAnchor;

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
      anchor={TOP | LEFT | RIGHT}
      application={app}
      $={(self) => {
        window = self;
        onCleanup(() => self.destroy());
      }}
    >
      <AutoHide resizeHook={resize}>
        <box hexpand cssName="centerbox">
          Text text text
          <HideButton></HideButton>
        </box>
      </AutoHide>
    </window>
  );
}
