import { Button } from "@/components/ui/button";
import { Link } from "@tanstack/react-router";
import {
  DownloadIcon,
  FolderOpenIcon,
  PanelLeftCloseIcon,
  PanelLeftOpenIcon,
  SettingsIcon,
} from "lucide-react";
import { useState } from "react";
import { css, cx } from "styled-system/css";

export function Sidebar() {
  const [isExpanded, setIsExpanded] = useState(false);

  return (
    <div
      className={cx(
        "group",
        css({
          w: "fit-content",
          display: "grid",
          gridTemplateRows: "1fr auto",
          transition: "width 0.2s",
          interpolateSize: "allow-keywords",
          _expanded: {
            w: "32",
          },
        }),
      )}
      data-expanded={isExpanded ? "" : undefined}
    >
      <div
        className={css({
          display: "grid",
          alignContent: "start",
          gap: 1,
        })}
      >
        <SidebarButton
          onClick={() => setIsExpanded((prev) => !prev)}
          className={css({
            w: "fit-content",
          })}
        >
          {isExpanded ? <PanelLeftCloseIcon /> : <PanelLeftOpenIcon />}
        </SidebarButton>
        <SidebarButton asChild>
          <Link to="/">
            <FolderOpenIcon />
            <span>科目一覧</span>
          </Link>
        </SidebarButton>
        <SidebarButton asChild>
          <Link to="/download">
            <DownloadIcon />
            <span>ダウンロード</span>
          </Link>
        </SidebarButton>
      </div>
      <div>
        <SidebarButton asChild>
          <Link to="/settings">
            <SettingsIcon />
            <span>設定</span>
          </Link>
        </SidebarButton>
      </div>
    </div>
  );
}

function SidebarButton({
  className,
  ...props
}: React.ComponentProps<typeof Button>) {
  return (
    <Button
      size="sm"
      variant="ghost"
      className={cx(
        css({
          display: "flex",
          justifyContent: "start",
          lineHeight: 1,
          h: "auto",
          minW: "0",
          p: 2,
          fontWeight: "normal",
          _currentPage: {
            bg: "colorPalette.a3",
          },
          "& > span": {
            display: "none",
            _groupExpanded: {
              display: "block",
            },
          },
        }),
        className,
      )}
      {...props}
    />
  );
}
