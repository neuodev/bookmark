# Install Bookmark

1. Clone the repo
   ```bash
   git clone https://github.com/AhmedIbrahim336/bookmark.git
   ```
2. Make sure you have [rust](https://www.rust-lang.org/tools/install) intalled

   ```bash
   cargo build --release
   ```

3. Add `/target/release/` to your `PATH`

   - Windows: Run `Command Prompt` as and administrator then run this command
     ```sh
        setx path "%PATH%;<BOOKMARK>"
     ```
     make sure to repleace `<BOOKMARK>` with the absoule path to the `release` directory above
   - MacOS or Linux
     ```bash
     export PATH="$HOME/release:$PATH"
     ```
