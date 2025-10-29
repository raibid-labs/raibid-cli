# Neovim Tailwind Tools - Deprecation Warning Fix

## Problem Summary

You're seeing deprecation warnings in Neovim about `require('lspconfig')` being deprecated in favor of `vim.lsp.config`. The warnings are coming from the `tailwind-tools.nvim` plugin.

```
The `require('lspconfig')` "framework" is deprecated, use vim.lsp.config
Feature will be removed in nvim-lspconfig v3.0.0
```

## Root Cause

**tailwind-tools.nvim was archived on August 29, 2025** by the owner and is now read-only. The plugin uses the old deprecated lspconfig API and will not receive updates to fix this.

- **Current version in your config**: commit `fbe982901d4508b0dcd80e07addf0fcb6dab6c49`
- **Location**: `/Users/beengud/.config/nvim/lua/layers/ui/plugins.lua:123-126`
- **Usage**: Also used in completion formatter at `/Users/beengud/.config/nvim/lua/layers/completion/configs.lua:198`

## Solutions

### Option 1: Remove tailwind-tools.nvim (Recommended if not using Tailwind)

If you're not actively working with Tailwind CSS projects, simply disable the plugin:

**File**: `/Users/beengud/.config/nvim/lua/layers/ui/plugins.lua`

```lua
cosmos.add_plugin('luckasRanarison/tailwind-tools.nvim', {
  enabled = false,  -- Add this line
  dependencies = { 'nvim-treesitter/nvim-treesitter' },
  opts = {},
})
```

Also comment out the usage in completion config:

**File**: `/Users/beengud/.config/nvim/lua/layers/completion/configs.lua:198`

```lua
-- vim_item = require('tailwind-tools.cmp').lspkind_format(idx, vim_item)
```

### Option 2: Replace with Modern Alternatives (Recommended for Tailwind users)

Replace tailwind-tools.nvim with actively maintained alternatives:

#### Step 1: Remove old plugin

In `/Users/beengud/.config/nvim/lua/layers/ui/plugins.lua`, comment out or remove:

```lua
-- cosmos.add_plugin('luckasRanarison/tailwind-tools.nvim', {
--   dependencies = { 'nvim-treesitter/nvim-treesitter' },
--   opts = {},
-- })
```

#### Step 2: Add replacement plugins

Add to the same file:

```lua
-- Tailwind color hints in completion menu
cosmos.add_plugin('roobert/tailwindcss-colorizer-cmp.nvim', {
  config = function()
    require('tailwindcss-colorizer-cmp').setup({
      color_square_width = 2,
    })
  end,
})

-- Optional: Auto-sort Tailwind classes
cosmos.add_plugin('laytan/tailwind-sorter.nvim', {
  dependencies = {'nvim-treesitter/nvim-treesitter', 'nvim-lua/plenary.nvim'},
  build = 'cd formatter && npm ci && npm run build',
  config = function()
    require('tailwind-sorter').setup({
      on_save_enabled = true,
    })
  end,
})
```

#### Step 3: Update completion formatter

In `/Users/beengud/.config/nvim/lua/layers/completion/configs.lua`, replace line 198:

```lua
-- Old:
-- vim_item = require('tailwind-tools.cmp').lspkind_format(idx, vim_item)

-- New:
vim_item = require('tailwindcss-colorizer-cmp').formatter(idx, vim_item)
```

#### Step 4: Ensure Tailwind LSP is configured

Make sure you have the Tailwind CSS language server installed via Mason:

```vim
:MasonInstall tailwindcss-language-server
```

Your LSP config should automatically pick it up if you have proper LSP setup.

### Option 3: Suppress the Warning (Temporary workaround)

**Not recommended**, but if you want to temporarily suppress the warning while deciding:

Add to your init.lua or early in your config:

```lua
-- Suppress lspconfig deprecation warnings
vim.deprecate = function() end
```

## Recommended Action

**If you use Tailwind CSS**: Go with **Option 2** - replace with modern alternatives.

**If you don't use Tailwind CSS**: Go with **Option 1** - disable the plugin.

## What Each Plugin Does

### tailwind-tools.nvim (Archived)
- LSP integration
- Color hints
- Conceal/fold support
- Utilities and commands

### Modern Replacements

**tailwindcss-colorizer-cmp.nvim**
- Adds color squares in nvim-cmp completion menu
- Shows Tailwind color previews
- Last updated: March 2024
- Status: Stable, low maintenance needed

**tailwind-sorter.nvim**
- Auto-sorts Tailwind classes (like prettier-plugin-tailwindcss)
- Works with any language via Treesitter
- Last activity: June 2025
- Status: Actively maintained

**tailwind-fold.nvim** (Already in your config!)
- Conceals long class attributes
- You already have this at line 128-133

**Mason + tailwindcss-language-server**
- Provides LSP features (autocomplete, diagnostics, hover)
- Official Tailwind support
- Standard setup via Mason

## Implementation Steps

1. Make a backup of your Neovim config
2. Choose Option 1 or Option 2 based on your usage
3. Edit the files mentioned above
4. Run `:Lazy sync` to update plugins
5. Restart Neovim
6. Verify the warnings are gone

## Additional Resources

- [tailwindcss-colorizer-cmp.nvim](https://github.com/roobert/tailwindcss-colorizer-cmp.nvim)
- [tailwind-sorter.nvim](https://github.com/laytan/tailwind-sorter.nvim)
- [Neovim LSPConfig Migration Guide](https://vi.stackexchange.com/questions/47239/how-to-properly-migrate-from-deprecated-lspconfig-setup-to-vim-lsp-config)
