-- rust-resource-pulsar-library main module.
-- Renders the _messaging Cargo crate (Cargo.toml + lib.rs + settings.rs).
--
-- The calling archetype is responsible for:
--   1. Declaring the messaging crate in the workspace Cargo.toml members list
--   2. Adding the messaging crate dep to the binary crate ({% if has_messaging %} block)
--
-- API:
--   local pulsar = require("rust-resource-pulsar")
--   pulsar.render(context, { destination = context:get("project-name") })
--
-- Context contract (prompt() fills if absent):
--   prefix-name     — kebab-case first segment (e.g. "billing")
--   suffix-name     — kebab-case second segment (e.g. "service")
--   prefix_name     — snake_case first segment (e.g. "billing")
--   suffix_name     — snake_case second segment (e.g. "service")
--   project-name    — kebab-case full name (e.g. "billing-service")

local M = {}

function M.prompt(context)
    if not context:get("prefix-name") then
        context:prompt_text("Service Prefix:", "prefix_name", {
            cases = Cases.programming(),
            placeholder = "billing",
        })
    end
    if not context:get("suffix-name") then
        context:prompt_select("Service Suffix:", "suffix_name", {
            "service", "orchestrator", "adapter",
        }, { default = "service", cases = Cases.programming() })
    end
    if not context:get("project-name") then
        context:set("project-name", context:get("prefix-name") .. "-" .. context:get("suffix-name"))
    end
    return context
end

function M.render(context, opts)
    opts = opts or {}
    local d = opts.destination
    if d and d ~= "" then
        directory.render("contents", context, { destination = d })
    else
        directory.render("contents", context)
    end
    return context
end

function M.run(context, opts)
    M.prompt(context, opts)
    M.render(context, opts)
    return context
end

return M
