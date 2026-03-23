"use client";

import Link from "next/link";
import { AlertTriangle } from "lucide-react";

import { Button } from "@/component/ui/button";

type AppNotFoundProps = {
  compact?: boolean;
};

export function AppNotFound({ compact = false }: AppNotFoundProps) {
  return (
    <div
      className={`flex min-h-full items-center justify-center ${
        compact ? "px-4 py-10 sm:px-6" : "px-6 py-16 sm:px-8"
      }`}
    >
      <div className="w-full max-w-2xl rounded-[28px] border border-white/10 bg-[#22293b]/95 p-8 text-white shadow-[0_24px_80px_rgba(0,0,0,0.35)] backdrop-blur">
        <div className="mb-6 inline-flex h-14 w-14 items-center justify-center rounded-2xl border border-[#38b2ac]/25 bg-[#38b2ac]/12 text-[#4fd1c5]">
          <AlertTriangle className="h-7 w-7" />
        </div>
        <p className="text-sm font-medium uppercase tracking-[0.32em] text-[#4fd1c5]/80">
          Error 404
        </p>
        <h1 className="mt-3 text-3xl font-semibold tracking-tight text-white sm:text-4xl">
          This page is not available yet.
        </h1>
        <p className="mt-4 max-w-xl text-sm leading-7 text-[#9ca3b4] sm:text-base">
          The authenticated layout is active, but this destination has not been
          implemented. Use Dashboard to return to the live experience.
        </p>
        <div className="mt-8 flex flex-col gap-3 sm:flex-row">
          <Button
            asChild
            className="h-11 rounded-xl bg-[#2f9e9d] px-5 text-sm font-semibold text-white hover:bg-[#36acab]"
          >
            <Link href="/dashboard">Go to Dashboard</Link>
          </Button>
          <Button
            asChild
            variant="outline"
            className="h-11 rounded-xl border-white/10 bg-white/5 px-5 text-sm font-medium text-white hover:bg-white/10 hover:text-white"
          >
            <Link href="/">Back to Home</Link>
          </Button>
        </div>
      </div>
    </div>
  );
}
