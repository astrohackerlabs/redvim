[
  "FROM"
  "AS"
  "RUN"
  "CMD"
  "LABEL"
  "EXPOSE"
  "ENV"
  "ADD"
  "COPY"
  "ENTRYPOINT"
  "VOLUME"
  "USER"
  "WORKDIR"
  "ARG"
  "ONBUILD"
  "STOPSIGNAL"
  "HEALTHCHECK"
  "SHELL"
  "MAINTAINER"
  "CROSS_BUILD"
] @keyword

[
  ":"
  "@"
] @operator

(comment) @comment

(image_spec
  (image_tag
    ":" @punctuation.delimiter)
  (image_digest
    "@" @punctuation.delimiter))

[
  (double_quoted_string)
  (single_quoted_string)
  (json_string)
] @string

(heredoc_content) @string

[
  (heredoc_marker)
  (heredoc_end)
] @constant

(escape_sequence) @string.escape

(expansion
  [
    "$"
    "{"
    "}"
  ] @punctuation.delimiter
)

(expansion_operator) @operator

(variable) @constant

(arg_pair
  name: (unquoted_string) @property)

(env_pair
  name: (unquoted_string) @property)

(label_pair
  key: (_) @property)

; Generic flag names are hidden tokens in the published generated parser.
; Color the complete flag; nested variable constants retain their own color.
(param) @property

(mount_param
  name: _ @property)

(mount_param_param) @property

(expose_port) @number

; RedVim host-language additions; no shell-language injection.
(image_name) @string.special
(image_alias) @type
(line_continuation) @punctuation.delimiter
