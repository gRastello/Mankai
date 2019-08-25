# Special forms
A _special form_ in Mankai is a form to which special evaluation rules apply. _Special forms_ always end in `!`.

## List of special forms

### `defun!`

`(defun! fname (arg1 arg2 ... argN) body)`

Define a function named `fname` that takes arguments `arg`, `arg2`, ..., `argN` and returns the result of evaluating `body`.

#### `defun!` vs `define!` and `lambda!`
You can achieve a similar result by binding the result of a `lambda!` expression with `define!`. However using `defun!` is preferred since it will give better error messages:

```
(define! f (lambda! (n) (+ n 1)))
=> <user-defined function>
(f)
Runtime error: found 0 arguments but 'anonymous function' requires 1!
(defun! f (n) (+ n 1))
=> <user-defined function>
(f)
Runtime error: found 0 arguments but 'f' requires 1!
```

#### Examples

```
(defun! my-addition (a b) (+ a b))
=> <user-defined function>
(my-addition 1 2)
=> 3
```

### `if!`

`(if! condition then else)`

If `condition` evaluates to `true` then `then` is evaluated. If `condition` evaluates to `false` then `else` is evaluated. If `condition` does not evaluate to a boolean a runtime error is reaised. Note that the `else` expression is _always required_.

#### Examples

```
(if! true 1 2)
=> 1
(if! false 1 2)
=> 2
(if! (= 2 2.5) 1 2)
=> 2
```

### `lambda!`

`(lambda! (arg1 arg2 ... argN) body)`

Creates an anonymous function that takes arguments `arg1`, `arg2`, ... `argN` and returns the result of evaluating `body`.

#### Examples

```
((lambda! (x y) (+ x y)) 1 2)
=> 3
((lambda! (f x) (f (f x))) (lambda! (n) (+ n 1)) 1)
=> 3
```

### `define!`

`(define! name value)`

Binds `name` to the result of evaluating `value`.

#### Examples

You can use `define!` to create variables to hold any kind of value:

```
(define! foo 2)
=> 2
(+ 1 foo)
=> 3
(define! bar (+ 1 2))
=> 3
bar
=> 3
```

You can even use `define!` to create variables to hold functions or special forms:

```
(define! my-multiplication *)
=> <native function>
(my-multiplication 2 2 3)
=> 12
(define! my-define define!)
=> <special form>
(my-define foo 3)
=> 3
foo
=> 3
```
