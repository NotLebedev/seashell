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
export default function AutoHide({
  children,
  resizeHook,
}: ParentProps<{ resizeHook: () => void }>) {
  const [hovered, setHovered] = createState(false);
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
  enterController.connect("leave", () => setHovered(false));

  return (
    <box
      orientation={Gtk.Orientation.VERTICAL}
      $={(self) => self.add_controller(enterController)}
    >
      <revealer
        revealChild={displayed}
        onNotifyChildRevealed={() => resizeHook()}
      >
        <box
          orientation={Gtk.Orientation.VERTICAL}
          hexpand
          $={(self) => self.add_controller(leaveController)}
        >
          {children}
        </box>
      </revealer>
      <box
        valign={Gtk.Align.START}
        css="padding: 2px; background-color: white; opacity: 1%; border-radius: 0;"
      />
    </box>
  );
}
