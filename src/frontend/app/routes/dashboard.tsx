import { PageShell } from "~/components/PageShell";
import type { Route } from "../+types/root";
import { SidebarProvider, SidebarTrigger } from "~/components/ui/sidebar";
import { AppSidebar } from "~/components/sidebar";

export function meta(_: Route.MetaArgs) {
  return [
    { title: "RustyMine Dashboard" }
  ];
}

export default function dashboard() {
  return (
    <PageShell>
      <SidebarProvider>
        <AppSidebar />
        <main className="flex-1">
          <SidebarTrigger />
        </main>
      </SidebarProvider>
    </PageShell>
  );
}
