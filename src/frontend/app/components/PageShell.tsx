export function PageShell({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex bg-zinc-950 w-screen min-h-screen">{children}</div>
  );
}
