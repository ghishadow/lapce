; Lower priority to prefer @parameter when identifier appears in parameter_declaration.
((identifier) @variable (#set! "priority" 95))

[
  "const"
  "default"
  "enum"
  "extern"
  "inline"
  "static"
  "struct"
  "typedef"
  "union"
  "volatile"
  "goto"
  "register"
] @keyword

"sizeof" @keyword.operator
"return" @keyword.return

[
  "while"
  "for"
  "do"
  "continue"
  "break"
] @repeat

[
 "if"
 "else"
 "case"
 "switch"
] @conditional

"#define" @constant.macro
[
  "#if"
  "#ifdef"
  "#ifndef"
  "#else"
  "#elif"
  "#endif"
  (preproc_directive)
] @keyword

"#include" @include

[ ";" ":" "," ] @punctuation.delimiter

"..." @punctuation.special

[ "(" ")" "[" "]" "{" "}"] @punctuation.bracket

[
  "="

  "-"
  "*"
  "/"
  "+"
  "%"

  "~"
  "|"
  "&"
  "^"
  "<<"
  ">>"

  "->"
  "."

  "<"
  "<="
  ">="
  ">"
  "=="
  "!="

  "!"
  "&&"
  "||"

  "-="
  "+="
  "*="
  "/="
  "%="
  "|="
  "&="
  "^="
  ">>="
  "<<="
  "--"
  "++"
] @operator

;; Make sure the comma operator is given a highlight group after the comma
;; punctuator so the operator is highlighted properly.
(comma_expression [ "," ] @operator)

[
 (true)
 (false)
] @boolean

(conditional_expression [ "?" ":" ] @conditional)

(string_literal) @string
(system_lib_string) @string
(escape_sequence) @string.escape

(null) @constant.builtin
(number_literal) @number
(char_literal) @character

[
 (preproc_arg)
 (preproc_defined)
]  @function.macro

(((field_expression
     (field_identifier) @property)) @_parent
 (#not-has-parent? @_parent template_method function_declarator call_expression))

(field_designator) @property
(((field_identifier) @property)
 (#has-ancestor? @property field_declaration)
 (#not-has-ancestor? @property function_declarator))

(statement_identifier) @label

[
 (type_identifier)
 (primitive_type)
 (sized_type_specifier)
 (type_descriptor)
] @type

(sizeof_expression value: (parenthesized_expression (identifier) @type))

((identifier) @constant
 (#lua-match? @constant "^[A-Z][A-Z0-9_]+$"))
(enumerator
  name: (identifier) @constant)
(case_statement
  value: (identifier) @constant)

;; Preproc def / undef
(preproc_def
  name: (_) @constant)
(preproc_call
  directive: (preproc_directive) @_u
  argument: (_) @constant
  (#eq? @_u "#undef"))

(call_expression
  function: (identifier) @function.call)
(call_expression
  function: (field_expression
    field: (field_identifier) @function.call))
(function_declarator
  declarator: (identifier) @function)
(preproc_function_def
  name: (identifier) @function.macro)

(comment) @comment

;; Parameters
(parameter_declaration
  declarator: (identifier) @parameter)

(parameter_declaration
  declarator: (pointer_declarator) @parameter)

(preproc_params (identifier) @parameter)

[
  "__attribute__"
  "__cdecl"
  "__clrcall"
  "__stdcall"
  "__fastcall"
  "__thiscall"
  "__vectorcall"
  "_unaligned"
  "__unaligned"
  "__declspec"
  (attribute_declaration)
] @attribute

(ERROR) @error

((identifier) @field
 (#match? @field "(^_|^m_|_$)"))

(parameter_declaration
  declarator: (reference_declarator) @parameter)
; function(Foo ...foo)
(variadic_parameter_declaration
  declarator: (variadic_declarator
                (_) @parameter))
; int foo = 0
(optional_parameter_declaration
    declarator: (_) @parameter)

;(field_expression) @parameter ;; How to highlight this?
(template_function
  name: (identifier) @function)

(template_method
  name: (field_identifier) @method)

(((field_expression
     (field_identifier) @method)) @_parent
 (#has-parent? @_parent template_method function_declarator call_expression))

(field_initializer
 (field_identifier) @property)

(function_declarator
  declarator: (field_identifier) @method)

(concept_definition
  name: (identifier) @type)

(namespace_identifier) @namespace
((namespace_identifier) @type
                        (#lua-match? @type "^[A-Z]"))
((namespace_identifier) @constant
                        (#lua-match? @constant "^[A-Z][A-Z_0-9]*$"))
(case_statement
  value: (qualified_identifier (identifier) @constant))
(namespace_definition
  name: (identifier) @namespace)

(using_declaration . "using" . "namespace" . [(qualified_identifier) (identifier)] @namespace)

(destructor_name
  (identifier) @method)

(function_declarator
      declarator: (qualified_identifier
        name: (identifier) @function))
(function_declarator
      declarator: (qualified_identifier
        name: (qualified_identifier
          name: (identifier) @function)))
((function_declarator
      declarator: (qualified_identifier
        name: (identifier) @constructor))
 (#lua-match? @constructor "^[A-Z]"))

(operator_name) @function
"operator" @function
"static_assert" @function.builtin

(call_expression
  function: (qualified_identifier
              name: (identifier) @function.call))
(call_expression
  function: (qualified_identifier
              name: (qualified_identifier
                      name: (identifier) @function.call)))
(call_expression
  function:
      (qualified_identifier
        name: (qualified_identifier
              name: (qualified_identifier
                      name: (identifier) @function.call))))

(call_expression
  function: (field_expression
              field: (field_identifier) @function.call))

((call_expression
  function: (identifier) @constructor)
(#lua-match? @constructor "^[A-Z]"))
((call_expression
  function: (qualified_identifier
              name: (identifier) @constructor))
(#lua-match? @constructor "^[A-Z]"))

((call_expression
  function: (field_expression
              field: (field_identifier) @constructor))
(#lua-match? @constructor "^[A-Z]"))

;; constructing a type in an initializer list: Constructor ():  **SuperType (1)**
((field_initializer
  (field_identifier) @constructor
  (argument_list))
 (#lua-match? @constructor "^[A-Z]"))


; Constants

(this) @variable.builtin
(nullptr) @constant

(true) @boolean
(false) @boolean

; Literals

(raw_string_literal)  @string

; Keywords

[
 "try"
 "catch"
 "noexcept"
 "throw"
] @exception


[
 "class"
 "decltype"
 "constexpr"
 "explicit"
 "final"
 "friend"
 "mutable"
 "namespace"
 "override"
 "private"
 "protected"
 "public"
 "template"
 "typename"
 "using"
 "virtual"
 "co_await"
 "concept"
 "requires"
 "consteval"
 "constinit"
 (auto)
] @keyword

[
 "co_yield"
 "co_return"
] @keyword.return

[
 "new"
 "delete"

 ;; these keywords are not supported by the parser
 ;"eq"
 ;"not_eq"
 ;
 ;"compl"
 ;"and"
 ;"or"
 ;
 ;"bitand"
 ;"bitand_eq"
 ;"bitor"
 ;"bitor_eq"
 ;"xor"
 ;"xor_eq"
] @keyword.operator

"<=>" @operator

"::" @punctuation.delimiter

(literal_suffix) @operator