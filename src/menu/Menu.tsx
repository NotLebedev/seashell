import { Astal } from "ags/gtk4";
import { ParentProps } from "../util";
import app from "ags/gtk4/app";
import { Setter } from "ags";

export default function Menu({
  name,
  children,
  setForceDisplay,
}: ParentProps<{ name: string; setForceDisplay: Setter<boolean> }>) {
  const { TOP, RIGHT } = Astal.WindowAnchor;

  return (
    <window
      visible={false}
      name={name}
      anchor={TOP | RIGHT}
      application={app}
      margin_top={56}
      marginRight={16}
      onShow={() => setForceDisplay(true)}
      onHide={() => setForceDisplay(false)}
    >
      {children}
    </window>
  );
}
