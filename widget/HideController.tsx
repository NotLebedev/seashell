import { Gtk } from "ags/gtk4";
import { ParentProps } from "./util";
import { createComputed, createState } from "ags";
import AstalHyprland from "gi://AstalHyprland?version=0.1";

function isEmptyWorkspace(state: AstalHyprland.Hyprland): boolean {
  return state.focused_workspace.clients.length === 0;
}

/**
 * Component ot auto hide its contents and show on hover
 */
export default function AutoHide({ children }: ParentProps) {
  const [hovered, setHovered] = createState(true);
  const [emptyWorkspace, setEmptyWorkspace] = createState(
    isEmptyWorkspace(AstalHyprland.get_default()),
  );

  const displayed = createComputed(
    [hovered, emptyWorkspace],
    (hovered, emptyWorkspace) => hovered || emptyWorkspace,
  );

  const enterController = new Gtk.EventControllerMotion();
  const leaveController = new Gtk.EventControllerMotion();

  AstalHyprland.get_default().connect("notify::focused-workspace", (state) => {
    setEmptyWorkspace(isEmptyWorkspace(state));
  });

  enterController.connect("enter", () => setHovered(true));
  leaveController.connect("leave", () => setHovered(false));

  return (
    <overlay class="hideController">
      <box
        hexpand
        class={displayed((d) => (d ? "hider" : "hidden hider"))}
        $={(self) => self.add_controller(leaveController)}
      >
        {children}
      </box>
      <box // Smaller overlay box that tracks entry
        $type="overlay"
        height_request={10}
        valign={Gtk.Align.START}
        $={(self) => self.add_controller(enterController)}
      />
    </overlay>
  );
}
