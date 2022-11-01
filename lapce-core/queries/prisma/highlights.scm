; source: https://github.com/helix-editor/helix/blob/master/runtime/queries/prisma/highlights.scm
; licence: https://github.com/helix-editor/helix/blob/master/LICENSE
; spdx: MPL-2.0

(string) @string

(enumeral) @constant
(number) @constant.numeric

(variable) @variable
(column_type) @type

(arguments) @variable.other.member
(model_declaration (identifier) @type)

[
 "datasource"
 "enum"
 "generator"
 "model"
 "type"
] @keyword

[
 (comment)
 (developer_comment)
] @comment

[
 (attribute)
 (block_attribute_declaration)
 (call_expression)
] @function.builtin

[
 (true)
 (false)
 (null)
] @constant.builtin.boolean

[
 "("
 ")"
 "["
 "]"
 "{"
 "}"
] @punctuation.bracket

[
 ":" 
 ","
] @punctuation.delimiter

[
 "="
 "@"
 "@@"
 (binary_expression)
] @operator