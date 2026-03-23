import Link from "next/link";
import {
  ArrowUpRight,
  Bell,
  Gift,
  Sparkles,
  Trophy,
  Wallet,
  Zap,
} from "lucide-react";

const statCards = [
  { label: "Total Predictions", value: "128" },
  { label: "Accuracy Rate", value: "68.4%" },
  { label: "Current Rank", value: "#24", accent: <ArrowUpRight className="h-5 w-5 text-[#43c3be]" /> },
  { label: "Total Rewards Earned", value: "$1,240" },
];

const activePredictions = [
  {
    category: "Crypto",
    title: "BTC above $95,000 by Friday",
    prediction: "Prediction: Yes",
    status: "Live",
    ends: "Ends in 19h",
    reward: "92 XLM",
  },
  {
    category: "Crypto",
    title: "ETH/BTC ratio above 0.055",
    prediction: "Prediction: No",
    status: "Live",
    ends: "Ends in 2d 6h",
    reward: "68 XLM",
    negative: true,
  },
  {
    category: "Finance",
    title: "Fed raises rates in March",
    prediction: "Prediction: Yes",
    status: "Live",
    ends: "Ends in 8d 12h",
    reward: "156 XLM",
  },
];

const notifications = [
  { icon: ArrowUpRight, title: "Your BTC prediction settles in 3 hours", age: "2h ago" },
  { icon: Trophy, title: "You moved up 3 spots on leaderboard", age: "5h ago" },
  { icon: Gift, title: "Rewards from Weekend Market Clash claimable", age: "1d ago" },
  { icon: Bell, title: "Invite to Crypto Elite League", age: "1d ago" },
];

