# Phase 1 · High-Level Language Specification

Version: 1.0 frozen baseline  
Scope: high-level language design only  
Model: source-first scripting language with internal execution representation  
Out of scope: IR format, VM architecture, parser algorithm, optimizer behavior, bytecode, implementation roadmap

---

## 0. Freeze Status

This document is the frozen Phase 1 high-level language baseline.

After this freeze, Phase 1 language features are not expanded casually. Changes to this document require an explicit amendment with one of the following justifications:

1. resolving an internal contradiction in the language specification
2. closing a grammar or semantic incompleteness discovered during IR design
3. correcting a safety, capability, or foreign-boundary flaw
4. clarifying wording without changing semantics

New feature expansion is deferred until after Phase 2 IR design has validated the existing language surface.

---

## 1. Normative Scope

This document specifies the high-level source language.

It defines:

1. source text structure
2. lexical grammar
3. indentation and block structure
4. declarations
5. statements
6. expressions
7. value kinds
8. binding and scope
9. function semantics
10. record semantics
11. container semantics
12. module semantics
13. error semantics
14. diagnostics
15. unsupported constructs

It does not define:

1. AST representation
2. IR representation
3. bytecode
4. VM instruction format
5. VM dispatch strategy
6. memory management
7. optimization passes
8. parser implementation
9. package manager implementation
10. execution cache format

Source text is the only stable program artifact described by this language specification.

Any internal IR used by an implementation is not part of this document and is not a public bytecode layer.

---

## 2. Design Corrections Against Common Dynamic-Language Failure Modes

The language remains script-oriented, but it deliberately avoids several failure modes historically exposed by Python and other dynamic languages.

The following rules are normative language design constraints.

### 2.1 No Implicit New Binding by Assignment

A new binding must be introduced with `let`, `const`, `def`, `record`, `import`, or `export`.

Plain assignment updates an existing mutable binding.

This avoids accidental local shadowing and Python-style unbound-local ambiguity.

### 2.2 Block Scope Exists

Blocks introduce lexical scopes.

Names declared inside `if`, `while`, `for`, `try`, `catch`, and nested blocks do not leak into the surrounding scope.

This avoids loop-variable leakage and branch-local accidental reuse.

### 2.3 Conditions Must Be Boolean

`if`, `elif`, `while`, logical `and`, logical `or`, and logical `not` operate on `Bool`.

The language does not use generalized truthiness in control-flow conditions.

This avoids bugs caused by empty strings, empty containers, numeric zero, or `nil` silently controlling flow.

### 2.4 No Implicit Type Coercion

The language does not implicitly convert between strings, numbers, booleans, nil, or containers.

String concatenation requires strings.

Numeric arithmetic requires numeric operands.

Boolean operations require booleans.

Display conversion is performed only by explicitly specified display contexts such as `print`.

### 2.5 Integer Arithmetic Must Not Silently Wrap

Integer arithmetic is exact at the language level.

If an implementation uses bounded integers, overflow must raise `NumericOverflowError`.

Silent wraparound is not conforming behavior.

### 2.6 Function Defaults Are Not Persistent Mutable Objects

Default parameter expressions are evaluated at call time, not at function-definition time.

Each omitted argument evaluates its default expression anew.

This avoids the mutable-default-argument trap.

### 2.7 Records Have Fixed Shape

User-defined aggregate data uses `record`.

Record fields are declared explicitly.

Instances cannot receive undeclared fields dynamically.

This avoids arbitrary monkey-patching, unstable object layout, and accidental attribute creation.

### 2.8 Module Exports Are Explicit

A module exposes only names marked with `export`.

Wildcard import is not defined.

This avoids accidental API leakage and import-order-dependent public interfaces.

### 2.9 Errors Are Structured Values

Runtime errors are structured values with stable fields.

Bare catch-all exception swallowing is not defined.

This avoids broad, silent exception masking.

### 2.10 No CPython C-Extension Compatibility

The language does not support CPython C-extension compatibility.

A conforming implementation is not required or expected to support:

1. CPython C API
2. CPython ABI
3. Python binary wheels
4. Python extension modules
5. CPython object layout
6. CPython reference-counting semantics
7. CPython GIL semantics
8. direct `PyObject*` interoperation

Foreign code may be exposed only through separately specified, capability-gated foreign interfaces, host modules, WASM modules, or a future native module ABI designed for this language.

Native interoperation must not become a backdoor that bypasses the language capability model by default.

### 2.11 Equality and Identity Are Distinct

`==` is value equality.

`is` is identity equality.

The language does not conflate object identity with structural equality.

---

## 3. Source Text

### 3.1 Source File

A source file is a module.

A module is executed by evaluating its top-level declarations and statements in source order, subject to module import semantics.

There is no required `main` function.

### 3.2 Encoding

Source text must be valid UTF-8.

Ill-formed UTF-8 is a lexical error.

### 3.3 Unicode Normalization

Identifiers are normalized to NFC for comparison.

A source file containing two identifier spellings that normalize to the same identifier in the same scope is invalid.

Implementations should diagnose mixed-script identifiers when they are visually confusable, but confusable detection is not required for conformance.

### 3.4 Line Terminators

Recognized physical line terminators:

```text
LF
CRLF
CR
```

All line terminators are normalized to logical newline.

### 3.5 Comments

A comment begins with `#` outside a string literal and continues to the end of the physical line.

```python
# comment
let x = 1  # trailing comment
let s = "# not comment"
```

### 3.6 Blank Lines

Blank lines contain only whitespace and/or comments.

Blank lines do not produce statements and do not affect indentation.

### 3.7 Logical Lines

A logical line normally corresponds to one physical line.

A physical newline inside parentheses, brackets, or braces does not terminate the logical line.

---

## 4. Indentation and Blocks

### 4.1 Indentation

The language is indentation-sensitive.

Compound headers end with `:` and are followed by an indented block.

```python
if x > 0:
    print(x)
```

### 4.2 Spaces Only

Leading indentation is measured in spaces.

Tabs in leading indentation are invalid.

### 4.3 Indentation Stack

The lexer maintains an indentation stack.

The initial indentation level is `0`.

At each non-blank logical line outside continuation context:

1. if indentation equals the top level, no token is emitted
2. if indentation is greater, emit `INDENT`
3. if indentation is smaller, emit one or more `DEDENT`
4. if the smaller indentation does not match a previous level, raise `IndentationError`

### 4.4 Empty Blocks

Empty blocks are invalid.

The language does not define `pass`.

---

## 5. Lexical Tokens

### 5.1 Token Categories

```text
identifier
keyword
integer literal
float literal
string literal
operator
delimiter
NEWLINE
INDENT
DEDENT
EOF
```

### 5.2 Identifiers

Identifiers are case-sensitive.

Grammar:

```text
identifier_start    ::= "_" | ASCII_ALPHA | Unicode_XID_Start
identifier_continue ::= "_" | ASCII_ALNUM | Unicode_XID_Continue
identifier           ::= identifier_start { identifier_continue }
```

An identifier must not be a reserved keyword.

### 5.3 Reserved Keywords

Defined keywords:

```text
and
as
assert
break
catch
const
continue
def
defer
elif
else
enum
export
false
field
finally
for
from
if
import
in
is
let
match
mutable
nil
not
or
raise
readonly
record
return
true
try
use
while
```

Contextual keywords:

```text
case
default
doc
effect
requires
returns
test
```

Contextual keywords have special meaning only in specific grammar positions.

