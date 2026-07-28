// agent-hooks-manager frontend shell - scaffold only.
// Pages to build: Hooks list | Agent status | Residue scan | Performance log | Presets.

import { invoke } from "@tauri-apps/api/core";

type Agent = string;

export default function App() {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [residue, setResidue] = useState<string[]>([]);

  async function refresh() {
    setAgents(await invoke<Agent[]>("list_agents"));
    setResidue(await invoke<string[]>("scan_residue"));
  }

  return (
    <main className="min-h-screen p-6 font-sans">
      <h1 className="text-2xl font-bold">agent-hooks-manager</h1>
      <p className="text-sm opacity-70">
        Unified hooks manager for Claude Code, Codex, Grok Build &amp; more.
      </p>

      <section className="mt-6">
        <h2 className="text-lg font-semibold">Detected agents</h2>
        <ul className="list-disc pl-6">
          {agents.map((a) => (
            <li key={a}>{a}</li>
          ))}
        </ul>
      </section>

      <section className="mt-6">
        <h2 className="text-lg font-semibold">Residue scan</h2>
        {residue.length === 0 ? (
          <p className="text-sm opacity-70">No third-party residue detected.</p>
        ) : (
          <ul className="list-disc pl-6 text-red-500">
            {residue.map((r, i) => (
              <li key={i}>{r}</li>
            ))}
          </ul>
        )}
      </section>

      <button onClick={refresh} className="mt-6 rounded bg-black px-4 py-2 text-white">
        Refresh
      </button>
    </main>
  );
}

// minimal local useState to avoid extra import noise in scaffold
import { useState } from "react";
