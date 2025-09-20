import { useAtomValue } from "jotai";
import { WifiOffIcon } from "lucide-react";
import { css } from "styled-system/css";
import { Divider, Flex } from "styled-system/jsx";
import { authenticatedAtom } from "@/features/auth/atoms/authenticated";
import { Column } from "../components/column";
import { Download } from "../components/download";
import { YearSelect } from "../components/year-select";

export function CoursePage() {
  const authenticated = useAtomValue(authenticatedAtom);
  const isOffline = authenticated === "offline";

  if (isOffline) {
    return (
      <main
        className={css({
          display: "grid",
          placeItems: "center",
          placeContent: "center",
          gap: 4,
          height: "full",
          color: "fg.muted",
        })}
      >
        <WifiOffIcon />
        オフラインモードでは科目一覧は利用できません
      </main>
    );
  }

  return (
    <main
      className={css({
        display: "grid",
        gridTemplateRows: "auto auto minmax(0, 1fr)",
      })}
    >
      <Flex m="1" justifyContent="space-between">
        <YearSelect />
        <Download />
      </Flex>
      <Divider orientation="horizontal" />
      <Column />
    </main>
  );
}
