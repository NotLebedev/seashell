import { Gtk } from "ags/gtk4"
import { ParentProps } from "./util"
import { createState } from "ags"

/**
 * Component ot auto hide its contents and show on hover
 */
export default function AutoHide({ children }: ParentProps) {
  const [hidden, setHidden] = createState(false)

  const enterController = new Gtk.EventControllerMotion()
  const leaveController = new Gtk.EventControllerMotion()

  enterController.connect("enter", () => setHidden(false))
  leaveController.connect("leave", () => setHidden(true))

  return (
    <overlay class="hideController">
      <box hexpand class={hidden((h) => (h ? "hidden hider" : "hider"))}>
        {children}
      </box>
      <box // Big overlay box that tracks leave after hover
        $type="overlay"
        halign={Gtk.Align.FILL}
        valign={Gtk.Align.FILL}
        $={(self) => self.add_controller(leaveController)}
      />
      <box // Smaller overlay box that tracks entry
        $type="overlay"
        height_request={10}
        valign={Gtk.Align.START}
        $={(self) => self.add_controller(enterController)}
      />
    </overlay>
  )
}
