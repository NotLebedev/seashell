import { ParentProps } from "./util";

/**
 * Base component for all widgets in bar.
 * Anything going into bar shoul be put inside
 *
 * ```tsx
 * <BarComponent>
 *   <label label="Sample text"/>
 * </BarComponent>
 * ```
 */
export default function BarComponent({ children }: ParentProps) {
  return <box class="barComponent">{children}</box>;
}
