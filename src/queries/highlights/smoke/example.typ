// café: let set show are comments
/* outer /* nested */ still comment */
= Typst in RedVim

A paragraph with *strong text*, _emphasized text_, and `raw let text`.
See https://typst.app and @intro.

== Code <intro>
- First item
+ Numbered item
/ Term: Description

#import "helpers.typ": greet as welcome
#let answer = 42
#let distance = 12pt
#let enabled = true
#let greeting(name) = [Hello, #name!]
#let values = (1, 2, 3)
#let record = (title: "café", count: answer)
#let escaped = "quote: \" and newline: \n"
#let square = x => x * x
#set text(size: 10pt)
#show strong: it => it.body
#greeting("world")
#if enabled [Yes] else [No]

== Mathematics
Inline $x_1^2 + alpha / beta$ and display math:
$ sum_(i=1)^n i = (n(n+1))/2 $

== Embedded source
```rust
fn main() { let total = 73; }
```

```unknown-language
let unparsed = "plain raw text"
```

After raw blocks, *still strong*.
