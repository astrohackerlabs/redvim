# Austin Night

RedVim's embedded normal and recovery default is `austin-night.json`.
An existing explicit choice continues to win. Select
`theme = "austin-night.json"` in your config, or remove the theme override,
to adopt the default. No configuration files are migrated or rewritten.

Astrohacker adapted Enkia's MIT Tokyo Night, retaining all 114 token rules,
their scope lists and font styles. Original `tokyo-night.json` SHA-256:
`4ef4134792f0411ee6852a52716b306d24dd344c5883fc831488ad8d44b8bbfb`.
See `licenses/enkia-tokyo-night.txt` for the original copyright and license.
Nine leading rules specify RedVim's effective Tree-sitter capture colors;
this is necessary because scope lookup takes the first matching rule.
Unused VS Code semantic-token metadata is omitted: RedVim does not consume it.

## Palette provenance

These are existing Astrohacker palette values, not a new color scheme.
Source paths below are relative to the Astrohacker business repository.

| Source | SHA-256 |
| --- | --- |
| `code/astrohacker/ts/ui/src/styles/tokens.css` | `496faf92dc5e2ab184f86ec0b1b4949fd99fbb7b637da939a0d389f47fd15b76` |
| `code/termplot/ts/termplot/app/lib/austin-night-template.ts` | `db55b8382fd7a1c2d8ea6a03e6642b47db37222c1139084b028abfed800c7b97` |
| `patches/ghostty/patches/issue-26081616315182/0031-Austin-Night-theme-and-product-sodium-yellow.patch` | `6863e82327e52a8f4c664eace75c9e8a73cb77f054c1ba8e1e0675aa1580d8b2` |
| `patches/ghostty/patches/issue-26081616315182/0032-Dock-AppIcon-plate-and-Austin-Night-111219-ground.patch` | `fb033021676e8549202dd4ddae3abd27eca2ddaddad689cdbd4e78803163cda2` |

## Effective roles

| Roles | Color |
| --- | --- |
| Editor ground | `#111219` |
| Inactive tabs, inputs, inner status | `#060608` |
| Current line, popup/dialog, active tab, inline comment | `#292e42` |
| Main text, variables, cursor, selected text | `#c0caf5` |
| Properties, secondary text, ANSI white | `#a9b1d6` |
| Comments, gutter, placeholders | `#9aa5ce` |
| Separators | `#3b4261` |
| Functions, focus, links, modified Git indicators, outer status background | `#7aa2f7` |
| Keywords | `#bb9af7` |
| Types, operators, escapes | `#7dcfff` |
| Strings, added Git indicators | `#9ece6a` |
| Constants, numbers, parameters, warnings | `#e8a84a` |
| Tags, errors, deleted Git indicators | `#f7768e` |
| Selection; active search at 128/255 opacity | `#33467c` |
| Other search matches | `#292e42` |
| Cursor text and ANSI black | `#15161e` |
| ANSI bright black | `#414868` |

Normal and bright ANSI accents share the six accent colors; bright white uses
main text. Outer status text is explicitly editor ground on blue. The intro's
existing red period is preserved by the renderer.

The remaining inherited workbench fields are consolidated to the nearest RGB
palette value, then important editor roles are assigned explicitly as above.
Inherited token colors are consolidated to the nearest of the ten text colors
from main foreground through red. For example, orange `#ff9e64` and gold
`#e0af68` become sodium, cyan variants become `#7dcfff`, and dark comment
variants are superseded by the readable leading comment rule. The JSON is the
complete per-key/per-scope mapping; no runtime color generation is required.
Tokyo Night's alpha overlays are replaced by opaque palette colors. The sole
intentional alpha is active search `#33467c80`: selection blue at 128/255,
composited by RedVim against the editor ground (`#111219`), producing
`#222c4a` (the renderer truncates channels). Other matches use raised `#292e42`.

RedVim's existing renderer adjusts text contrast for cursor, selection and
brackets, but search changes only backgrounds. The first PTY check caught
unreadable green text on the initial sodium search background. Search now uses
a dark blue overlay that preserves readable syntax colors. Unit tests check actual
parsed styles and at least 4.5:1 normal/current-line syntax, gutter, menu,
selected, status and unadjusted search text after compositing. Breadcrumb text
is explicitly muted foreground on editor ground. Separator and ANSI swatches
are not treated as ordinary editor text. The existing surface renderer may
derive additional colors for cards and state separation; this theme does not
change that shared rendering behavior.
