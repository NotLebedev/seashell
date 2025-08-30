import { Gtk } from "ags/gtk4";
import AstalPowerProfiles from "gi://AstalPowerProfiles?version=0.1";

function PowerProfileSelector() {
  const powerProfiles = AstalPowerProfiles.get_default();

  const saver = (
    <togglebutton
      onClicked={() => (powerProfiles.activeProfile = "power-saver")}
    >
      <image
        iconName="power-profile-power-saver-symbolic"
        iconSize={Gtk.IconSize.LARGE}
      />
    </togglebutton>
  ) as Gtk.ToggleButton;
  const balanced = (
    <togglebutton
      group={saver}
      onClicked={() => (powerProfiles.activeProfile = "balanced")}
    >
      <image
        iconName="power-profile-balanced-symbolic"
        iconSize={Gtk.IconSize.LARGE}
      />
    </togglebutton>
  ) as Gtk.ToggleButton;
  const performance = (
    <togglebutton
      group={balanced}
      onClicked={() => (powerProfiles.activeProfile = "performance")}
    >
      <image
        iconName="power-profile-performance-symbolic"
        iconSize={Gtk.IconSize.LARGE}
      />
    </togglebutton>
  ) as Gtk.ToggleButton;

  function selectActiveProfile(state: AstalPowerProfiles.PowerProfiles) {
    switch (state.activeProfile) {
      case "power-saver":
        saver.set_active(true);
        break;
      case "balanced":
        balanced.set_active(true);
        break;
      case "performance":
        performance.set_active(true);
        break;
    }
  }

  selectActiveProfile(powerProfiles);
  powerProfiles.connect("notify::active-profile", selectActiveProfile);

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
