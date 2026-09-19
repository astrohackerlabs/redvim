; Adapted from the pinned official GPUWeb query. Match leaf names rather than
; whole declarations/types so initializers and template/attribute arguments
; retain their independent styles. Provenance and licenses ship with RedVim.
(ident) @variable
(type_specifier (template_elaborated_ident (ident) @type))
(type_alias_decl (ident) @type)
(struct_decl (ident) @type)
(function_header (ident) @function)
(function_header (template_elaborated_ident (ident) @type))
(call_phrase (template_elaborated_ident (ident) @function))
(param (ident) @variable.parameter)
(member_ident) @property
(swizzle_name) @property

((ident) @type.builtin
 (#match? @type.builtin "^(bool|i32|u32|f32|f16|vec[234][fhiu]?|mat[234]x[234][fh]?|array|atomic|ptr|sampler|sampler_comparison|texture_[a-z0-9_]+)$"))

[(line_comment) (block_comment)] @comment
(bool_literal) @constant.builtin
(int_literal) @number
(float_literal) @float
(builtin_value_name) @constant.builtin
(interpolate_type_name) @constant.builtin
(interpolate_sampling_name) @constant.builtin

["var" "let" "const" "override" "fn" "struct" "alias"
 "if" "else" "loop" "for" "while" "switch" "case" "default"
 "break" (continue_statement) "continuing" "return" "discard"] @keyword
["enable" "requires" "diagnostic" "const_assert"] @keyword.directive
["@" "align" "binding" "blend_src" "builtin" "group" "id"
 "interpolate" "invariant" "location" "must_use" "size"
 "workgroup_size" "vertex" "fragment" "compute"] @keyword.directive
(attribute (ident_pattern_token) @keyword.directive)

["(" ")" "[" "]" "{" "}" (template_args_start) (template_args_end)] @punctuation.bracket
["," "." ":" ";"] @punctuation.delimiter
["+" "-" "*" "/" "%" "!" "~" "&" "|" "^" "==" "!="
 "&&" "||" "=" "+=" "-=" "*=" "/=" "%=" "&=" "|=" "^="
 "++" "--" "->" (less_than) (less_than_equal) (greater_than)
 (greater_than_equal) (shift_left) (shift_right) (shift_left_assign)
 (shift_right_assign)] @operator
