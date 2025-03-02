import { JotaiProvider } from "./jotai";

type Props = {
  children?: React.ReactNode;
};

export function Providers({ children }: Props) {
  return <JotaiProvider>{children}</JotaiProvider>;
}
