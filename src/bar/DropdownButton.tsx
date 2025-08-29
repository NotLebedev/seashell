import { Gtk } from "ags/gtk4";
import { ParentProps } from "../util";
import { Setter } from "ags";

export default function MenuButton({
  children,
  setForceDisplay,
  menu,
}: ParentProps<{ menu: JSX.Element; setForceDisplay: Setter<boolean> }>) {
  return (
    <Gtk.MenuButton
      class="dropdownButton"
      popover={(<Gtk.Popover>{menu}</Gtk.Popover>) as Gtk.Popover}
      direction={Gtk.ArrowType.DOWN}
      onNotifyActive={(source) => setForceDisplay(source.active)}
      $={(self) => {
        self.popover.hasArrow = false;
        // Compensate for no arrow in layout
        self.popover.set_offset(0, 12);
      }}
    >
      <box orientation={Gtk.Orientation.HORIZONTAL}>{children}</box>
    </Gtk.MenuButton>
  );
}