export default function DashboardPage() {
  return (
    <div className="min-h-full bg-[#141824] text-white">
      <section className="border-b border-white/8 px-5 py-6 sm:px-8 lg:px-10">
        <div className="flex flex-col gap-6 xl:flex-row xl:items-start xl:justify-between">
          <div>
            <h1 className="text-3xl font-semibold tracking-tight sm:text-[2.45rem]">
              Welcome back, Ayomide
            </h1>
            <p className="mt-2 max-w-2xl text-sm text-[#97a0b5] sm:text-base">
              Here&apos;s a quick overview of your prediction activity and performance.
            </p>
          </div>
          <div className="flex flex-col gap-3 sm:flex-row">
            <button
              type="button"
              className="rounded-xl bg-[#2f9e9d] px-6 py-3 text-sm font-semibold text-white transition hover:bg-[#38adaa]"
            >
              Make Prediction
            </button>
            <button
              type="button"
              className="rounded-xl border border-white/10 bg-transparent px-6 py-3 text-sm font-medium text-[#d6daea] transition hover:bg-white/5"
            >
              Create Competition
            </button>
          </div>
        </div>
      </section>

      <div className="px-5 py-6 sm:px-8 lg:px-10">
        <section className="grid gap-4 md:grid-cols-2 2xl:grid-cols-4">
          {statCards.map((card) => (
            <article
              key={card.label}
              className="rounded-[24px] border border-white/6 bg-[#242b3d] px-6 py-5 shadow-[0_12px_40px_rgba(0,0,0,0.18)]"
            >
              <p className="text-xs font-medium uppercase tracking-[0.2em] text-[#7d879c]">
                {card.label}
              </p>
              <div className="mt-4 flex items-end gap-3">
                <p className="text-4xl font-semibold tracking-tight text-white">
                  {card.value}
                </p>
                {card.accent}
              </div>
            </article>
          ))}
        </section>

        <div className="mt-6 grid gap-6 xl:grid-cols-[minmax(0,1.7fr)_minmax(320px,0.8fr)]">
          <div className="space-y-6">
            <section className="rounded-[28px] border border-white/6 bg-[#242b3d] p-7 shadow-[0_16px_50px_rgba(0,0,0,0.18)]">
              <div className="flex items-start justify-between gap-6">
                <div>
                  <h2 className="text-[2rem] font-semibold tracking-tight text-white">
                    Reputation Snapshot
                  </h2>
                </div>
                <div className="hidden items-center gap-2 rounded-full bg-white/5 px-3 py-1.5 text-sm text-[#d1b069] md:flex">
                  <Trophy className="h-4 w-4" />
                  Gold
                </div>
              </div>

              <div className="mt-7 grid gap-6 lg:grid-cols-[auto_minmax(0,1fr)_auto] lg:items-center">
                <div className="flex flex-col items-start gap-3">
                  <div className="flex h-22 w-22 items-center justify-center rounded-full bg-[#2f9e9d] text-5xl font-semibold text-white shadow-[0_10px_30px_rgba(47,158,157,0.35)]">
                    A
                  </div>
                  <span className="rounded-full bg-[#c5a766] px-4 py-2 text-xs font-semibold text-[#1d2333]">
                    Gold Predictor
                  </span>
                </div>

                <div className="space-y-5">
                  <div className="grid gap-5 sm:grid-cols-2">
                    <div>
                      <p className="text-xs uppercase tracking-[0.24em] text-[#7f8aa3]">
                        Reputation Score
                      </p>
                      <p className="mt-3 text-6xl font-semibold tracking-tight text-white">
                        840
                      </p>
                      <p className="mt-3 text-sm text-[#8a93a8]">100 to next tier</p>
                    </div>
                    <div className="flex items-start justify-start sm:justify-end">
                      <div className="rounded-2xl border border-[#c5a766]/25 bg-[#c5a766]/10 px-4 py-3 text-left sm:text-right">
                        <p className="text-xs uppercase tracking-[0.24em] text-[#c5a766]/80">
                          Tier
                        </p>
                        <p className="mt-2 text-3xl font-semibold text-[#edd69d]">Gold</p>
                      </div>
                    </div>
                  </div>

                  <div className="h-2.5 rounded-full bg-[#1b2132]">
                    <div className="h-full w-[84%] rounded-full bg-[#2f9e9d]" />
                  </div>

                  <div className="grid gap-4 md:grid-cols-2">
                    <div className="rounded-2xl bg-[#202739] px-5 py-4">
                      <p className="text-xs uppercase tracking-[0.24em] text-[#7f8aa3]">
                        Current Streak
                      </p>
                      <p className="mt-3 flex items-center gap-2 text-4xl font-semibold text-white">
                        <Zap className="h-7 w-7 text-[#d7b56b]" />
                        5
                      </p>
                      <p className="mt-2 text-sm text-[#8a93a8]">correct in a row</p>
                    </div>
                    <div className="rounded-2xl bg-[#202739] px-5 py-4">
                      <p className="text-xs uppercase tracking-[0.24em] text-[#7f8aa3]">
                        Correct Predictions
                      </p>
                      <p className="mt-3 text-4xl font-semibold text-white">87</p>
                      <p className="mt-2 text-sm text-[#8a93a8]">of 128 total</p>
                    </div>
                  </div>

                  <div className="flex flex-wrap gap-3 text-sm text-[#9aa3b8]">
                    {[
                      { icon: Trophy, label: "Top 50 Global" },
                      { icon: Zap, label: "Fast Mover" },
                      { icon: Sparkles, label: "Data Driven" },
                    ].map(({ icon: Icon, label }) => (
                      <div
                        key={label}
                        className="inline-flex items-center gap-2 rounded-xl bg-[#202739] px-4 py-3"
                      >
                        <Icon className="h-4 w-4 text-[#4fd1c5]" />
                        <span>{label}</span>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </section>

            <section>
              <div className="mb-4 flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
                <h2 className="text-[2rem] font-semibold tracking-tight text-white">
                  Active Predictions
                </h2>
                <Link
                  href="/my-predictions"
                  className="inline-flex items-center gap-2 text-sm font-medium text-[#2f9e9d]"
                >
                  View All Predictions
                  <ArrowUpRight className="h-4 w-4" />
                </Link>
              </div>
              <div className="grid gap-4 lg:grid-cols-3">
                {activePredictions.map((item) => (
                  <article
                    key={item.title}
                    className="rounded-[24px] border border-white/6 bg-[#242b3d] p-5"
                  >
                    <p className="text-xs font-medium uppercase tracking-[0.2em] text-[#7d879c]">
                      {item.category}
                    </p>
                    <h3 className="mt-3 text-xl font-semibold leading-8 text-white">
                      {item.title}
                    </h3>
                    <div className="mt-5 space-y-3 text-sm text-[#97a0b5]">
                      <p className="flex items-center gap-2">
                        <span
                          className={`h-2.5 w-2.5 rounded-full ${
                            item.negative ? "bg-[#dc7a73]" : "bg-[#3dc19a]"
                          }`}
                        />
                        {item.prediction}
                      </p>
                      <p className="flex items-center gap-2">
                        <span className="h-2.5 w-2.5 rounded-full bg-[#3dc19a]" />
                        {item.status}
                      </p>
                      <p>{item.ends}</p>
                    </div>
                    <div className="mt-5 border-t border-white/8 pt-4">
                      <p className="text-xs uppercase tracking-[0.2em] text-[#7d879c]">
                        Potential Reward
                      </p>
                      <p className="mt-2 text-3xl font-semibold text-[#d5b46c]">
                        {item.reward}
                      </p>
                    </div>
                  </article>
                ))}
              </div>
            </section>
          </div>

          <div className="space-y-6">
            <section className="rounded-[28px] border border-[#c5a766]/55 bg-[#242b3d] p-6 shadow-[0_16px_50px_rgba(0,0,0,0.18)]">
              <div className="flex items-start justify-between gap-4">
                <h2 className="text-2xl font-semibold tracking-tight text-white">
                  Rewards Wallet
                </h2>
                <div className="rounded-xl bg-[#1d2333] p-2 text-[#d5b46c]">
                  <Wallet className="h-5 w-5" />
                </div>
              </div>
              <p className="mt-6 text-center text-xs uppercase tracking-[0.24em] text-[#7f8aa3]">
                Total Earned
              </p>
              <div className="mt-3 flex items-center justify-center gap-3 text-[#f1d37e]">
                <Sparkles className="h-8 w-8" />
                <p className="text-5xl font-semibold">$1,240</p>
                <Sparkles className="h-8 w-8" />
              </div>
              <div className="mt-8 space-y-4 text-sm text-[#9ba4b8]">
                <div className="flex items-center justify-between">
                  <span>Claimable Rewards</span>
                  <span className="font-semibold text-[#4fd1c5]">$150</span>
                </div>
                <div className="flex items-center justify-between">
                  <span>Pending Payouts</span>
                  <span className="font-semibold text-[#d5b46c]">$95</span>
                </div>
                <div className="flex items-center justify-between">
                  <span>Wallet Balance</span>
                  <span className="font-semibold text-[#2f9e9d]">420 XLM</span>
                </div>
              </div>
              <button
                type="button"
                className="mt-8 w-full rounded-xl bg-[#c5a766] px-5 py-3.5 text-sm font-semibold text-[#1d2333] transition hover:bg-[#d6ba7e]"
              >
                Claim Rewards
              </button>
            </section>

            <section className="rounded-[28px] border border-white/6 bg-[#242b3d] p-6">
              <div className="flex items-center justify-between gap-4">
                <h2 className="text-2xl font-semibold tracking-tight text-white">
                  Notifications
                </h2>
                <div className="flex gap-2">
                  <span className="h-2.5 w-2.5 rounded-full bg-[#2f9e9d]" />
                  <span className="h-2.5 w-2.5 rounded-full bg-[#d37f79]" />
                </div>
              </div>

              <div className="mt-6 space-y-4">
                {notifications.map(({ icon: Icon, title, age }) => (
                  <div
                    key={title}
                    className="flex gap-4 border-b border-white/8 pb-4 last:border-b-0 last:pb-0"
                  >
                    <div className="mt-0.5 rounded-xl bg-[#202739] p-3 text-[#4fd1c5]">
                      <Icon className="h-5 w-5" />
                    </div>
                    <div>
                      <p className="text-sm leading-7 text-[#dce1ec]">{title}</p>
                      <p className="mt-1 text-xs text-[#7f8aa3]">{age}</p>
                    </div>
                  </div>
                ))}
              </div>
            </section>
          </div>
        </div>
      </div>
    </div>
  );
}
