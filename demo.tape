# Emily demo — records a GIF of the language compiling a config.
Output emily.gif

Set Shell "zsh"
Set FontSize 16
Set Width 1000
Set Height 640
Set Padding 40
Set Theme "Dracula"

# Go to the project and clear for a clean start.
Type "cd ~/projects/emily_language && clear"
Enter
Sleep 500ms

# 1. Show the source: two imports, then a combined config.
Type "cat examples/config.em"
Enter
Sleep 1s

# 2. Compile it to JSON.
Type "cargo run -q -- examples/config.em"
Enter
Sleep 1500ms

# 3. Same config, YAML output.
Type "cargo run -q -- --format yaml examples/config.em"
Enter
Sleep 1500ms

# 4. A broken example, to show the line/column error message.
Type "cargo run -q -- examples/error_example.em"
Enter
Sleep 2500ms
