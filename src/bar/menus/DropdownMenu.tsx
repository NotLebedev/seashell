import { Gtk } from "ags/gtk4";

function PowerProfileSelector() {
  const saver = (
    <togglebutton>
      <image
        iconName="power-profile-power-saver-symbolic"
        iconSize={Gtk.IconSize.LARGE}
      />
    </togglebutton>
  ) as Gtk.ToggleButton;
  const balanced = (
    <togglebutton group={saver}>
      <image
        iconName="power-profile-balanced-symbolic"
        iconSize={Gtk.IconSize.LARGE}
      />
    </togglebutton>
  ) as Gtk.ToggleButton;
  const performance = (
    <togglebutton group={balanced}>
      <image
        iconName="power-profile-performance-symbolic"
        iconSize={Gtk.IconSize.LARGE}
      />
    </togglebutton>
  ) as Gtk.ToggleButton;

  saver.set_active(true);

  return (
    <box class="toggleGroup" hexpand homogeneous>
      {saver}
      {balanced}
      {performance}
    </box>
  );
}

export default function DropdownMenu() {
  return (
    <box orientation={Gtk.Orientation.VERTICAL}>
      <Gtk.Calendar />
      <PowerProfileSelector />
    </box>
  );
}
