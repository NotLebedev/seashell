import AstalBattery from "gi://AstalBattery?version=0.1";
import BarComponent from "./BarComponent";
import { createState } from "gnim";

export default function Battery() {
  if (!AstalBattery.get_default().isBattery) {
    // If device has no battery don't display this widget
    return <></>;
  }

  const [chargePercent, setChargePercent] = createState(0);

  AstalBattery.get_default().connect("notify::percentage", (state) => {
    setChargePercent(state.get_percentage());
  });

  return (
    <BarComponent>
      <label label={chargePercent((p) => `${p}%`)} />
    </BarComponent>
  );
}
