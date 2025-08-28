import app from "ags/gtk4/app";
import { Astal, Gdk } from "ags/gtk4";
import AutoHide from "./HideController";
import { Props } from "../util";
import { Accessor, onCleanup } from "ags";
import Clock from "./widgets/Clock";
import Lang from "./widgets/Lang";
import Battery from "./widgets/Battery";

export default function Bar({
  gdkmonitor,
  forceDisplayed,
}: Props<{ gdkmonitor: Gdk.Monitor; forceDisplayed: Accessor<boolean> }>) {
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
      marginRight={16}
      $={(self) => {
        window = self;
        resize();
        onCleanup(() => self.destroy());
      }}
    >
      <AutoHide resizeHook={resize} forceDisplay={forceDisplayed}>
        <box hexpand cssName="centerbox">
          <Lang />
          <Battery />
          <Clock />
        </box>
      </AutoHide>
    </window>
  );
}
