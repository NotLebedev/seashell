import app from "ags/gtk4/app";
import { Astal, Gdk } from "ags/gtk4";
import AutoHide from "./HideController";
import { Props } from "../util";
import { Accessor, onCleanup, Setter } from "ags";
import Clock from "./widgets/Clock";
import Lang from "./widgets/Lang";
import Battery from "./widgets/Battery";
import Tray from "./widgets/Tray";

export default function Bar({
  gdkmonitor,
  forceDisplayed,
  setForceDisplay,
}: Props<{
  gdkmonitor: Gdk.Monitor;
  forceDisplayed: Accessor<boolean>;
  setForceDisplay: Setter<boolean>;
}>) {
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
        resize();
        onCleanup(() => self.destroy());
      }}
    >
      <AutoHide resizeHook={resize} forceDisplay={forceDisplayed}>
        <box hexpand cssName="centerbox">
          <Lang />
          <Tray setForceDisplay={setForceDisplay} />
          <Battery />
          <Clock />
        </box>
      </AutoHide>
    </window>
  );
}
