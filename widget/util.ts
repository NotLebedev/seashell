type PlainObject = { [key: string]: unknown };

type Props<T extends PlainObject> = T;
type ParentProps<T extends PlainObject = {}> = Props<T> & {
  children: JSX.Element | Array<JSX.Element>;
};

export type { PlainObject, Props, ParentProps };
