; Pinned official Caddyfile query adapted for RedVim. Opaque literal/argument
; content remains host-colored; no embedded-language injection is claimed.
(comment) @comment
[
  (environment_variable)
  (placeholder)
] @constant

[
  (network_address)
  (ip_address_or_cidr)
  (path)
  (path_matcher)
] @type

[
  (named_route_identifier)
  (site_address)
] @keyword
(snippet_name) @function

(directive (directive_name) @property)

; declaration of a named matcher
(named_matcher (matcher_identifier (matcher_name)) @function.macro)

; reference to a named matcher
(matcher (matcher_identifier (matcher_name)) @function.macro)

; directive within a named matcher declaration
(matcher_directive (matcher_directive_name) @function.method)

; Keep path matchers as paths; only a bare wildcard gets the matcher color.
(matcher "*" @function.macro)

[
  (interpreted_string_literal)
  (raw_string_literal)
  (argument)
  (heredoc_body)
  (cel_expression)
] @string
(escape_sequence) @string.escape
[(heredoc_start) (heredoc_end)] @constant
"<<" @punctuation.delimiter

[
  (duration_literal)
  (int_literal)
  (status_code_fallback)
] @number

[
  "{"
  "}"
] @punctuation.bracket