Reserved for future use:

```text
async
await
class
enum
global
lambda
nonlocal
static
trait
type
where
yield
```

### 5.4 Delimiters

```text
( )
[ ]
{ }
, :
.
..
->
|
?
```

### 5.5 Operators

```text
+
-
*
/
%
=
+=
-=
*=
/=
%=
==
!=
<
<=
>
>=
```

Keyword operators:

```text
and
or
not
is
in
```

---

## 6. Literals

### 6.1 Nil

```python
nil
```

`nil` denotes the singleton nil value.

### 6.2 Booleans

```python
true
false
```

Booleans are distinct from integers.

`true == 1` evaluates to `false`.

`false == 0` evaluates to `false`.

### 6.3 Integers

Integer literals are decimal.

```text
integer_literal ::= "0" | nonzero_digit { digit | "_" digit }
```

Leading zeros are invalid except for `0`.

Valid:

```python
0
1
123
1_000_000
```

Invalid:

```python
01
1_
1__0
```

The sign is not part of an integer literal.

### 6.4 Floats

Float literals represent binary64 floating-point values.

```text
float_literal ::=
    digits "." digits [ exponent ]
  | digits exponent
```

Valid:

```python
1.0
0.5
10e3
1.5e-2
```

Invalid:

```python
1.
.5
```

### 6.5 Strings

String literals may be single-quoted or double-quoted.

```python
"hello"
'world'
```

Supported escapes:

```text
\\
\'
\"
\n
\r
\t
\0
\u{H...H}
```

A string literal may not contain an unescaped physical newline.

Triple-quoted strings are not defined.

Raw strings are not defined.

### 6.6 Lists

List literals use brackets.

```python
let xs = [1, 2, 3]
let empty = []
```

A trailing comma is permitted.

```python
let xs = [
    1,
    2,
    3,
]
```

Lists are ordered mutable sequences.

### 6.7 Maps

Map literals use braces and colon-separated key/value pairs.

```python
let m = {"a": 1, "b": 2}
let empty = {}
```

A trailing comma is permitted.

Map keys must be hashable.

Hashable key kinds:

```text
Nil
Bool
Int
String
```

Floats are not hashable keys in this language.

Lists, maps, records, functions, and errors are not hashable keys unless a later specification defines explicit hash behavior.

### 6.8 Record Construction Literals

Record instances are constructed by calling a record constructor using named field arguments.

```python
let p = Point(x = 1, y = 2)
```

All required fields must be provided unless the record declares field defaults.

Unknown field names are errors.

Duplicate field names are errors.

---

## 7. Grammar Notation

The grammar is descriptive EBNF.

```text
A ::= B        definition
A | B         alternative
{ A }         zero or more
[ A ]         optional
"token"       literal token
IDENT         identifier token
NEWLINE       logical line terminator
INDENT        indentation increase
DEDENT        indentation decrease
EOF           end of file
```

---

## 8. Module Grammar

```text
module ::= { top_level_item } EOF
```

```text
top_level_item ::= module_prelude_item | declaration | statement | test_block

module_prelude_item ::= requires_clause
```

Top-level statements are permitted.

Top-level `return`, `break`, and `continue` are invalid.

---


## 8.1 Grammar Closure Notes

The following grammar integration rules are normative.

1. `enum_definition` is a declaration.
2. `match_statement` and `use_statement` are compound statements.
3. `defer_statement` and `assert_statement` are simple statements.
4. `test_block` is a top-level item and is not an ordinary runtime statement.
5. `requires_clause` is a module prelude item.
6. `effect_clause` is attached to function and method definitions.
7. `case` is contextual:
   - inside an `enum` body, `case` declares an enum case
   - inside a `match` body, `case` declares a pattern case
8. `field` is contextual inside `record` bodies.
9. `doc`, `effect`, `requires`, `returns`, and `test` are contextual grammar words where specified.

A contextual keyword may be used as an identifier outside its grammar context only if doing so does not conflict with the lexical keyword set.

## 9. Declarations

```text
declaration ::=
    let_declaration
  | const_declaration
  | function_definition
  | record_definition
  | enum_definition
  | import_declaration
  | export_declaration
```

Declarations introduce bindings.

A declaration may appear at module scope or block scope unless otherwise specified.

### 9.1 Let Declaration

```text
let_declaration ::= "let" binding_pattern [ type_annotation ] "=" expression NEWLINE
```

```text
type_annotation ::= ":" type_expression
```

`let` introduces a mutable binding.

```python
let x = 1
let count: Int = 0
x = 2
```

A `let` binding may be reassigned.

If a type annotation is present, every value assigned to the binding must satisfy the declared type contract.

### 9.2 Const Declaration

```text
const_declaration ::= "const" binding_pattern [ type_annotation ] "=" expression NEWLINE
```

`const` introduces an immutable binding.

```python
const pi: Float = 3.14159
const name: String = "core"
```

A `const` binding cannot be reassigned.

If a type annotation is present, the initializer value must satisfy the declared type contract.

`const` is binding immutability, not deep value immutability.

If a const binding refers to a mutable list, the binding cannot be rebound, but the list may still be mutated through that reference.

### 9.3 Function Definition

```text
function_definition ::=
    "def" IDENT "(" [ parameter_list ] ")" ":" block
```

A function definition introduces an immutable binding equivalent to a `const` function binding.

Reassigning the function name in the same scope is invalid.

### 9.4 Record Definition

```text
record_definition ::= "record" IDENT ":" record_block
```

```text
record_block ::= NEWLINE INDENT { record_member } DEDENT
```

```text
record_member ::=
    field_declaration
  | method_definition
```

A record definition introduces an immutable binding for the record type.

### 9.5 Enum Definition

```text
enum_definition ::= "enum" IDENT ":" enum_block
```

```text
enum_block ::= NEWLINE INDENT { enum_case } DEDENT
```

```text
enum_case ::= "case" IDENT [ "(" [ enum_payload_list ] ")" ] NEWLINE

enum_payload_list ::= enum_payload { "," enum_payload } [ "," ]

enum_payload ::= IDENT type_annotation
```

An enum definition introduces an immutable binding for a closed nominal sum type.

Each case introduces a constructor under the enum type namespace.

Example:

```python
enum Result:
    case Ok(value: String)
    case Err(error: Error)
```

Enum cases are closed. New cases cannot be added outside the enum definition.

Case names must be unique within an enum.

Payload field names must be unique within a case.

Enum values are immutable.

Enum cases are intended to be used with `match`.

Example:

```python
let r = Result.Ok(value = "done")

match r:
    case Result.Ok(value = text):
        print(text)
    case Result.Err(error = e):
        raise e
```

### 9.6 Import Declaration

```text
import_declaration ::=
    "import" module_path [ "as" IDENT ] NEWLINE
  | "from" module_path "import" import_list NEWLINE
```

```text
module_path ::= IDENT { "." IDENT }
import_list ::= import_item { "," import_item } [ "," ]
import_item ::= IDENT [ "as" IDENT ]
```

Wildcard import is not defined.

### 9.7 Export Declaration

```text
export_declaration ::=
    "export" ( let_declaration | const_declaration | function_definition | record_definition | enum_definition )
```

`export` marks the introduced binding as part of the module's public interface.

Only exported names are importable from another module.

---

## 10. Statements

```text
statement ::=
    simple_statement
  | compound_statement
```

