# Bookmark

Create static websites out of markdown

<p align="center">
  <img src="./bookmark.png" alt="bookmark"/>
</p>

# Useage

To start using it make sure you read [install bookmark](./INSTALL.md)

## Start new project

```bash
$ bookmark new <my-book> # Should see a prompt asking for the book and author names
my-book
├── book.json # Book config
└── src # All markdown file
    ├── assets/
    |    └── App.js
    └── example.md
```

## Build your project

from within your project folder where `book.json` lives you should call this command

```bash
$ bookmark build
# Output
[Done] Chapter 1 - Intorduction to rust
[Done] Chapter 2 - Closures
```

Should see the generated website in the `dist` directory

**note**: If you will commit your book into a repo make sure to ignore the `dist` folder as you can generate it anytime

## Full commands list

```bash
$ bookmark -h
bookmark
Convert markdown to production ready websites

USAGE:
    bookmark.exe <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    build    Combile markdown into HTML
    help     Print this message or the help of the given subcommand(s)
    new      Start new book
```

# Coming soon

1. Light/Dark theme
2. Sub-chapters
3. Toggle sidebar
4. Copy code snippet
5. **Proper error hanlding & error message**
