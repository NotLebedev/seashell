import { Gtk } from "ags/gtk4";
import { ParentProps } from "./util";
import { createComputed, createExternal, createState } from "ags";
import AstalHyprland from "gi://AstalHyprland?version=0.1";

function isEmptyWorkspace(state: AstalHyprland.Hyprland): boolean {
  return state.focused_workspace?.clients?.length === 0;
}

/**
 * Component ot auto hide its contents and show on hover
 */
export default function AutoHide({
  children,
  resizeHook,
}: ParentProps<{ resizeHook: () => void }>) {
  // Bar is hovered on
  const [hovered, setHovered] = createState(false);
  // Currently on empty workspace
  const [emptyWorkspace, setEmptyWorkspace] = createState(
    isEmptyWorkspace(AstalHyprland.get_default()),
  );
  // Show for a few seconds when just starting up
  const initialDisplay = createExternal(true, (set) => {
    const timeout = setTimeout(() => set(() => false), 1000);

    return () => clearTimeout(timeout);
  });

  const displayed = createComputed(
    [hovered, emptyWorkspace, initialDisplay],
    (hovered, emptyWorkspace, initialDisplay) =>
      hovered || emptyWorkspace || initialDisplay,
  );

  const enterController = new Gtk.EventControllerMotion();

  AstalHyprland.get_default().connect("notify::focused-workspace", (state) => {
    setEmptyWorkspace(isEmptyWorkspace(state));
  });

  enterController.connect("enter", () => setHovered(true));
  enterController.connect("leave", () => setHovered(false));

  return (
    <box
      orientation={Gtk.Orientation.VERTICAL}
      $={(self) => {
        self.add_controller(enterController);
        self.set_size_request(-1, 4);
      }}
    >
      <revealer revealChild={displayed} onNotifyChildRevealed={resizeHook}>
        <box orientation={Gtk.Orientation.VERTICAL} hexpand>
          {children}
        </box>
      </revealer>
    </box>
  );
}