```text
simple_statement ::=
    assignment_statement NEWLINE
  | augmented_assignment_statement NEWLINE
  | expression_statement NEWLINE
  | return_statement NEWLINE
  | break_statement NEWLINE
  | continue_statement NEWLINE
  | raise_statement NEWLINE
  | assert_statement NEWLINE
  | defer_statement NEWLINE
```

```text
compound_statement ::=
    if_statement
  | while_statement
  | for_statement
  | try_statement
  | match_statement
  | use_statement
```

Declarations may appear where statements appear.

### 10.1 Assignment

```text
assignment_statement ::= assignment_target "=" expression
```

```text
assignment_target ::= IDENT | attribute_target | index_target
```

Plain name assignment requires the name to resolve to an existing mutable binding.

Assignment does not create a new binding.

Invalid:

```python
x = 1  # invalid if x was not previously declared
```

Valid:

```python
let x = 1
x = 2
```

### 10.2 Augmented Assignment

```text
augmented_assignment_statement ::= assignment_target augmented_operator expression
```

```text
augmented_operator ::= "+=" | "-=" | "*=" | "/=" | "%="
```

`a += b` is semantically equivalent to evaluating `a + b` and assigning the result back to `a`, except that the assignment target is evaluated only once.

The target must be assignable.

### 10.3 Expression Statement

```text
expression_statement ::= expression
```

The result is discarded.

Expression statements do not automatically print.

### 10.4 Return

```text
return_statement ::= "return" [ expression ]
```

`return` is valid only inside a function or method body.

If no expression is provided, the returned value is `nil`.

### 10.5 Break

```text
break_statement ::= "break"
```

`break` is valid only inside `while` or `for`.

### 10.6 Continue

```text
continue_statement ::= "continue"
```

`continue` is valid only inside `while` or `for`.

### 10.7 Raise

```text
raise_statement ::= "raise" expression
```

The expression must evaluate to an `Error` value.

Raising a non-error value is a runtime type error.

---

### 10.8 Assert

```text
assert_statement ::= "assert" expression [ "," expression ]
```

The first expression must evaluate to `Bool`.

If the value is `true`, execution continues.

If the value is `false`, an `AssertionError` is raised.

If the optional second expression is present, it must evaluate to `String` and becomes the assertion message.

Example:

```python
assert x > 0, "x must be positive"
```

Assertions are part of the language semantics.

A conforming implementation must not silently remove assertion checks unless it is explicitly running in a separately specified unchecked mode.


## 11. Blocks and Scope

```text
block ::= NEWLINE INDENT { declaration | statement } DEDENT
```

A block introduces a lexical block scope.

Names declared in a block are visible from the point of declaration to the end of that block and to nested blocks.

Names declared in a block are not visible outside that block.

Example:

```python
if flag:
    let x = 1

print(x)  # name error
```

Function bodies introduce function scopes, which are also block scopes.

Record bodies introduce record-member declaration scopes.

Catch clauses introduce catch scopes.

---

## 12. Control Statements

### 12.1 If

```text
if_statement ::=
    "if" expression ":" block
    { "elif" expression ":" block }
    [ "else" ":" block ]
```

The condition expression must evaluate to `Bool`.

No other value kind is accepted as a condition.

### 12.2 While

```text
while_statement ::= "while" expression ":" block
```

The condition expression must evaluate to `Bool`.

### 12.3 For

```text
for_statement ::= "for" IDENT "in" expression ":" block
```

The iterable expression must evaluate to an iterable value.

Built-in iterable values:

```text
List
Map
Range
```

Strings are not iterable by default.

This avoids accidental character iteration.

Map iteration yields keys in insertion order.

The loop variable is a fresh immutable binding scoped to each iteration body.

Example:

```python
for x in xs:
    print(x)
```

The loop variable is not visible after the loop.

### 12.4 Try / Catch / Finally

```text
try_statement ::=
    "try" ":" block
    catch_clause
    [ finally_clause ]
```

```text
catch_clause ::= "catch" IDENT [ "if" expression ] ":" block
finally_clause ::= "finally" ":" block
```

The catch identifier binds the caught `Error` value.

If a catch guard is present, it must evaluate to `Bool`.

A `catch` clause catches runtime errors raised by `raise` or by language runtime operations.

Syntax errors, indentation errors, and static semantic errors are not catchable.

`finally` executes after the try/catch control path before control leaves the try statement.

---

## 13. Expressions

```text
expression ::= or_expression
```

```text
or_expression ::= and_expression { "or" and_expression }
and_expression ::= not_expression { "and" not_expression }
not_expression ::= "not" not_expression | identity_expression
identity_expression ::= comparison { ("is" | "is" "not") comparison }
comparison ::= additive { comparison_operator additive }
comparison_operator ::= "==" | "!=" | "<" | "<=" | ">" | ">=" | "in"
additive ::= multiplicative { ("+" | "-") multiplicative }
multiplicative ::= unary { ("*" | "/" | "%") unary }
unary ::= ("+" | "-") unary | call_or_access
call_or_access ::= primary { call_suffix | attribute_suffix | index_suffix | slice_suffix }
call_suffix ::= "(" [ argument_list ] ")"
attribute_suffix ::= "." IDENT
index_suffix ::= "[" expression "]"
slice_suffix ::= "[" [ expression ] ".." [ expression ] "]"
primary ::= literal | IDENT | "(" expression ")"
```

```text
argument_list ::= argument { "," argument } [ "," ]
argument ::= expression | IDENT "=" expression
```

Positional arguments must precede named arguments.

Duplicate named arguments are errors.

### 13.1 Assignment Is Not an Expression

Invalid:

```python
let x = (y = 1)
```

### 13.2 Function Call

The callee must evaluate to a callable value.

Callable values:

```text
Function
BuiltinFunction
RecordConstructor
Method
```

### 13.3 Attribute Access

```python
p.x
```

Attribute access is defined for records and modules.

Accessing an undefined attribute is a runtime error.

### 13.4 Indexing

```python
xs[0]
m["key"]
```

Indexing is defined for lists and maps.

List indices must be integers in range `0 <= index < len(list)`.

Negative indices are invalid.

This avoids Python-style implicit reverse indexing.

---

## 14. Operator Precedence

From lowest to highest:

```text
or
and
not
is / is not
comparison: == != < <= > >= in
addition/subtraction: + -
multiplication/division/modulo: * / %
unary: + -
call / attribute / index
primary
```

Logical operators operate only on Bool.

Comparison chaining is allowed.

Example:

```python
a < b < c
```

Operands are evaluated left to right exactly once.

---

## 15. Evaluation Order

Evaluation order is deterministic and left-to-right.

Function call order:

1. evaluate callee
2. evaluate positional arguments left to right
3. evaluate named arguments left to right
4. bind parameters
5. execute function body

Attribute access order:

1. evaluate receiver
2. resolve attribute

Index access order:

1. evaluate indexed object
2. evaluate index expression
3. perform index lookup

`and` and `or` short-circuit.

---

## 16. Runtime Value Kinds

The language defines the following runtime value kinds:

```text
Nil
Bool
Int
Float
String
List
Map
Range
RecordType
RecordInstance
EnumType
EnumValue
ReadOnlyView
Function
BuiltinFunction
Module
Error
```

All values are first-class unless explicitly restricted.

### 16.1 Nil

`nil` is a singleton.

### 16.2 Bool

`Bool` has exactly two values:

```text
true
false
```

### 16.3 Int

`Int` is an exact signed integer at the language level.

### 16.4 Float

`Float` is a binary64 floating-point value.

