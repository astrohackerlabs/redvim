#!/usr/bin/env nu
# café 🙂: bundled Nushell highlighting
def greet [name: string, --loud] {
  let message = $"Hello, ($name)!"
  if $loud { $message | str upcase } else { $message }
}
let items = [1 2 3]
let settings = {delay: 5sec, size: 10kb, enabled: true}
$items | each {|item| $item * 2 }
greet "café 🙂" --loud
