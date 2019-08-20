# Native functions
Natives act like any other function, but are not implemented in Mankai.

## List of natives

### `+`

`(+ n1 n2 ... nm)`

Where `n1`, `n2`, ..., `nm` are all numbers return the sum of all the arguments i.e. `n1 + n2 + ... + nm`. A runtime error is reported if any of the given arguments do not evaluate to a number.
n
#### Using `+` as unary operator
When `+` is used as a unary operator it just returns its argument i.e. `(+ n)` is equivalent to `n` with the exception that if `n` does not evaluate to a number then a runtime error is reported.

#### Examples

```
(+ 1 1)
=> 2
(+ 2 2 2)
=> 6
(+ 10)
=> 10
```

### `-`

`(- n1 n2 ... nm)`

Where `n1`, `n2`, ..., `nm` are all numbers returns the result of `n1 - n2 - ... - nm`. A runtime error is reported if any of the given arguments do not evaluate to a number.

#### Using `-` as unary operator
When `-` is used as a unary operator it returns the inverse its argument.

#### Examples

```
(- 3 2)
=> 1
(- 3 5)
=> -2
(- 6)
=> -6
```

### `*`

`(* n1 n2 ... nm)`

Where `n1`, `n2`, ..., `nm` are all numbers returns the multiplication of all arguments together i.e. `n1 * n2 * ... * nm`. A runtime error is reported if any of the given arguments do not evaluate to a number.

#### Using `*` as unary operator
When `*` is used as a unary operator it just returns its argument i.e. `(* n)` is equivalent to `n` with the exception that if `n` does not evaluate to a number then a runtime error is reported.

#### Examples

```
(* 2 2)
=> 4
(* 3 2 2)
=> 12
(* 22)
=> 22
```

### `/`

`(/ n1 n2 ... nm)`

Where `n1`, `n2`, ..., `nm` are all numbers return the result of `n1 / n2 / ... / nm`. A runtime error is reported if any of the given arguments do not evaluate to a number.

#### Using `/` as a unary operator
When `/` is used as a unary operator it returns the multiplicative inverse of the given argument i.e. `(/ n)` evaluates to `1 / n` (if `n` evaluates to a number, otherwise a runtime error is reported).

#### Examples

```
(/ 1 2)
=> 0.5
(/ 1 2 2)
=> 0.25
(/ 2)
=> 0.5
```

### `car`

`(car l)`

Return the first element of `l`. If `l` is not a list (or it's the empty list) a runtime error is reported.

#### Examples

```
(car (list 1 2 3))
=> 1
(car (list (list) 2))
=> ()
```

### `cdr`

`(cdr l)`

Return the tail of `l`. If `l` is not a list (or it's the empty list) a runtime error is reported.

#### Examples

```
(cdr (list 1 2 3))
=> (2 3)
(cdr (list (list 1 2) (list 3 3)))
=> ((3 3))
```

### `cons`

`(cons l a b ... z)`

Append `a`, `b`, ..., `z` to `l`. If `l` is not a list a runtime error is reported.

#### Examples

```
(cons (list 1 2) 3)
=> (1 2 3)
(cons (list) 1 2 3)
=> (1 2 3)
```

### `list`

`(list arg1 arg2 ... argn)`

Return the list `(arg1 arg2 ... argn)`.

#### Using `list` with no arguments
`list` can be used with no arguments and will produce the empty list `()`.

#### Examples

```
(list 1 2 3)
=> (1 2 3)
(list "foo" "bar")
=> ("foo" "bar")
```

### `string-concat`

`(string-concat s1 s2 ... sn)`

Where `s1`, `s1`, ..., `sm` are all strings return the concatenation of those strings. A runtime error is reported if any of the given arguments do not evaluate to a string.

#### Using `string-concat` as a unary operator
When `string-concat` is used as a unary operator the given argument is returned unchanged (a runtime error is still reported if its not a string).

#### Examples

```
(string-concat "foo" " " "bar" " " "baz")
=> "foo bar baz"
```

### `to-string`

`(to-string x)`

Convert `x` to a string. Works on all Mankai objects.

#### Examples

```
(to-string 3)
=> "3"
(to-string "foo")
=> "foo"
(to-string +)
=> "<native function>"
```
