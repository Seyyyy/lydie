import type { UserConfig } from "@commitlint/types";
import { RuleConfigSeverity } from "@commitlint/types";

const Configurations: UserConfig = {
  extends: ["@commitlint/config-conventional"],
  rules: {
    "type-enum": [
      RuleConfigSeverity.Error,
      "always",
      [
        "feat",
        "fix",
        "doc",
        "perf",
        "refactor",
        "test",
        "chore",
        "security",
        "ci",
        "build",
      ],
    ],
  },
};

module.exports = Configurations;