### 16.5 String

`String` is an immutable sequence of Unicode scalar values.

String indexing is not defined.

String iteration is not defined.

Explicit string operations may be provided by builtins or libraries.

### 16.6 List

`List` is a mutable ordered sequence.

List equality is structural and order-sensitive.

### 16.7 Map

`Map` is a mutable insertion-ordered mapping from hashable keys to values.

Map equality is structural and order-insensitive.

Map iteration yields keys in insertion order.

### 16.8 Range

`Range` represents an arithmetic sequence of integers.

A range is iterable.

Range construction is provided by the builtin `range`.

### 16.9 RecordType

A record type is a nominal fixed-shape aggregate type.

### 16.10 RecordInstance

A record instance contains fields declared by its record type.

No undeclared field can be added.

### 16.11 EnumType

An enum type is a nominal closed sum type.

### 16.12 EnumValue

An enum value contains:

```text
enum type
case name
payload fields
```

Enum values are immutable.

### 16.13 ReadOnlyView

A read-only view is a non-mutating view over a mutable aggregate.

Read-only views prevent mutation through that view, but do not necessarily freeze the underlying value if other mutable references exist.

### 16.14 Function

A user-defined callable value.

### 16.15 BuiltinFunction

A host-provided callable value.

### 16.16 Module

A module value exposes exported names as attributes.

### 16.17 Error

An error value contains at least:

```text
code: String
message: String
```

Optional implementation-provided fields may include source span, cause, and stack trace.

---

## 17. Binding and Scope

### 17.1 Binding Kinds

Bindings are either mutable or immutable.

Created by:

```text
let       mutable binding
const     immutable binding
def       immutable binding
record    immutable binding
import    immutable binding
for var   immutable per-iteration binding
catch var immutable catch binding
```

### 17.2 Declaration Before Use

A name must be declared before it is referenced in the same block.

Forward references from function bodies are resolved at call time through lexical environments, but the referenced name must exist in an enclosing scope by the time it is accessed.

### 17.3 Duplicate Bindings

Declaring the same name twice in the same lexical scope is a static semantic error.

Nested scopes may shadow outer names.

### 17.4 Assignment Resolution

Plain assignment resolves the target name using lexical lookup.

If the resolved binding is mutable, it is updated.

If the resolved binding is immutable, assignment is a static semantic error when statically knowable, otherwise a runtime binding error.

If the name cannot be resolved, assignment is a name error.

### 17.5 Closure Capture

Functions capture referenced outer bindings lexically.

Captured immutable bindings can be read.

Captured mutable bindings can be read and, if the assignment target resolves to that binding, updated.

No `global` or `nonlocal` keyword exists.

### 17.6 Module Scope

Module scope is the outermost lexical scope of a source file.

Top-level declarations bind names in module scope.

Only exported module-scope names are visible to other modules.

### 17.7 Builtin Scope

Builtin names are resolved after lexical and module scopes.

A local declaration may shadow a builtin.

---

## 18. Function Semantics

### 18.1 Parameter Grammar

```text
parameter_list ::= parameter { "," parameter } [ "," ]
parameter ::= IDENT [ type_annotation ] [ "=" expression ]
```

Function definitions may declare a return type contract:

```text
function_definition ::=
    "def" IDENT "(" [ parameter_list ] ")" [ "->" type_expression ] ":" block
```

Examples:

```python
def add(a: Int, b: Int) -> Int:
    return a + b

def id(x):
    return x
```

If a parameter type annotation is present, the argument value must satisfy that type contract.

If a return type annotation is present, every returned value must satisfy that type contract.

Once a parameter has a default expression, all following parameters must also have defaults.

Invalid:

```python
def f(a = 1, b):
    return b
```

### 18.2 Call Arguments

Arguments may be positional or named.

Positional arguments must precede named arguments.

A parameter may receive at most one value.

### 18.3 Default Argument Evaluation

Default expressions are evaluated at call time when the corresponding argument is omitted.

Each omitted argument evaluates its default expression independently.

Default expressions are evaluated left to right in the function call environment after earlier parameters have been bound.

Example:

```python
def push(x, xs = []):
    xs.append(x)
    return xs
```

Each call without `xs` receives a fresh list.

### 18.4 Return

If a function body completes without `return`, the call result is `nil`.

### 18.5 Recursion

A function may recursively call itself if its binding is visible.

### 18.6 Function Identity

Each execution of a function definition creates a distinct function value.

---

## 19. Record Semantics

### 19.1 Record Definition

```python
record Point:
    field x
    field y
```

### 19.2 Field Declaration

```text
field_declaration ::= [ "mutable" ] "field" IDENT [ type_annotation ] [ "=" expression ] NEWLINE
```

A field is immutable unless declared `mutable`.

If a type annotation is present, every stored field value must satisfy that type contract.

Field names must be unique within a record.

Field default expressions are evaluated at construction time.

### 19.3 Constructor

A record type is callable as a constructor.

Constructor arguments must be named.

```python
let p = Point(x = 1, y = 2)
```

All fields without defaults must be provided.

Fields with defaults may be omitted.

Unknown fields are errors.

Duplicate field initializers are errors.

### 19.4 Field Access

```python
p.x
```

A field can be read if it exists.

### 19.5 Field Assignment

```python
p.x = 10
```

Field assignment is valid only for mutable fields.

Assigning to an immutable field is an error.

### 19.6 Methods

A method is a function declared inside a record body.

```python
record Counter:
    mutable field value

    def inc(self):
        self.value += 1
        return self.value
```

A method call:

```python
c.inc()
```

passes the receiver as the first argument.

Methods do not create dynamic fields.

Methods are resolved by record type.

There is no inheritance.

There is no method overriding.

There is no metaclass mechanism.

---

## 20. Container Semantics

### 20.1 List Indexing

```python
xs[0]
```

Valid indices are integers in range.

Negative indices are errors.

### 20.2 List Assignment

```python
xs[0] = value
```

List assignment requires a valid index.

### 20.3 List Methods

The core language defines these list methods:

```text
append(value) -> nil
pop() -> value
```

`pop` on an empty list raises `IndexError`.

### 20.4 Map Indexing

```python
m["key"]
```

Accessing a missing key raises `KeyError`.

### 20.5 Map Assignment

```python
m["key"] = value
```

The key must be hashable.

### 20.6 Membership

```python
x in xs
key in m
```

`in` returns Bool.

For lists, membership uses `==`.

For maps, membership checks key presence.

For strings, `in` is not defined in this core specification.

---

## 21. Numeric Semantics

### 21.1 Numeric Promotion

```text
Int op Int       -> Int, except /
Int op Float     -> Float
Float op Int     -> Float
Float op Float   -> Float
```

`/` returns Float.

### 21.2 Division

Division by zero raises `DivisionByZeroError`.

### 21.3 Modulo

Modulo by zero raises `DivisionByZeroError`.

Integer modulo uses floor-remainder semantics:

```text
a % b = a - floor(a / b) * b
```

### 21.4 Float NaN

`NaN == NaN` is false.

`NaN != NaN` is true.

NaN is not hashable.

---

## 22. Equality and Identity

### 22.1 Equality

`==` is value equality.

Rules:

```text
nil                 singleton equality
Bool                boolean value equality
Int                 mathematical integer equality
Float               binary64 equality
Int/Float           numeric equality after promotion
String              Unicode scalar sequence equality
List                structural order-sensitive equality
Map                 structural key/value equality
RecordInstance      same record type and field-wise equality
Function            identity equality
BuiltinFunction     identity equality
Module              identity equality
Error               identity equality unless specified otherwise
```

