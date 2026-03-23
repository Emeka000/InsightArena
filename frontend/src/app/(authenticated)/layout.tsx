import type { ReactNode } from "react";

import { DashboardShell } from "@/component/dashboard-shell";

export default function AuthenticatedLayout({
  children,
}: {
  children: ReactNode;
}) {
  return <DashboardShell>{children}</DashboardShell>;
}
