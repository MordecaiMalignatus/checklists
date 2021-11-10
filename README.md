# Checklist Stepper

This is a WIP title for a thing I've been working on. The long story short is
that it steps through checklists.

This is done by presenting a TUI with a single pane, one step per. That way you
don't skip steps, and you don't miss them having to parse a block of text.

The actual functionality is deliberately skeletal. There's two ways to get steps
into `cls`:

```
$ printf "foo\nbar\nbaz" | cls
$ cls -f ./prepared-checklist.txt
```

Both of those will present you with your list, one item at a time.

## Internals and repurposing

The logic that relates to displaying steps and handling the interface is
deliberately kept separate in a crate. This way, you can adapt the general
framework with relatively little effort.

`cls`, the binary, is intended to be a thin wrapper with CLI parsing and such
over the library.

## Things to do

- checklists in markdown extractor
- Add ending slide
- Allow me to mark slides as complete, and have a modal filtering between
  complete being in and excluded from display.
