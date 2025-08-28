import AstalTray from "gi://AstalTray?version=0.1";
import BarWidget from "../BarWidget";
import { createState, For, Setter } from "ags";
import { Props } from "../../util";
import { Gtk } from "ags/gtk4";

function TrayItem({
  item,
  setForceDisplay,
}: Props<{ item: AstalTray.TrayItem; setForceDisplay: Setter<boolean> }>) {
  return (
    <Gtk.MenuButton
      iconName={item.iconName}
      direction={Gtk.ArrowType.DOWN}
      onNotifyActive={(source) => setForceDisplay(source.active)}
      menuModel={item.menuModel}
      $={(self) => self.insert_action_group("dbusmenu", item.actionGroup)}
    >
      <image gicon={item.gicon} />
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
