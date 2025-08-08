# Language Design

## About

This is the design for an experimental programming language. As of this writing,
only this design exists. I'm just dreaming here. There's no implementation, and
there might never be one.

Not only that, but the design itself is pretty raw. It needs a few more rounds
of review and refinement, and I might only do that when inspiration strikes.
There's no schedule here.

This design is written in the form of documentation, because I found that to be
an effective tool to channel my thoughts. **Don't mistake this for actual
documentation of an actual language.**

## Objectives

I have the following goals in working on this language:

1. Create a minimal language that is usable as a portable assembler.
2. Explore the potential of stack-based programming for that purpose.
3. Do 2. without reinventing Forth. I have some of my own ideas.
4. Explore the unification between runtime and compile-time code.
5. Learn more about WebAssembly by using it as a compilation target.

## Design

### Basic Operations

The language is stack-based, as that makes for a very concise syntax that is
easy to work with.

Well, easy to work with for me, who's going to be stuck with implementing this.
You might see it differently. And I might join you in that view, later on, when
I'll become a user of the language.

But let's move on. Here's a basic example.

```
1 2 +
```

We have three identifiers here, all of which refer to functions. And all of
those are intrinsic functions, that are built into the compiler.

`1` and `2` simply return these respective values and put them on the stack. All
numbers are 32-bit integers, which depending on context are treated as signed
(two's complement) or unsigned.

`+` then takes those two values from the stack, and puts their sum back on.

Once this is finished, the stack contains the number `3`. Plus whatever was on
there in the first place.

### Bindings

I have a hard time dealing with just a stack. It's a skill issue, I know. But
this is my language. And I'd rather design and implement bindings than spend a
decade or five achieving enlightenment.

```
1 2 => a b .
```

This is the binding operator (`=>`). After it, you put a whitespace-delimited
list of identifiers. You close that list with a period (`.`).

The binding operator consumes as many values from the stack, as there are
identifiers in the list. It _binds_ names to those values, creating _bindings_.
Those bindings may be accessed later by _calling_ those names.

Here's an example of doing just that.

```
1 2 => a b .
a b +
# The result is `3`.
```

### Stack Manipulation

There's a nice side effect to having the binding operator: It saves us from
having to learn all those stack manipulation operators, that other stack-based
languages tend to have.

We can just implement them using bindings.

```
# dup
1 => a . a a

# swap
1 2 => a b . b a

# over
1 2 => a b . a b a
```

### Formatting

All that the compiler cares about, is that tokens are separated by whitespace.
It doesn't care how much whitespace that is, or what kind.

Here's a binding example from above:

```
1 2 => a b . b a
```

It could also be written like this:

```
1 2 => a b .
b a
```

Or like this:

```
1
2
=>
a
b
.
b
a
```

Or literally any other way. As long as the order of the tokens is the same, and
there's whitespace between them, the resulting code is equivalent.

### Blocks

If we put `{` and `}` around some code, we group that code into a block.

```
{ 2 + }
```

A block is a value, just like a number. In fact, it is represented as a number
on the stack (the address of the block's code). There is no type system to
distinguish between blocks and "real" numbers, so be careful when using them!

The result of the code above, is to put a block that contains this code on the
stack.

We can apply that block, to execute the code it contains.

```
1 { 2 + } apply
# The result is `3`.
```

Or we can bind a name to the block, and apply it via that name.

```
{ 2 + } => add_two .
1 add_two
# The result is `3`.
```

By calling a block by name, we apply it automatically, rather than placing the
block on the stack. This way, we define _functions_.

### Function Parameters

So far, we've only defined functions with implicit parameters. `add_two`
consumes an argument, a number, but it doesn't specify that explicitly.

It does so _implicitly_, by putting a number on the stack, but then calling a
function that takes 2 numbers from the stack. As a result, the combined function
consumes one number and returns another one.

And that's often fine. But sometimes we want named parameters. And we can
already do that!

```
{ => n
  n 2 +
}
  fun => add_two
```

This is the first example where we do something new, but don't require a new
feature to do it. We just combine things that we already have; blocks,
functions, and bindings; and we get a function with named parameters.

Strictly speaking, the parameter is still implicit. Only instead of the `+`
consuming more than the function itself provides, we have a binding that
consumes something that the caller needs to supply.

And by putting such a binding first in a block, we can make the requirements of
that block very explicit.

This demonstrates a very important design principle: A minimal set of orthogonal
features, that can be combined in interesting ways to build up more interesting
things.

### Higher-Order Functions

By wrapping a function call into a block, we can use that function as the
argument of a higher-order function.

```
1 2 3 { + } do_twice
# The result is `6`.
```

Here we pass a block to a function that, presumably, applies that block twice.

### Environments

Every block has an environment, which is the (possibly empty) set of bindings
that it calls, which are not defined inside of it.

```
{ one two + } => three .
```

This block calls three bindings, `one`, `two`, and `+`. None of those are
defined inside of it, so all of them are part of its environment.

`+` is an intrinsic function, which is always available. But `one` and `two` are
not defined anywhere. The compiler translates them as calls to a sentinel
function, which fails at runtime.

However, if such undefined functions are defined later, the compiler makes sure
that the right function gets called.

```
# `one` and `two` not available here yet; compiled as calls to the sentinel
# function.
{ one two + } => three .

# Missing functions defined later; all calls to the sentinel function are being
# replaced.
{ 1 } => one .
{ 2 } => two .
```

These rules can lead to confusing situations, if abused.

```
# `one` and `two` are not available yet.
{ one two + } => three .

# We put a block calling `three` on the stack here. But nothing is called yet,
# so all is good.
{ three }

# Now we provide the missing functions.
{ 1 } => one .
{ 2 } => two .

# And finally we call `three`, indirectly, by applying the block on the stack
# that calls it. All missing functions have been provided by now, so all is
# well, even though all _wasn't_ well when the block was put on the stack.
apply
```

You should avoid situations such as these. If you define a block that calls
functions that have not been defined yet, don't apply that block in its parent
scope.

If you apply a block in its parent scope (for example calling a higher-order
function), maybe define it right before that, to avoid any confusion.

### Compilation Model

There is no distinction between code that is executed at runtime (i.e. regular
old code) and code that only has effects at compile-time (defining functions,
types, etc.).

There is only code, and it all follows the same rules.

This means that the compiler is also an interpreter. It directly executes the
top-level code at compile-time. All examples we've seen so far can be fed
directly into it.

The compiler treats this top-level code as an implicit block. It translates any
local bindings of that block into exports of the resulting WebAssembly module.

### To Be Continued...

This is as far as I made it. Much more left to do, later. For example:

- Error Handling
- Memory Management

## Roadmap

Once the design is a bit more solid, the goal of this section is to extract the
first few minimal steps, that I can start with.

## Extensions

This section details some possible extensions to the language design, which I
wanted to leave out of the initial scope.

### Number Literals

Once there are multiple number types, but before the compile has become advanced
enough to reach sentience, we can create values of those numbers by specifying
the type explicitly.

```
1:u32 # 32-bit, unsigned
2:s8  # 8-bit, signed
```

### Functions

Automatically applying a block that is bound to a name should work well, for the
most part. If we still wanted to put a block on the stack, instead of applying
it, we could do so by wrapping it in another block.

```
{ 2 + } => add_two .
{ add_two }
```

At runtime, the resulting block that is put on the stack here is equivalent to
the original `add_two` block. The only way to possibly observe the additional
layer of block is through a slight difference in performance, and even that
could be optimized.

However, once we get into metaprogramming and start reading the contents of
blocks with compile-time functions, the difference _does_ become observable.

The rest of this section presents a different take on blocks and functions. The
reason I don't want to go for that initially, is that it requires a static type
system to work.

---

We can use a block to build a function.

```
{ 2 + } fun => add_two .
```

This is almost what we did above, except that we call the intrinsic function
`fun` after defining the block. `fun` consumes a block and returns a function
that wraps that block.

Functions are mostly like blocks, except that _evaluating_ them doesn't put a
function value on the stack. It _applies_ the function. And _evaluating_ a value
is what we always do, when we call it by name.

```
{ 2 + } fun => add_two .
1 add_two
# The result is `3`.
```

That's the same as above, except we no longer need that `apply`.

### Arrays

If we put some values between `[` and `]`, that groups them into a single array
value, and puts that on the stack.

```
[ 1 2 3 ]
```

And with that, we can write the previous example more elegantly, and more
generally.

```
[ 1 2 3 ] { + } fold
```

In principle, we could allow arbitrary code between `[` and `]`. As long as that
results in zero or more values of the same type, it would make for a valid
array.

```
1 [ 2 + ]
# The result is `[ 3 ]`.

[ { 1 } 3 times ]
# The result is `[ 1 1 1 ]`.
```

But there's a catch: This is a compiled language, and one with explicit memory
allocation at that. No auto-boxing shenanigans! Hence these arrays live on the
stack, and we must know their size at compile-time.

A sufficiently smart compiler could determine that for all of the examples
above. But since we're going to start with a pretty dumb one, let's take the
easy way out: Anything but a series of literals is undefined behavior.

(There is another option: dynamically sized stack allocations. My language
designer instincts tell me that this is going to lead to complexity, and
restrictions elsewhere, most likely. Let's avoid it.)

To get values out of an array later, we can use the `get` function.

```
[ 1 2 3 ] 0 get
# The result is `1`.
```

Or we can update an array after we created it, using `set`.

```
[ 1 2 3 ] 0 4 set
# The result is `[ 4 2 3 ]`.
```

`get`ing or `set`ing with an index that is out of bounds for the given array, is
undefined behavior.

### Records

Arrays are pretty good, as far as aggregate data types go. But we also need
composite data types to have some real fun. Hence, records.

```
{
  1 => a .
  2 => b .
}
  rec
```

Here we have a block, in which we create some bindings. Then we pass that to
another intrinsic function, `rec`. It applies the block, just like `apply`. But
it doesn't put the block's result on the stack.

Instead, it takes all the bindings in the block, uses those as the fields of a
new record, then puts that record on the stack.

What happens to the block's result? Well, in this case it doesn't make a
difference, because the block returns nothing. If it did, that would be
undefined behavior.

We can use variants of the `get` and `set` functions that we used for arrays, to
access the fields of records. We just need a new type of literal for that, the
symbol.

```
{ 1 => x . 2 => y . } @x get
# The result is `1`.

{ 1 => x . 2 => y . } @x 3 set
# The result is `{ 3 => x . 2 => y . }`.
```

A symbol starts with `@`. What follows must be a valid identifier. If you use
`get` or `set` with a symbol that doesn't match a field of the record, that's
undefined behavior.

### Modules

I said above that the top-level context is implicitly a block. But it's not just
any block, and also not a unique one that the compiler needs to treat in a
special way.

It's another example of a block wrapped in something else. Like a function.
Except, this one is more like a function that runs at compile-time. It's a
module.

And as with functions, we can use a special intrinsic function to define our
own.

```
{
  { 2 + } fn => add_two .
  { 3 + } fn => add_three .
}
  mod => ConstAdd .
```

Here's a module that defines two functions. When the compiler encounters this
module, it treats it like the top-level module: It evaluates it, and translates
its bindings into code.

Except, since this is not the top-level module, its bindings don't become
exported symbols of the WebAssembly module. They are made available to other
code instead.

### Destructuring

The introduction goes with rather simple means for working with arrays and
records. A different take on that would be destructuring.

```
# Define an array for later use.
[ 1 2 3 ] => array .

# Access all items of the array.
array => [ a b c ]

# Access only the second item, drop the others.
array => [ _ b _ ]

# Access only the first item, drop the others.
array => [ a ... ]
```

The same would work for records.

```
# Define a record for later use.
{ 1 => x . 2 => y . 3 => z . } rec => vec .

# Access all fields of the record.
vec => { x y z } .

# Access only two, drop the third.
vec => { x y _ } .

# Access only one, drop all others.
vec => { x ... } .
```

And the various forms would be combined.

```
1 array 2 vec => a { x ... } b [ c ... ] .
```

But the binding needs to always define exactly how many values from the stack it
consumes. If it doesn't, that's undefined behavior.

```
1 2 3 => a ...
# Undefined behavior. Do you expect to bind `a` to `1`? What if more values
# were added to the stack earlier? And we know nothing about what the caller of
# the current function might have put there.
```

## Notes and Questions

This is a list of things that I want to change in this document. Some are clear
and specific. Others are things that I'm not happy with, but don't have a better
solution for yet.

### Private Bindings

As per the design above, all bindings in a block are public. Eventually, there
needs to be some way to have private bindings though. But I'd like to avoid
adding a new feature for that.

Is there some way to use the existing features to implement that? Maybe a module
shouldn't be used whole. It could return a block instead.

That returned block could reference private bindings via lexical scoping,
without those private bindings being exposed.

### Types

I want types to be values that are constructed by the compile-time code. But so
far, the details are unclear.

### Parameterized Modules

Since modules are built from blocks, just like functions, they can do everything
functions can do. Which includes defining parameters.

It might be best to declare this undefined behavior initially. But eventually,
this would be very useful. Since modules can be seen as compile-time functions,
their compile-time arguments are basically generics.

This is another example of something we get for free by combining our minimal
but orthogonal building blocks. This is something that the reference should go
into.

### Default Context

By importing modules into a block, we define that block's execution context.
There should always be a default context though, with some intrinsic functions.
That context should defer between compile- and runtime.

The default compile-time context, available in modules, could allow us to emit
warnings and errors. The default runtime context would provide stuff like debug
output.

There would be a shared subset between both, which includes functions we can
expect to be available everywhere. Stuff like `+`.

### Modules, Simplified

As an alternative to the `mod` intrinsics, there could be a rule that blocks
bound to CamelCase names become modules.