### 22.2 Identity

`is` tests identity.

For singleton and primitive immutable values, identity is defined as value identity.

For records, lists, maps, functions, modules, and errors, identity is object identity.

```python
x is nil
a is b
a is not b
```

---

## 23. Boolean Logic

### 23.1 Bool-Only Logical Operators

`and`, `or`, and `not` require Bool operands.

They return Bool.

Invalid:

```python
let x = 1 and 2
let y = not nil
```

### 23.2 Short-Circuit

`and` and `or` short-circuit.

```python
false and f()  # f is not called
true or f()    # f is not called
```

---

## 24. Modules

### 24.1 Module Identity

Each source file is a module.

A module has:

```text
module scope
module initialization state
export table
```

### 24.2 Exported Names

Only names declared with `export` are externally visible.

```python
export const pi = 3.14159

export def square(x):
    return x * x
```

### 24.3 Import Forms

```python
import math
import math.geometry as geom
from math import square
from math import square as sq
```

Imported bindings are immutable.

### 24.4 Module Initialization

Importing a module initializes it once.

Repeated imports return the same module value.

### 24.5 Circular Imports

Circular imports are permitted only for already-initialized exported bindings.

Accessing an exported binding before initialization raises `ImportCycleError`.

### 24.6 No Wildcard Import

Invalid:

```python
from math import *
```

### 24.7 No Implicit Relative Import

All module paths are resolved from explicit module roots.

Relative import syntax is not defined in this specification.

---

## 25. Error Semantics

### 25.1 Error Value

An error is a structured value.

Minimum fields:

```text
code: String
message: String
```

### 25.2 Creating Errors

The builtin constructor `error(code, message)` creates an Error value.

```python
raise error("InvalidState", "counter is closed")
```

### 25.3 Raising Errors

`raise` requires an Error value.

### 25.4 Catching Errors

```python
try:
    risky()
catch e if e.code == "InvalidState":
    print(e.message)
finally:
    cleanup()
```

The catch guard is optional.

The catch guard must evaluate to Bool.

A catch clause without a guard catches all runtime Error values.

Static errors are not catchable.

### 25.5 Finally

`finally` executes whether the try block:

1. completes normally
2. raises an error
3. returns
4. breaks
5. continues

If `finally` itself raises or returns, it replaces the pending control flow.

---


## 26. Optional Type Contracts

### 26.1 Nature of Type Contracts

Type annotations are optional runtime-verifiable contracts.

They are part of the high-level language.

They are not a separate static type system in this specification.

Type annotations are runtime contracts. They may be preserved by IR and may be used by diagnostics or optimization, but language correctness does not depend on whole-program static inference.

A conforming implementation must enforce type contracts at runtime where they are declared.

An implementation may additionally use type contracts for diagnostics, editor tooling, or optimization, but those uses are not required by this specification.

### 26.2 Type Expression Grammar

```text
type_expression ::= union_type

union_type ::= postfix_type { "|" postfix_type }

postfix_type ::= primary_type [ "?" ]

primary_type ::=
    type_name
  | list_type
  | map_type
  | function_type
  | "(" type_expression ")"

type_name ::= IDENT

list_type ::= "List" "[" type_expression "]"

map_type ::= "Map" "[" type_expression "," type_expression "]"

function_type ::= "Function" "[" [ type_list ] "->" type_expression "]"

type_list ::= type_expression { "," type_expression }
```

`T?` is shorthand for `T | Nil`.

Examples:

```python
let x: Int = 1
let name: String? = nil
let xs: List[Int] = [1, 2, 3]
let table: Map[String, Int] = {"a": 1}
```

### 26.3 Builtin Type Names

The following builtin type names are defined:

```text
Nil
Bool
Int
Float
String
List
Map
Range
Function
Module
Error
Any
Never
```

`Any` accepts any value.

`Never` accepts no value and is used for functions that do not return normally.

Record type names are introduced by `record` definitions.

### 26.4 Union Types

A union type `A | B` accepts values satisfying either `A` or `B`.

Example:

```python
let id: Int | String = 1
id = "x"
```

### 26.5 Optional Types

`T?` is equivalent to `T | Nil`.

Example:

```python
let maybe_name: String? = nil
```

### 26.6 List and Map Type Contracts

`List[T]` requires each element currently stored in the list to satisfy `T`.

Mutating a typed list must preserve the element contract.

`Map[K, V]` requires keys to satisfy `K` and values to satisfy `V`.

Map key types must also be hashable.

### 26.7 Function Type Contracts

`Function[[A, B] -> R]` describes a callable that accepts two arguments satisfying `A` and `B`, and returns a value satisfying `R`.

The exact callable identity and closure environment are not part of the type contract.

### 26.8 Contract Failure

A value that fails a declared type contract raises `TypeContractError`.

Examples:

```python
let x: Int = "no"          # TypeContractError
def f(x: Int) -> Int:
    return "bad"           # TypeContractError
```

### 26.9 Type Narrowing

Within a `match` case or an identity comparison against `nil`, a language implementation may narrow apparent types for diagnostics.

This specification does not require compile-time narrowing, but it defines no semantic obstacle to it.

---

## 27. Pattern Matching

### 27.1 Match Statement

```text
match_statement ::= "match" expression ":" NEWLINE INDENT { case_clause } DEDENT

case_clause ::= "case" pattern [ "if" expression ] ":" block
```

`match` evaluates its subject expression exactly once.

Cases are tested in source order.

The first matching case whose guard is absent or evaluates to `true` is selected.

If no case matches, the match statement has no effect.

A guard expression must evaluate to `Bool`.

### 27.2 Pattern Grammar

```text
pattern ::=
    wildcard_pattern
  | literal_pattern
  | binding_pattern
  | record_pattern
  | list_pattern
  | map_pattern
  | or_pattern
```

```text
wildcard_pattern ::= "_"

literal_pattern ::= "nil" | "true" | "false" | integer_literal | float_literal | string_literal

binding_pattern ::= IDENT

record_pattern ::= IDENT "(" [ field_pattern_list ] ")"

field_pattern_list ::= field_pattern { "," field_pattern } [ "," ]

field_pattern ::= IDENT "=" pattern

list_pattern ::= "[" [ pattern_list ] "]"

pattern_list ::= pattern { "," pattern } [ "," ]

map_pattern ::= "{" [ map_pattern_list ] "}"

map_pattern_list ::= map_pattern_entry { "," map_pattern_entry } [ "," ]

map_pattern_entry ::= literal_pattern ":" pattern

or_pattern ::= pattern "|" pattern
```

### 27.3 Wildcard Pattern

`_` matches any value and does not bind a name.

### 27.4 Literal Pattern

A literal pattern matches by `==`.

### 27.5 Binding Pattern

A binding pattern matches any value and introduces an immutable binding scoped to the case block.

Example:

```python
match value:
    case x:
        print(x)
```

### 27.6 Record Pattern

A record pattern matches a record instance of the named record type and then matches the listed fields.

Example:

```python
match p:
    case Point(x = 0, y = y):
        print(y)
```

Unmentioned fields are ignored.

Unknown fields in a record pattern are static semantic errors when the record type is known.

### 27.7 List Pattern

A list pattern matches a list with exactly the same length and whose elements match positionally.

Example:

```python
match xs:
    case [a, b]:
        print(a + b)
```

