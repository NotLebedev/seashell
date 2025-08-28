type PlainObject = { [key: string]: unknown };

type Props<T extends PlainObject> = T;
// eslint-disable-next-line @typescript-eslint/no-empty-object-type
type ParentProps<T extends PlainObject = {}> = Props<T> & {
  children?: JSX.Element | Array<JSX.Element>;
};

export type { PlainObject, Props, ParentProps };
