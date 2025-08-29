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

  let menuButton!: Gtk.MenuButton;
  const clickController = new Gtk.GestureClick({ button: 0 });
  clickController.connect("pressed", (gesture) => {
    gesture.set_state(Gtk.EventSequenceState.CLAIMED);
    if (gesture.get_current_button() == 1) {
      item.activate(0, 0);
    } else if (gesture.get_current_button() === 3) {
      menuButton.popup();
    }
  });

  return (
    <Gtk.MenuButton
      class="trayItem"
      direction={Gtk.ArrowType.DOWN}
      onNotifyActive={(source) => setForceDisplay(source.active)}
      $={(self) => {
        menuButton = self;
        self.add_controller(clickController);

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

  const filteredItems = items((items) => items.filter((item) => item.id));

  return (
    <BarWidget>
      <For each={filteredItems}>
        {(item) => {
          return <TrayItem item={item} setForceDisplay={setForceDisplay} />;
        }}
      </For>
    </BarWidget>
  );
}
