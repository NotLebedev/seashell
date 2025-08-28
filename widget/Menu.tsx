import { Astal } from "ags/gtk4";
import { ParentProps } from "./util";
import app from "ags/gtk4/app";

export default function Menu({
  name,
  children,
}: ParentProps<{ name: string }>) {
  const { TOP, RIGHT } = Astal.WindowAnchor;

  return (
    <window
      visible
      name={name}
      anchor={TOP | RIGHT}
      application={app}
      margin_top={56}
      marginRight={16}
    >
      {children}
    </window>
  );
}
