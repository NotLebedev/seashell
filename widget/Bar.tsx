import app from "ags/gtk4/app";
import { Astal, Gtk, Gdk } from "ags/gtk4";
import AutoHide from "./HideController";

function HideButton() {
  return (
    <button onClicked={(self) => console.log(self, "clicked")}>
      <label label="Run" />
    </button>
  );
}

export default function Bar(gdkmonitor: Gdk.Monitor) {
  const { TOP, LEFT, RIGHT } = Astal.WindowAnchor;

  return (
    <window
      visible
      name="bar"
      class="Bar"
      gdkmonitor={gdkmonitor}
      exclusivity={Astal.Exclusivity.IGNORE}
      anchor={TOP | LEFT | RIGHT}
      application={app}
    >
      <AutoHide>
        <box hexpand cssName="centerbox">
          Text text text
          <HideButton></HideButton>
        </box>
      </AutoHide>
    </window>
  );
}
