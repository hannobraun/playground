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

## Introduction

### Basic Operations

The language is stack-based, as that makes for a very concise syntax that is
easy to work with.

Well, easy to work with for me, who's going to be stuck with implementing this.
You might see it differently. And I might join you in that view, later on, when
I'll become a user of the language myself.

But let's move on. Here's a basic example.

```
1 2 +
```

We have three identifiers here, all of which refer to functions. And all of
those are intrinsic functions, that are built into the compiler.

`1` and `2` simply return these respective values and put them on the stack (all
numbers are signed 32-bit integers). `+` then takes those two values from the
stack, and puts their sum back on.

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

A block is a value, just like a number. The result of the code above, is to put
a block that contains this code on the stack.

We can apply that block, to execute the code it contains.

```
1 { 2 + } apply
# The result is `3`.
```

Or we can bind a name to the block, and execute it using that name.

```
{ 2 + } => add_two .
1 add_two
# The result is `3`.
```

By calling a block by name, we apply it automatically. This way, we define
_functions_.

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

### The Compiler

As I said above, this is a compiled language. And now it's finally time to talk
about the compiler.

The compiler is also an interpreter! We could just feed all of the examples
we've seen so far into it directly, without any additional structure, and it
would run them. If they return a result, it would display that.

But that's not all it does. Because the code it runs is not just any code. It's
compile-time code! It takes what that code defines, then translates that into a
WebAssembly module.

What I said above, about not needing additional structure to run those examples,
is not completely true. _We_ don't need to add anything else. But to the
compiler, there is some implicit structure here.

To the compiler, the top-level context, where we write all that code that we
don't put into blocks, that top-level context is a block too. It's blocks all
the way down!

I said above, if that block has a result, the compiler will display that. But
that's not all that happens. It then turns all the bindings in that block into
exports of the WebAssembly module.

A value is exported as a global. Functions are exported as functions. (And
there's going to be a third category of binding, modules. We'll get to those in
a moment.)

I hope that this means the compiler can be relatively simple. It tokenizes, it
parses, it evaluates, it translates. But that is just dumb infrastructure. There
is no intelligence there, no decision-making.

Because that intelligence is what your program provides. The part that's
evaluated at compile-time.

### Modules

I said that the top-level context is implicitly a block. But it's not just any
block, and also not a unique one that the compiler needs to treat in a special
way.

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

### To Be Continued...

This is as far as I made it. Much more left to do, later. For example:

- Importing Modules
- Error Handling
- Memory Management

## Roadmap

Once the design is a bit more solid, the goal of this section is to extract the
first few minimal steps, that I can start with.

## Reference

The introduction above starts with the basics, and builds up from there. It
follows a path that maps out what we actually need to implement, at a minimum,
to make the language useful.

This reference builds on that, by expanding the concepts already introduced with
new capabilities. It's more like a menu of things to implement later, should
there ever be time and focus to do that.

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

### Closures

I think it makes sense for all blocks to close over their lexical environment.
That would naturally allow runtime functions to use things defined in their
compile-time context, like other functions.

This closing over their environment should be simple, only copying bindings. If
we allowed references, we'd either open the door for easy but horrible mistakes,
require a borrow checker, or require auto-boxing shared bindings.

Implemented naively, this would require functions to be defined in a specific
order. A function could only call functions that were defined before it. But I
think we can get around that, with a relatively simple rule.

The rule would be, that blocks can access all bindings in their environment,
that are available when the block is moved:

- A block passed as an argument is moved then and there. It only can access what
  was defined before it.
- A function in a module is moved by the compiler, between evaluation and
  translation of the module. At that point, the whole module has been evaluated
  and all bindings are available.
- The latter case would still work, if modules returned a block as a result, to
  enable private and public bindings. Then the move would be from the module to
  the export block, which could be constructed at the very end of the module.
