# The Mankai programming language
Mankai is a toy programming language designed to be run on Telegram bots. It's dynamically typed and has a lisp-like syntax.

## Types
Mankai objects can currently be of four kinds: numbers, strings, special forms or native functions. Mankai numbers are always 64-bit floats.

## Examples
If you fire up the REPL you can start typing in some expressions:

```
1
=> 1
"foo"
=> "foo"
```
As you can see numbers and strings evaluate to themselves (as expected). Here's a more involved (but still trivial) session featuring calls to native functions and special forms:

```
(+ 1 1)
=> 2
(set! two (+ 1 1))
=> 2
two
=> 2
(set! three (+ 1 (* 1 2)))
=> 3
three
=> 3
(set! one (- three two))
=> 1
(set! five (+ three one one))
=> 5
(+ five one)
=> 6
```

You can read about all special forms and native functions by following the following links:
- [special forms](special_forms.md)
- [native functions](native_functions.md)
