import { type JSX } from "react";

export type FilterInfo = {
  id: string;
  args: any[];
};

export interface UpdateProps {
  onUpdate: (info: FilterInfo) => void;
  onError: () => void;
}

export interface Option {
  name: string;
  component: (props: UpdateProps) => JSX.Element;
}
