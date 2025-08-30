import AstalBattery from "gi://AstalBattery?version=0.1";
import BarWidget from "../BarWidget";
import { createState } from "gnim";

export default function Battery() {
  const battery = AstalBattery.get_default();
  if (!battery.isBattery) {
    // If device has no battery don't display this widget
    return <></>;
  }

  const [chargePercent, setChargePercent] = createState(battery.percentage);

  battery.connect("notify::percentage", (state) => {
    setChargePercent(state.percentage);
  });

  return (
    <BarWidget>
      <label
        label={chargePercent((p) =>
          new Intl.NumberFormat("en-US", { style: "percent" }).format(p),
        )}
      />
    </BarWidget>
  );
}
