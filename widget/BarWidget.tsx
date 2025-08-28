import { ParentProps } from "./util";

/**
 * Base component for all widgets in bar.
 * Anything going into bar should be put
 * as children of BarWidget
 *
 * ```tsx
 * <BarWidget>
 *   <label label="Sample text"/>
 * </BarWidget>
 * ```
 */
export default function BarWidget({ children }: ParentProps) {
  return <box class="barWidget">{children}</box>;
}
