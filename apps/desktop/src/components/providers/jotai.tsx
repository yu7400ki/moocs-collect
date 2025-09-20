import { createStore, Provider } from "jotai";

export const store = createStore();

type Props = {
  children?: React.ReactNode;
};

export function JotaiProvider({ children }: Props) {
  return <Provider store={store}>{children}</Provider>;
}