Rest patterns are not defined in this version.

### 27.8 Map Pattern

A map pattern matches if all listed keys exist and their values match.

Only literal keys are permitted in map patterns.

Example:

```python
match m:
    case {"status": "ok", "value": v}:
        print(v)
```

### 27.9 Or Pattern

`p1 | p2` matches if either pattern matches.

Both sides of an or-pattern must bind the same set of names.

### 27.10 Pattern Binding Conflicts

A pattern may not bind the same name twice.

Invalid:

```python
case [x, x]:
    print(x)
```

### 27.11 Match Coverage

This specification does not require exhaustiveness checking.

A future static analysis layer may warn for non-exhaustive matches over closed record or enum-like domains.

---


## 27.11 Structured Control-Flow Unwinding

The language defines a deterministic unwinding order for block exit, `defer`, `use`, and `finally`.

A control-flow exit is one of:

```text
normal completion
return
break
continue
raise
```

When a block scope exits:

1. deferred callables registered in that block execute in last-in-first-out order
2. resource cleanup owned by `use` bindings in that block executes
3. pending control flow is propagated to the enclosing construct

For a `try` statement:

1. control first leaves the active try or catch block
2. defers and use-cleanups belonging to that block execute
3. the `finally` block executes
4. defers and use-cleanups inside the finally block execute when that finally block exits
5. resulting control flow propagates outward

If a cleanup operation raises while another error is already pending, the original error remains primary and the cleanup error is attached as suppressed information when the implementation supports suppressed errors.

If a cleanup operation returns, breaks, or continues, that control flow replaces the previous pending control flow only where such replacement is valid in the surrounding syntactic context.

A `return`, `break`, or `continue` from a cleanup context that would target a scope no longer active is a runtime control-flow error.

## 28. Resource Management

### 28.1 Rationale

Scripts frequently interact with files, sockets, locks, database connections, and other resources.

The language defines structured resource management to avoid relying on finalizers or unpredictable garbage collection.

### 28.2 Use Statement

```text
use_statement ::= "use" IDENT "=" expression ":" block
```

The expression must evaluate to a resource value.

A resource value must provide a `close()` method.

The resource is bound as an immutable name inside the use block.

When control leaves the block, `close()` is invoked exactly once.

Control may leave the block by normal completion, return, break, continue, or raise.

Example:

```python
use f = open_file("data.txt"):
    print(f.read())
```

`open_file` is not a core builtin in this specification; the example illustrates resource semantics only.

### 28.3 Close Error

If the block completes normally and `close()` raises an error, that error becomes the result of the use statement.

If the block is already leaving with a pending error and `close()` raises another error, the close error is attached as suppressed error information to the original error if the implementation supports suppressed errors.

If suppressed errors are not supported, the original error takes precedence.

### 28.4 Defer Statement

```text
defer_statement ::= "defer" expression NEWLINE
```

The expression must evaluate to a zero-argument callable.

A deferred callable is executed when the current block scope exits.

Deferred callables execute in last-in-first-out order.

Example:

```python
let lock = acquire_lock()
defer lock.release

critical()
```

A `defer` statement inside a loop registers a deferred callable for the current iteration block.

### 28.5 Defer Restrictions

`defer` is invalid at module top level.

A deferred callable must not rely on names that have gone out of scope except through captured lexical bindings.

---

## 29. Capability-Oriented Module Effects

### 29.1 Capability Principle

Potentially dangerous host effects should be accessed through explicit capabilities.

The language core does not grant file, network, process, environment, or clock access implicitly.

Such effects are made available only through imported modules or host-provided capability values.

### 29.2 Capability Import Annotation

A module may declare required external capabilities.

```text
requires_clause ::= "requires" capability_list NEWLINE

capability_list ::= IDENT { "," IDENT } [ "," ]
```

Example:

```python
requires fs, net
```

A `requires` clause must appear before executable top-level statements.

### 29.3 Standard Capability Names

This specification reserves the following capability names:

```text
fs
net
process
env
clock
random
ffi
```

The language does not define how capabilities are granted. It only defines that effectful modules may declare requirements.

### 29.4 Effect Metadata

Functions may optionally declare effects:

```text
effect_clause ::= "effect" "[" [ effect_list ] "]"

effect_list ::= IDENT { "," IDENT }
```

Example:

```python
def load(path: String) -> String effect[fs]:
    return read_file(path)
```

Effect declarations are metadata contracts.

A conforming implementation must preserve them as language-level metadata.

Whether they are statically enforced is outside this document.

### 29.5 No Ambient Authority by Default

The core language does not define ambient filesystem, network, subprocess, or environment-variable access.

This is a security boundary at the language design level.

---

## 30. Protocolized Core Operations

### 30.1 Core Protocols

The language defines a small set of semantic protocols.

Protocols are not user-definable in this version.

They are language-recognized behavioral categories.

```text
Callable
Iterable
Indexable
Resource
Displayable
Hashable
Comparable
```

### 30.2 Callable

Values callable by `()` satisfy `Callable`.

Callable values include:

```text
Function
BuiltinFunction
RecordConstructor
Method
```

### 30.3 Iterable

Values usable in `for` satisfy `Iterable`.

Core iterable values:

```text
List
Map
Range
```

String is not iterable in the core language.

### 30.4 Indexable

Values usable with `[]` satisfy `Indexable`.

Core indexable values:

```text
List
Map
```

### 30.5 Resource

Values usable in `use` satisfy `Resource`.

A resource must expose a zero-argument `close` method.

### 30.6 Displayable

All core values are displayable through `print`.

Display is not the same as string conversion.

Implicit string conversion is not defined.

### 30.7 Hashable

Hashable values may be map keys.

Core hashable values:

```text
Nil
Bool
Int
String
```

### 30.8 Comparable

Comparable values may use ordering operators.

Core comparable pairs:

```text
Int/Int
Int/Float
Float/Int
Float/Float
String/String
```

Record and list ordering is not defined.

---

## 31. Concurrency Boundary

### 31.1 No Implicit Shared-Memory Concurrency

The core language does not define threads.

There is no implicit shared mutable state between concurrent executions.

### 31.2 Task Reserved Semantics

The keywords `async` and `await` remain reserved for future use.

This version does not define asynchronous functions or coroutines.

### 31.3 Deterministic Core

A conforming core program without host capabilities must be deterministic given the same source and inputs.

Nondeterminism may enter only through explicit capabilities such as `clock`, `random`, `net`, or host-provided external modules.

---


## 32. Destructuring Bindings

### 32.1 Binding Pattern Grammar

Declaration binding patterns are distinct from match patterns, although they share similar syntax.

```text
binding_pattern ::=
    IDENT
  | record_binding_pattern
  | list_binding_pattern
  | enum_binding_pattern
```

```text
record_binding_pattern ::= IDENT "(" [ field_binding_list ] ")"

field_binding_list ::= field_binding { "," field_binding } [ "," ]

field_binding ::= IDENT "=" binding_pattern

list_binding_pattern ::= "[" [ binding_pattern_list ] "]"

binding_pattern_list ::= binding_pattern { "," binding_pattern } [ "," ]

enum_binding_pattern ::= qualified_case_name "(" [ field_binding_list ] ")"

qualified_case_name ::= IDENT "." IDENT
```

### 32.2 Name Binding

Every identifier introduced by a binding pattern creates a binding according to the declaration kind.

Example:

```python
let [a, b] = [1, 2]
const Point(x = px, y = py) = p
```

