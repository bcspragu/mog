# mog

mog is an emoji picker. You search for your emoji, then hit enter to print it out to the terminal.

## Usage

```bash
# Brings up the picker
mog

# If only one match, just prints it out, otherwise brings up the picker with that pre-filled.
mog <term>

# Search for an emoji and copy it to your clipboard (assuming `xsel` is installed)
EMOJI=$(mog)
echo $EMOJI | xsel -b
```

## Why "mog"?

Spell it out :)

## Thoughts

This project was mostly started as an excuse to play around with [Aider](https://aider.chat/). The initial commits were generated by Aider using Claude Sonnet 3.5. The inital prompt was something like:

```
Write a Rust TUI application using the Ratatui crate. The TUI application is a
fuzzy-finding emoji picker. As a one-time process, it should read, parse, and
index the `emoji-slim.json` file using the Rust Tantivy full-text search engine,
and write that index out to disk. The TUI should read that index, and then
present an input for the user to filter emojis.
```

And the result was pretty good! I forgot to add the schema of `emoji-slim.json`, so that was all wrong, but it wasn't a bad start.

Tantivy wasn't a great match for the way people search for emojis, it's a better fit for search engine-like searches. That said, it still does pretty well, and it is definitely fast!

But a fuzzy finder library is a better fit here, so I added a second backend based on[nucleo](https://github.com/helix-editor/nucleo).

## TODO

- [ ] Replace dataset with Emojibase
- [ ] Make the UI nicer
- [ ] Figure out the graphical glitches

## Attribution

- Emoji data from: https://github.com/iamcal/emoji-data
- Tantivy for indexing: https://github.com/quickwit-oss/tantivy
- Ratatui for the TUI bits: https://github.com/ratatui/ratatui
