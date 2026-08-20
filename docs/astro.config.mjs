// @ts-check
import starlight from "@astrojs/starlight";
import { defineConfig } from "astro/config";

// https://astro.build/config
export default defineConfig({
  site: "https://google.com",
  integrations: [
    starlight({
      title: "swelog",
      description: "A Rust CLI for tracking daily accomplishments in Obsidian.",
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/RudraPatel2003/swelog-cli",
        },
      ],
      sidebar: [
        {
          label: "Getting Started",
          items: [
            { label: "Installation", slug: "getting-started/installation" },
            { label: "Quick Start", slug: "getting-started/quick-start" },
            { label: "Configuration", slug: "getting-started/configuration" },
          ],
        },
        {
          label: "Guides",
          items: [
            { label: "Daily Workflow", slug: "guides/daily-workflow" },
            { label: "Backfilling a Missed Day", slug: "guides/backfilling" },
          ],
        },
        {
          label: "Summarization",
          items: [
            { label: "Overview", slug: "summarization" },
            { label: "Providers and Models", slug: "summarization/providers" },
          ],
        },
        {
          label: "Integrations",
          items: [
            { label: "Overview", slug: "integrations" },
            { label: "GitHub", slug: "integrations/github" },
            { label: "Linear", slug: "integrations/linear" },
          ],
        },
        {
          label: "Reference",
          items: [
            { label: "Commands", slug: "reference/commands" },
            { label: "Authentication", slug: "reference/authentication" },
          ],
        },
      ],
    }),
  ],
});