### 32.3 Destructuring Failure

If the runtime value does not match the destructuring pattern, `PatternMatchError` is raised.

Example:

```python
let [a, b] = [1]  # PatternMatchError
```

### 32.4 Duplicate Names

A binding pattern may not bind the same name twice.

Invalid:

```python
let [x, x] = [1, 2]
```

### 32.5 Destructuring Assignment

Destructuring assignment to existing bindings is not defined in this specification.

Only declaration destructuring is defined.

Invalid:

```python
[a, b] = xs
```

---

## 33. Slices and Views

### 33.1 Slice Syntax

```text
slice_expression ::= receiver "[" [ start ] ".." [ end ] "]"
```

Examples:

```python
let a = xs[1..3]
let b = xs[..3]
let c = xs[2..]
let d = xs[..]
```

Start and end expressions must evaluate to `Int`.

Negative slice bounds are invalid.

### 33.2 List Slicing

List slicing returns a new list containing elements in the half-open range:

```text
[start, end)
```

Omitted start defaults to `0`.

Omitted end defaults to `len(list)`.

Out-of-range bounds are errors rather than silently clamped.

This avoids accidental partial reads caused by incorrect indices.

### 33.3 String Slicing

String slicing is defined over Unicode scalar positions, not bytes.

String slicing returns a new string.

Indexing a string with `[]` remains undefined; slicing is the only core positional string operation.

### 33.4 Read-Only Views

The builtin `readonly(value)` returns a read-only view of a list, map, or record instance.

Mutating through a read-only view raises `ReadOnlyError`.

Reading through a read-only view follows normal read semantics.

Read-only views are shallow.

Example:

```python
let xs = [1, 2, 3]
let ro = readonly(xs)
print(ro[0])
ro[0] = 10  # ReadOnlyError
```

---

## 34. String Formatting and Display

### 34.1 Format Strings

The language defines format strings using the prefix `f`.

```python
let name = "Ada"
print(f"hello {name}")
```

Format expressions are enclosed in `{` and `}`.

The expression inside braces follows normal expression grammar.

Format expressions are evaluated left to right.

### 34.2 No Implicit General String Coercion

Format strings are explicit display contexts.

They do not imply that `+` performs string conversion.

Invalid:

```python
let s = "count = " + 1
```

Valid:

```python
let s = f"count = {1}"
```

### 34.3 Escaping Braces

Literal braces in a format string are written as doubled braces:

```python
f"{{value}}"
```

### 34.4 Format Result

A format string evaluates to `String`.

Each embedded expression is converted using display representation, not debug representation.

### 34.5 Debug Display

The builtin `debug(value)` returns a diagnostic string representation.

Debug representation is stable enough for diagnostics but not guaranteed as a serialization format.

---

## 35. Documentation Comments and Metadata

### 35.1 Documentation Comments

A documentation comment begins with `##`.

A documentation comment applies to the next declaration if no blank line intervenes.

Example:

```python
## Returns the square of x.
export def square(x: Int) -> Int:
    return x * x
```

Documentation comments are part of source metadata.

They do not affect runtime semantics.

### 35.2 Declaration Metadata

The language reserves metadata blocks for future tooling.

```text
doc metadata is preserved
effect metadata is preserved
type contract metadata is preserved
```

This specification does not define arbitrary user attributes or decorators.

Decorators remain unsupported.

---

## 36. Test Blocks and Assertions

### 36.1 Test Block

```text
test_block ::= "test" string_literal ":" block
```

A test block is a named executable specification block.

Example:

```python
test "addition":
    assert 1 + 1 == 2
```

Test blocks do not execute during ordinary module execution unless the host explicitly runs tests.

This document defines test block syntax and semantics as source-level language structure, but does not define the external test runner.

### 36.2 Test Scope

A test block introduces a block scope.

It can access exported and non-exported declarations in the same module.

### 36.3 Assertion

`assert` is valid in ordinary code and in test blocks.

Assertion failure raises `AssertionError`.

---

## 37. Public API Sealing

### 37.1 Export Table

A module's export table is fixed after module initialization completes.

New exports cannot be created dynamically.

### 37.2 Re-Export

An imported binding may be re-exported explicitly.

```python
from math import square
export const sq = square
```

### 37.3 Export Alias

Direct export aliases use normal declarations.

There is no separate alias-only export syntax.

### 37.4 No Export Mutation

The export table is not a mutable map exposed to user code.

---

## 38. Standard Library Boundary

### 38.1 Core vs Standard Library

This specification defines the core language.

The standard library is separate.

Only the following builtin names are part of the core language:

```text
print
len
range
error
readonly
debug
```

All other utilities belong to modules.

### 38.2 Standard Module Names Reserved

The following standard module roots are reserved:

```text
core
text
math
collections
time
fs
net
process
env
random
testing
```

This specification reserves names only. It does not define their full APIs.

The reserved standard module roots do not become part of the frozen Phase 1 core semantics beyond their names, purity classification, and capability classification.

### 38.3 Capability-Gated Standard Modules

The following module roots require capabilities:

```text
fs       requires fs
net      requires net
process  requires process
env      requires env
time     requires clock
random   requires random
```

### 38.4 Pure Standard Modules

The following module roots are pure by default:

```text
core
text
math
collections
testing
```

---

## 39. Serialization Boundary

### 39.1 No Implicit Serialization

The language does not define implicit conversion from values to source, JSON, binary, or bytecode.

### 39.2 Serializable Core Values

The following value kinds are structurally serializable by future library APIs:

```text
Nil
Bool
Int
Float, except NaN and infinity unless explicitly allowed
String
List of serializable values
Map with String keys and serializable values
RecordInstance with serializable fields
EnumValue with serializable payloads
```

### 39.3 Non-Serializable Core Values

The following are not structurally serializable by default:

```text
Function
BuiltinFunction
Module
Error
ReadOnlyView
Resource
```

### 39.4 Serialization Is Library-Level

Serialization belongs to standard library modules, not implicit language semantics.

---


## 40. Foreign Boundary

### 40.0 Foreign Boundary Commitment

The foreign boundary is not part of the core execution model.

The language deliberately rejects CPython C-extension compatibility and Python binary wheel compatibility.

Foreign code may enter the ecosystem only through mechanisms specified outside the core language, such as:

```text
capability-gated FFI
host-provided modules
WASM modules
future language-native module ABI
process/RPC interoperation
data-format interoperation
```

Foreign mechanisms must not require exposure of internal VM object layout.

Foreign mechanisms must not rely on CPython object layout, reference counting, GIL behavior, or Python extension-module ABI.

Foreign mechanisms must not bypass capability restrictions by default.


### 40.1 FFI Is Not Core

Foreign function interfaces are not part of the core language.

The capability name `ffi` is reserved.

### 40.2 Host Values

A host may expose opaque host values.

Opaque host values are not inspectable by core language semantics unless wrapped by defined records, modules, or builtin functions.

### 40.3 Host Value Restrictions

Opaque host values are:

```text
not hashable by default
not serializable by default
not comparable by ordering
identity-comparable only if the host exposes stable identity
```

---

## 41. Builtins

The core language defines these builtin names:

```text
print
len
range
error
readonly
debug
```

### 41.1 print

```text
print(value) -> nil
```

Writes display representation followed by newline.

### 41.2 len

```text
len(value) -> Int
```

Defined for:

```text
String
List
Map
Range
```

### 41.3 range

```text
range(stop) -> Range
range(start, stop) -> Range
range(start, stop, step) -> Range
```

