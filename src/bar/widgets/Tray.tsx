import AstalTray from "gi://AstalTray?version=0.1";
import BarWidget from "../BarWidget";
import { createState, For, Setter } from "ags";
import { Props } from "../../util";
import { Gtk } from "ags/gtk4";

function TrayItem({
  item,
  setForceDisplay,
}: Props<{ item: AstalTray.TrayItem; setForceDisplay: Setter<boolean> }>) {
  const [icon, setIcon] = createState(item.gicon);

  item.connect("changed", (newItem) => {
    setIcon(newItem.gicon);
  });

  return (
    <Gtk.MenuButton
      class="trayItem"
      direction={Gtk.ArrowType.DOWN}
      onNotifyActive={(source) => setForceDisplay(source.active)}
      $={(self) => {
        self.set_menu_model(item.menuModel);
        self.insert_action_group("dbusmenu", item.actionGroup);

        self.popover.hasArrow = false;
        // Compensate for no arrow in layout
        self.popover.set_offset(0, 12);
      }}
    >
      <image gicon={icon} />
    </Gtk.MenuButton>
  );
}

export default function Tray({
  setForceDisplay,
}: Props<{ setForceDisplay: Setter<boolean> }>) {
  const [items, setItems] = createState(AstalTray.get_default().get_items());

  AstalTray.get_default().connect("notify::items", (state) => {
    setItems(state.get_items());
  });

  return (
    <BarWidget>
      <For each={items}>
        {(item) => {
          return <TrayItem item={item} setForceDisplay={setForceDisplay} />;
        }}
      </For>
    </BarWidget>
  );
}
