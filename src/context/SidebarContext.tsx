"use client";

import type { PropsWithChildren } from "react";
import { createContext, useContext, useEffect, useState } from "react";
import { setStorage, getStorage } from "@/interface/utils/storage";

interface SidebarContextProps {
  desktop: {
    isCollapsed: boolean;
    setCollapsed(value: boolean): void;
    toggle(): void;
  };
  mobile: {
    isOpen: boolean;
    close(): void;
    toggle(): void;
  };
}

const SidebarContext = createContext<SidebarContextProps | null>(null);

export function SidebarProvider({
  children,
}: PropsWithChildren<{}>) {
  const init_collapse: boolean = getStorage("local_collapse_status") ?? false;
  const [isOpenMobile, setIsOpenMobile] = useState(false);
  const [isCollapsed, setCollapsed] = useState(init_collapse);

  // 因为localstorage的访问和SSR的冲突问题，暂时先注释掉
  // useEffect(() => {
  //   const init_collapse: boolean = getStorage("local_collapse_status") ?? false;
  //   setCollapsed(init_collapse);
  // }, []);

  function handleSetCollapsed(value: boolean) {
    // 因为localstorage的访问和SSR的冲突问题，暂时先注释掉
    setStorage("local_collapse_status", value);
    setCollapsed(value);
  }

  return (
    <SidebarContext.Provider
      value={{
        desktop: {
          isCollapsed,
          setCollapsed: handleSetCollapsed,
          toggle: () => handleSetCollapsed(!isCollapsed),
        },
        mobile: {
          isOpen: isOpenMobile,
          close: () => setIsOpenMobile(false),
          toggle: () => setIsOpenMobile((state) => !state),
        },
      }}
    >
      {children}
    </SidebarContext.Provider>
  );
}

export function useSidebarContext(): SidebarContextProps {
  const context = useContext(SidebarContext);

  if (!context) {
    throw new Error(
      "useSidebarContext must be used within the SidebarContext provider!",
    );
  }

  return context;
}