All arguments must be Int.

`step` must not be zero.

### 41.4 error

```text
error(code, message) -> Error
```

Both arguments must be String.

### 41.5 readonly

```text
readonly(value) -> ReadOnlyView
```

Defined for:

```text
List
Map
RecordInstance
```

### 41.6 debug

```text
debug(value) -> String
```

Returns diagnostic representation.

`debug` is not serialization.

---

## 42. Diagnostics

### 42.1 LexicalError

Invalid UTF-8, invalid token, invalid escape, malformed literal.

### 42.2 IndentationError

Invalid indentation, unexpected indentation, missing indentation, mismatched dedent.

### 42.3 SyntaxError

Token sequence does not match grammar.

### 42.4 StaticSemanticError

Includes:

1. duplicate declarations in one scope
2. duplicate parameters
3. reference to name before declaration in same block
4. assignment to immutable binding
5. `return` outside function
6. `break` outside loop
7. `continue` outside loop
8. invalid export target
9. invalid record member
10. invalid type expression
11. invalid pattern binding
12. invalid capability declaration
13. invalid effect declaration

### 42.5 NameError

Unresolved name.

### 42.6 TypeError

Operation receives unsupported value kind.

### 42.6.1 TypeContractError

A value fails a declared runtime type contract.

### 42.6.2 PatternError

A pattern is malformed or attempts an invalid runtime match operation.

### 42.6.3 PatternMatchError

A declaration destructuring pattern fails to match the runtime value.

### 42.6.4 ReadOnlyError

A mutation is attempted through a read-only view.

### 42.6.5 AssertionError

An `assert` statement evaluates to false.

### 42.7 ArityError

Callable receives wrong number of arguments.

### 42.8 IndexError

List index out of range or invalid list index.

### 42.9 KeyError

Map key not found.

### 42.10 FieldError

Record field does not exist or immutable field assignment attempted.

### 42.11 ImportError

Module or exported name cannot be found.

### 42.12 ImportCycleError

Circular import accesses an uninitialized export.

### 42.13 DivisionByZeroError

Division or modulo by zero.

### 42.14 NumericOverflowError

Integer operation exceeds implementation numeric capacity where exact integers are not supported.

### 42.15 Diagnostic Location

Diagnostics should include:

```text
file
start line
start column
end line
end column
```

Line and column numbers are one-based.

---

## 43. Unsupported Constructs

The following are not part of this specification:

```text
CPython C API
CPython ABI
Python binary wheels
Python extension modules
CPython object layout compatibility
direct native pointer exposure
implicit dynamic library loading
public bytecode
class inheritance
metaclasses
operator overloading
descriptors
decorators
generators
async / await
macros
reflection protocol
eval
exec
wildcard import
implicit relative import
global
nonlocal
destructuring assignment
multiple assignment
comprehensions
negative indexing
string iteration
user-defined protocols
monkey patching
```

---

## 44. Examples

### 44.1 Declarations

```python
const pi = 3.14159
let x = 1
x = x + 1
```

### 44.2 Boolean Conditions

```python
let x = 10

if x > 0:
    print("positive")
else:
    print("non-positive")
```

Invalid:

```python
if x:
    print(x)
```

### 44.3 Functions With Safe Defaults

```python
def append_one(xs = []):
    xs.append(1)
    return xs

print(len(append_one()))
print(len(append_one()))
```

The two calls do not share the same default list.

### 44.4 Records

```python
record Point:
    field x
    field y

let p = Point(x = 1, y = 2)
print(p.x)
```

### 44.5 Mutable Record Field

```python
record Counter:
    mutable field value

    def inc(self):
        self.value += 1
        return self.value

let c = Counter(value = 0)
print(c.inc())
print(c.inc())
```

### 44.6 Lists

```python
let xs = [1, 2, 3]
xs.append(4)
print(len(xs))
print(xs[0])
```

### 44.7 Maps

```python
let m = {"a": 1}
m["b"] = 2

if "a" in m:
    print(m["a"])
```

### 44.8 For Loop

```python
for x in range(5):
    print(x)
```

### 44.9 Modules

```python
# mathx.sf
export def square(x):
    return x * x
```

```python
from mathx import square

print(square(5))
```

### 44.10 Structured Errors

```python
def fail():
    raise error("DemoError", "failure")

try:
    fail()
catch e if e.code == "DemoError":
    print(e.message)
finally:
    print("done")
```

---


### 44.11 Type Contracts

```python
let xs: List[Int] = [1, 2, 3]

def sum(xs: List[Int]) -> Int:
    let total: Int = 0
    for x in xs:
        total += x
    return total

print(sum(xs))
```

### 44.12 Pattern Matching

```python
record Point:
    field x: Int
    field y: Int

let p = Point(x = 0, y = 5)

match p:
    case Point(x = 0, y = y):
        print(y)
    case _:
        print("other")
```

### 44.13 Resource Management

```python
use file = open_file("data.txt"):
    print(file.read())
```

`open_file` is not a core builtin. The example demonstrates `use` block semantics.

### 44.14 Capabilities

```python
requires fs

def load(path: String) -> String effect[fs]:
    return read_file(path)
```

The example demonstrates declared effect metadata. Host capability granting is outside this specification.



### 44.15 Enum

```python
enum Status:
    case Pending
    case Done(value: String)
    case Failed(error: Error)

let s = Status.Done(value = "ok")

match s:
    case Status.Done(value = text):
        print(text)
    case Status.Failed(error = e):
        raise e
    case _:
        print("pending")
```

### 44.16 Destructuring Declaration

```python
record Point:
    field x: Int
    field y: Int

let p = Point(x = 1, y = 2)
let Point(x = px, y = py) = p

print(px + py)
```

### 44.17 Slicing

```python
let xs = [1, 2, 3, 4]
let mid = xs[1..3]

print(len(mid))
```

### 44.18 Format String

```python
let name = "Ada"
let age = 36

print(f"{name}: {age}")
```

### 44.19 Test Block

```python
test "range length":
    let xs = [0, 1, 2]
    assert len(xs) == 3, "unexpected length"
```

### 44.20 Read-Only View

```python
let xs = [1, 2, 3]
let ro = readonly(xs)

print(ro[0])
```


## 45. Conformance Boundary

A source program conforms if:

1. it is valid UTF-8
2. it obeys lexical rules
3. it obeys indentation rules
4. it matches the grammar
5. it satisfies static semantic rules
6. it uses only defined constructs
7. its runtime behavior follows this specification

An implementation conforms if:

1. it accepts conforming programs
2. it rejects invalid programs with diagnostics
3. it evaluates conforming programs according to this specification
4. it does not require a user-visible compilation step
5. it does not expose public bytecode as the required execution artifact

---

## 46. Language Boundary Summary

The language is defined by these commitments:

```text
source-first
indentation-sensitive
explicit declaration
block scoped
dynamic values
strict boolean conditions
no implicit coercion
first-class functions
safe call-time defaults
lexical closures
fixed-shape records
mutable and immutable bindings
mutable containers
explicit exports
no wildcard import
structured errors
optional runtime type contracts
pattern matching
closed nominal enums
safe declaration destructuring
half-open slicing
explicit format strings
documentation comments
test blocks and assertions
read-only views
structured resource management
capability-oriented effect metadata
protocolized core operations
deterministic core without ambient authority
no public bytecode
not Python-compatible
not CPython C-extension-compatible
frozen Phase 1 baseline
```
