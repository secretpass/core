import { Button, Dropdown, IconChevronDown, Link } from "@heroui/react";
import {
  IconBook2,
  IconBrandDiscord,
  IconBrandGithub,
  IconBrandMedium,
  IconBrandReddit,
  IconBrandSlack,
  IconCloudFilled,
  IconDeviceLaptop,
  IconExternalLink,
  IconMenu,
  IconX,
  type TablerIcon,
} from "@tabler/icons-react";
import { type ReactNode, useState } from "react";
import { Logo } from "@/components";
import { ThemeToggle } from "@/features/theme/theme-toggle";

interface LinkItemProps {
  label: string;
  Icon: TablerIcon;
  href: string;
}

const community_links: LinkItemProps[] = [
  {
    label: "Slack",
    Icon: IconBrandSlack,
    href: "https://slack.com",
  },
  {
    label: "Github",
    Icon: IconBrandGithub,
    href: "https://github.com",
  },
  {
    label: "Reddit",
    Icon: IconBrandReddit,
    href: "https://reddit.com",
  },
  {
    label: "Discord",
    Icon: IconBrandDiscord,
    href: "https://discord.com",
  },
];

const resources_links: LinkItemProps[] = [
  {
    label: "Docs",
    Icon: IconBook2,
    href: "https://secretpass.cloud/docs",
  },
  {
    label: "Blog",
    Icon: IconBrandMedium,
    href: "https://medium.com/secretpass",
  },
];

export function Navbar() {
  const [isOpen, setIsOpen] = useState(false);

  const toggleMenu = () => setIsOpen(!isOpen);

  return (
    <nav className="sticky top-0 z-50 w-full bg-background/95 backdrop-blur supports-backdrop-filter:bg-background/60">
      <header className="container mx-auto">
        <div className="flex grow h-16 items-center justify-between">
          <Link href="/" className="flex items-center space-x-2 no-underline">
            <Logo className="size-7" />
            <span className="text-2xl font-bold">Secretpass</span>
            <span className="text-2xl font-bold text-muted">Local</span>
            <IconDeviceLaptop className="size-7 text-muted" />
          </Link>

          <nav className="hidden md:flex items-center gap-3">
            <LinkDropDown items={community_links} label="Community" />
            <LinkDropDown items={resources_links} label="Resources" />

            <div className="flex items-center gap-4 border-l pl-4">
              <ThemeToggle />
              <Link
                href="https://secretpass.cloud"
                target="_blank"
                className="no-underline"
              >
                <Button size="sm">
                  <IconCloudFilled className="size-4" />
                  Cloud
                  <IconExternalLink className="size-4" />
                </Button>
              </Link>
            </div>
          </nav>

          <div className="flex items-center gap-4 md:hidden">
            <ThemeToggle />
            <button type="button" onClick={toggleMenu} aria-label="Toggle menu">
              {isOpen ? (
                <IconX className="h-6 w-6" />
              ) : (
                <IconMenu className="h-6 w-6" />
              )}
            </button>
          </div>
        </div>

        {isOpen && (
          <div className="md:hidden border-t px-4 bg-background">
            <div className="container py-4 flex flex-col gap-4">
              <Link
                href="/docs"
                className="text-sm flex items-center font-medium hover:text-primary transition-colors"
                onClick={() => setIsOpen(false)}
              >
                <IconExternalLink className="size-4" />
                Docs
              </Link>
              <div className="border-t pt-4">
                <Button className="w-full">
                  <Link href="/login">Sign In</Link>
                </Button>
              </div>
            </div>
          </div>
        )}
      </header>
    </nav>
  );
}

function LinkDropDown({
  items,
  label,
}: {
  items: LinkItemProps[];
  label: ReactNode;
}) {
  return (
    <Dropdown>
      <Dropdown.Trigger>
        <Button variant="ghost">
          {label}
          <IconChevronDown />
        </Button>
      </Dropdown.Trigger>
      <Dropdown.Popover>
        <Dropdown.Menu>
          {items.map((item) => (
            <Dropdown.Item
              className="flex items-center gap-2"
              key={item.href}
              href={item.href}
              target="_blank"
            >
              <item.Icon className="size-5" />
              <span className="grow">{item.label}</span>
              <IconExternalLink className="size-4" />
            </Dropdown.Item>
          ))}
        </Dropdown.Menu>
      </Dropdown.Popover>
    </Dropdown>
  );
}
