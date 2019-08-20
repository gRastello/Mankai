# Special forms
A _special form_ in Mankai is a form to which special evaluation rules apply. _Special forms_ always end in `!`.

## List of special forms

### `set!`

`(set! name value)`

Binds `name` to the result of evaluating `value`.

#### Examples

You can use `set!` to create variables to hold any kind of value:

```
(set! foo 2)
=> 2
(+ 1 foo)
=> 3
(set! bar (+ 1 2))
=> 3
bar
=> 3
```

You can even use `set!` to create variables to hold functions or special forms:

```
(set! my-multiplication *)
=> <native function>
(my-multiplication 2 2 3)
=> 12
(set! my-set set!)
=> <special form>
(my-set foo 3)
=> 3
foo
=> 3
```
