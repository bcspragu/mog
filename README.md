# mog

mog is an emoji picker. You search for your emoji, then hit enter to print it out to the terminal.

## Usage

```bash
# Brings up the picker
mog

# If only one match, just prints it out, otherwise brings up the picker with that pre-filled.
mog <term>

# Search for an emoji and copy it to your clipboard (assuming `xsel` is installed)
mog | xsel -b
```

## Thoughts

Tantivy wasn't a great choice here, as I should have just used a fast fuzzy library like [nucleo](https://github.com/helix-editor/nucleo), but it was at least educational for me.

## Attribution

- Emoji data from: https://github.com/iamcal/emoji-data
- Tantivy for indexing: https://github.com/quickwit-oss/tantivy
- Ratatui for the TUI bits: https://github.com/ratatui/ratatui
