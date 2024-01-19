
# Scope



## Features of Python to be Supported
* Base types:
  * int
  * float
  * str
  * list
  * tuple
  * set
  * dict
  * function
  * objects
* Control Flow:
  * If
  * For
  * While
  * With
  * Higher Order Functions



## Grammar
```
file: [statements] ENDMARKER
func_type: '(' [type_expressions] ')' '->' expression NEWLINE* ENDMARKER


statements: statement+
statement: simple_stmts | compound_stmt

statement_newline:
    | compound_stmt NEWLINE
    | simple_stmts
    | NEWLINE
    | ENDMARKER

simple_stmts:
    | simple_stmt !';' NEWLINE
    | ';'.simple_stmt+ [';'] NEWLINE

simple_stmt:
    | assignment
    | type_alias
    | star_expressions
    | return_stmt
    | import_stmt
    | 'pass'
    | 'break'
    | 'continue'
    | global_stmt
    | nonlocal_stmt

compound_stmt:
    | function_def
    | if_stmt
    | class_def
    | with_stmt
    | for_stmt
    | while_stmt
    | match_stmt
   
assignment:
    | NAME ':' expression ['=' annotated_rhs]
    | '(' single_target ')' ':' expression ['=' annotated_rhs]
    | single_target augassing ~ star_expressions

```