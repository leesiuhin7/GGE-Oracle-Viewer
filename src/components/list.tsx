import React, { useEffect, useImperativeHandle, useRef, useState } from "react";

export type Item = React.ComponentType<{
  remove: () => void;
  id: number;
}>;
export type Listener = () => void;

export interface ListHandle {
  append(item: Item): void;
}

export default function List({
  ref,
  onChange,
}: {
  ref?: React.Ref<ListHandle>;
  onChange?: (ids: number[]) => void;
}) {
  const [items, setItems] = useState<Map<number, Item>>(new Map());
  const nextId = useRef(0);

  const updateItems = (callback: (items: Map<number, Item>) => void) => {
    setItems((map) => {
      const newMap = new Map(map);
      callback(newMap);
      return newMap;
    });
  };
  useEffect(() => {
    onChange?.([...items.keys()]);
  }, [items]);

  useImperativeHandle(ref, () => ({
    append(item) {
      const id = nextId.current;
      nextId.current += 1;
      updateItems((items) => items.set(id, item));
    },
  }));

  return (
    <>
      {[...items.entries()].map(([id, Component]) => {
        function remove() {
          updateItems((items) => {
            items.delete(id);
          });
        }
        return <Component key={id} remove={remove} id={id} />;
      })}
    </>
  );
}
