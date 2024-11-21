-- with lspconfig
--
if require('lspconfig.configs').icalls ~= nil then
  require('lspconfig.configs').icalls = nil
end

require('lspconfig.configs').icalls_dev = {
  default_config = {
    cmd = { 'target/debug/icalls', '--stdio' },
    filetypes = { 'icalendar' },
    root_dir = function(_)
      return '/'
    end,
  },
}
require('lspconfig').icalls_dev.setup {
  -- init_options = {},
}

-- or without lspconfig
--
-- vim.lsp.start({
--   name = 'icalls',
--   cmd = { 'target/debug/icalls' },
--   root_dir = '.',
-- })

vim.lsp.set_log_level("DEBUG")
vim.keymap.set('n', 'K', vim.lsp.buf.hover, { noremap = true })
vim.keymap.set('n', 'gd', vim.lsp.buf.definition, { noremap = true })
vim.keymap.set('n', 'ga', vim.lsp.buf.code_action, { noremap = true })
