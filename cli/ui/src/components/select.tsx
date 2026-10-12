import {
  Description,
  Select as HeroSelect,
  Label,
  ListBox,
} from "@heroui/react";
import { type ReactNode, useMemo } from "react";
import { useSecretManager } from "@/manager";

export interface SelectItem {
  value: string;
  label: string;
}

export interface SelectProps<T extends string | string[]> {
  multiselect?: boolean;
  variant?: "primary" | "secondary";
  value: T;
  onChange?: (updated_value: T) => void;
  label: ReactNode;
  description?: ReactNode;
  items: SelectItem[];
  className?: string;
  disabled?: boolean;
}

export function Select<T extends string | string[]>({
  value,
  className,
  disabled,
  multiselect,
  onChange,
  label,
  items,
  description,
  variant,
}: SelectProps<T>) {
  return (
    <HeroSelect
      variant={variant}
      value={value}
      className={className}
      isDisabled={disabled}
      selectionMode={multiselect ? "multiple" : "single"}
      onChange={onChange ? (value) => onChange(value as T) : undefined}
    >
      <Label>{label}</Label>
      <HeroSelect.Trigger>
        <HeroSelect.Value />
        <HeroSelect.Indicator />
      </HeroSelect.Trigger>
      <Description>{description}</Description>
      <HeroSelect.Popover>
        <ListBox>
          {items.map((item) => (
            <ListBox.Item
              key={item.value}
              id={item.value}
              textValue={item.value}
            >
              {item.label}
              <ListBox.ItemIndicator />
            </ListBox.Item>
          ))}
        </ListBox>
      </HeroSelect.Popover>
    </HeroSelect>
  );
}

export function EnvironmentSelector<T extends string | string[]>(
  props: Omit<SelectProps<T>, "items">,
) {
  const manager = useSecretManager();
  const items = useMemo(() => {
    if (manager.user.level === "Admin") {
      return manager.config.environments.map((env) => ({
        label: env.name,
        value: env.name,
      }));
    } else {
      return (manager.user.access ?? []).map((env) => ({
        label: env,
        value: env,
      }));
    }
  }, [manager.config.environments, manager.user]);

  return <Select {...props} items={items} />;
}
