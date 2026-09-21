"use client";

import {Button, Dropdown, Label} from "@heroui/react";
import {IconDeviceDesktop, IconMoon, IconSun} from "@tabler/icons-react";

import {UiTheme} from "@/constants";
import {useTheme} from "./theme-provider";

export function ThemeToggle() {
  const {theme, setTheme} = useTheme();

  return (
        <Dropdown>
    <Button
      variant="ghost"
      isIconOnly
      aria-label="Toggle theme"
    >
      {theme === UiTheme.Light && <IconSun className="size-5" />}
      {theme === UiTheme.Dark && <IconMoon className="size-5" />}
      {theme === UiTheme.System && <IconDeviceDesktop className="size-5" />}
    </Button>
        <Dropdown.Popover>
          <Dropdown.Menu onAction={(key) => setTheme(key as UiTheme)}>
            <Dropdown.Item id={UiTheme.Light}>
              <IconSun className="size-5" />
              <Label>Light</Label>
            </Dropdown.Item>
            <Dropdown.Item id={UiTheme.Dark}>
              <IconMoon className="size-5" />
              <Label>Dark</Label>
            </Dropdown.Item>
            <Dropdown.Item id={UiTheme.System}>
              <IconDeviceDesktop className="size-5" />
              <Label>System</Label>
            </Dropdown.Item>
          </Dropdown.Menu>
        </Dropdown.Popover>
  </Dropdown>
  );
}
