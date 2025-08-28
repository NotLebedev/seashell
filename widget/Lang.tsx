import { createState } from "gnim";
import BarComponent from "./BarComponent";
import AstalHyprland from "gi://AstalHyprland?version=0.1";

// Patial info about devices of interest to keyboard layout
type DeviceInfo = {
  keyboards?: {
    main?: boolean;
    active_keymap?: string;
  }[];
};

function mapToShortName(fullName: string): string {
  const map = new Map<string, string>([
    ["English (US)", "us"],
    ["Russian", "ru"],
  ]);

  return map.get(fullName) ?? "us";
}

function parseLayout(state: AstalHyprland.Hyprland): string {
  const FALLBACK = "English (US)";
  const deviceInfo = JSON.parse(state.message("j/devices")) as DeviceInfo;

  if (!deviceInfo.keyboards) {
    return mapToShortName(FALLBACK);
  }

  const mainKb = deviceInfo.keyboards.find((kb) => kb.main);
  if (!mainKb) {
    return mapToShortName(FALLBACK);
  }

  return mapToShortName(mainKb.active_keymap ?? FALLBACK);
}

export default function Lang() {
  const [layoutLabel, setLayoutLabel] = createState(
    parseLayout(AstalHyprland.get_default()),
  );

  AstalHyprland.get_default().connect("keyboard-layout", async (state) => {
    setLayoutLabel(parseLayout(state));
  });

  return (
    <BarComponent>
      <label label={layoutLabel}></label>
    </BarComponent>
  );
}
