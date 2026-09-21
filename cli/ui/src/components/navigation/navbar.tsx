"use client";

import { Button } from "@heroui/react";
import { IconCloudFilled, IconMenu, IconX } from "@tabler/icons-react";
import Link from "next/link";
import { useState } from "react";
import { ThemeToggle } from "@/components/shared/theme-toggle";
import { Logo } from "../brand";

export function Navbar() {
  const [isOpen, setIsOpen] = useState(false);

  const toggleMenu = () => setIsOpen(!isOpen);

  return (
    <nav className="sticky top-0 z-50 w-full bg-background/95 backdrop-blur supports-backdrop-filter:bg-background/60">
      <header className="container mx-auto">
        <div className="flex grow h-16 items-center justify-between">
          <Link href="/" className="flex items-center space-x-2">
            <Logo className="size-7" />
            <span className="text-2xl font-bold">Secretpass</span>
            <span className="text-2xl font-bold text-muted">Cloud</span>
            <IconCloudFilled className="size-7 text-muted" />
          </Link>

          <nav className="hidden md:flex items-center gap-6">
            <Link
              href="#features"
              className="text-sm font-medium hover:text-primary transition-colors"
            >
              Features
            </Link>
            <Link
              href="#pricing"
              className="text-sm font-medium hover:text-primary transition-colors"
            >
              Pricing
            </Link>
            <Link
              href="#faq"
              className="text-sm font-medium hover:text-primary transition-colors"
            >
              FAQ
            </Link>
            <Link
              href="/docs"
              className="text-sm font-medium hover:text-primary transition-colors"
            >
              Docs
            </Link>
            <div className="flex items-center gap-4 border-l pl-4">
              <ThemeToggle />
              <Button size="sm">
                <Link href="/login">Sign In</Link>
              </Button>
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
                href="#features"
                className="text-sm font-medium hover:text-primary transition-colors"
                onClick={() => setIsOpen(false)}
              >
                Features
              </Link>
              <Link
                href="#pricing"
                className="text-sm font-medium hover:text-primary transition-colors"
                onClick={() => setIsOpen(false)}
              >
                Pricing
              </Link>
              <Link
                href="#faq"
                className="text-sm font-medium hover:text-primary transition-colors"
                onClick={() => setIsOpen(false)}
              >
                FAQ
              </Link>
              <Link
                href="/docs"
                className="text-sm font-medium hover:text-primary transition-colors"
                onClick={() => setIsOpen(false)}
              >
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
