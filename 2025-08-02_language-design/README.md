# Language Design

## About

This is the design for an experimental programming language. As of this writing, only this design exists. I'm just dreaming here. There's no implementation, and there might never be one.

Not only that, but the design itself is pretty raw. It needs a few more rounds of review and refinement, and I might only do that when inspiration strikes. There's no schedule here.

This design is written in the form of documentation, because I found that to be an effective tool to channel my thoughts. **Don't mistake this for actual documentation of an actual language.**


## Introduction

### Basic Operations

The language is stack-based, as that makes for a very concise syntax that is easy to work with.

Well, for me, who's going to be stuck with implementing this. You might see it differently. And I might join you in that view, later on, when I'll become a user of the language myself.

But let's move on. Here's a basic example.

```
1 2 +
```

We have three tokens here, all of which refer to functions. All of those intrinsic functions, that are built into the compiler.

`1` and `2` simply return these respective values and put them on the stack. `+` then takes those two values and puts their sum on the stack.

Once this is finished, the stack contains the number `3`. Plus whatever was on there in the first place.

This leaves a lot of questions unanswered. Like, what's actually the type of those numbers? What other number types are there? How does the compiler know what code to generate for `+`?

Long-term (and that day, most like, will never come), the answer might be type inference and other magic. But for now, let's not worry about it and prioritize ease of implementation.

There's just one number type, and which one that is is implementation-defined. And I guess relying on anything but positive numbers between 0 and 127 is undefined behavior then.

### Assignment

I don't know about you, but I have a hard time dealing with _just_ a stack. It's a skill issue. Others can do it. But implementing assignment is easier than spending a decade or five achieving enlightenment.

```
1 2 => a b .
```

This is the assignment operator (`=>`). After it, you put a whitespace-delimined list of identifiers. You close that list with a period (`.`).

The assignment operator consumes as many values from the stack, as there are identifiers in its list. It assigns names to those values, so they might be accessed later.

Here, we access the names that we assigned above.

```
a b +
```

### Stack Manipulation

There's a nice side effect to having the assignment operator: It saves us from having to learn all those stack manipulation operators, that other stack-based languages have.

We can just implement those using assignment.

```
# dup
1 => a . a a

# swap
1 2 => a b . b a

# over
1 2 => a b . a b a
```

### Formatting

All that the compiler cares about, is that tokens are separated with whitespace. It doesn't care how much whitespace that is, or what kind.

Here's an assignment example from above:

```
1 => a . a a
```

It could also be written like this:

```
1 => a .
a a
```

Or like this:

```
1
=>
a
.
a
a
```

Or literally any other way. As long as the order of the tokens is the same, and there's whitespace between them, the resulting code is equivalent.

### Blocks

If we put `{` and `}` around some code, we group it into a block.

```
{ 1 2 + }
```

A block is a value, just like a number. The result of the code above, is to put a block that contains this code on the stack.

We could then assign it a name:

```
{ 2 + } => add_two .
```

Or we could apply it:

```
1 { 2 + } apply
# The stack now contains `3`.
```

Or we could combine both:

```
{ 2 + } => add_two .
1 add_two apply
```

Having to write `apply` everywhere is not very convenient. And you might note, that we don't have to do this for intrinsic functions like `1`, `2`, `+`, or `apply` itself.

That's because what we assigned to a name here is not a function. It is, as I said, a block. And blocks are a building block. They can be made into functions, and other things as well.

### Functions

So let's do just that. Let's use a block to define a function.

```
{ 2 + } fun => add_two .
```

This is _almost_ what we did above, except that we call the intrinsic function `fun` after defining the block. `fun` consumes a block and returns a function that wraps that block.

Functionts are mostly like blocks, except that when we evaluate them, we don't put a function value on the stack. We apply the function. And evaluating a value is what we do, when we call it by name.

```
{ 2 + } fun => add_two .
1 add_two
# The stack now contains `3`.
```

That's the same as above, except we no longer need that `apply`.

### Function Parameters

So far, we've only defined functions with implicit parameters. `add_two` consumes an argument, a number, but it doesn't specify that explicitly.

It does so by putting a number on the stack, but then calling a function that takes 2 numbers from the stack. As a result, the whole function consumes one number and returns one number.

And that's often fine, but sometimes we want named function parameters. And we can already do that!

```
{ => n
  n 2 +
}
  fun => add_two
```

This is the first example where we do something new, but don't require a new feature to do it! We just combine things we already had; blocks, functions, and assignments; and we get a function with named parameters.

Strictly speaking, the parameter is still implicit. Only instead of the `+` consuming more than the function itself provides, we have an assignment that consumes something that the caller needs to supply.

And by putting such an assignment first in a block, we can make the requirements of that block very explicit.

This demonstrates a very important design principle: A minimal set of orthogonal features, that can be combined in interesting ways to build up more interesting constructs.

### Higher-Order Functions

By wrapping a function call into a block, we can use that function as the argument of a higher-order function.

```
1 2 3 { + } do_twice
# The stack now contains `6`.
```

Here we pass a block to a function that, presumably, applies that block twice. We could also pass a function instead.

```
1 2 3 { + } fun do_twice
```

If `do_twice` calls `apply` on its argument, then that could be defined for functions as well as for blocks. In that case, this would result in the same output.

Or `apply` might only be defined for blocks and not for functions, to avoid redundancy and prevent confusion and/or bad habits. Let's just answer that question with "undefined behavior" and move on.

### Arrays

If we put some values between `[` and `]`, that groups them into a single array value, and puts that on the stack.

```
[ 1 2 3 ]
```

And with that, we can write the "multi-addition" above a bit more generally.

```
[ 1 2 3 ] { + } fold
```

In principle, we could allow arbitrary code between `[` and `]`. As long as that results in zero or more values of the same type, it would make for a valid array.

```
1 [ 2 + ]
# Results in `[ 3 ]`.

[ { 1 } 3 times ] # results in
# Results in `[ 1 1 1 ]`.
```

But there's a catch: This is a compiled language, and one with explicit memory allocation at that. So no auto-boxing shenanigans. Hence these arrays live on the stack, and we must know the size of these arrays at compile-time.

A sufficiently smart compiler could determine that for all of the above. But since we're going to start with a pretty dumb one, let's take the easy way out: Anything but a series of literals is undefined behavior.

(There is another option: dynamically sized stack allocations. My language designer instincts tell me that this is going to lead to complexity, and restrictions elsewhere, most likely. Let's avoid it for now.)

To get values out of an array later, we can use the `get` function.

```
[ 1 2 3 ] 0 get
# Results in `1`.
```

Or we can update an array after we created it.

```
[ 1 2 3 ] 0 4 set
# Results in `[ 4 2 3 ]`.
```

`get`ing or `set`ing with an index that is out of bounds for the array is undefined behavior.

### Records

Arrays are pretty good, as far as aggregate data types go. But we need some more of those to have some real fun. Hence, records.

```
{
  1 => a .
  2 => b .
}
  rec
```

Here we have a block, in which we make some assignments. And then we pass that to another intrinsic function, `rec`. It applies the block, just like `apply`, but it doesn't put the block's output on the stack.

Instead, it takes all the assignments in the block, uses those as the fields in a new record, and puts that record on the stack.

What happens to the block's result? Well, in this case it doesn't make a difference, because the block has no output. If it had, that would be, you guessed it, undefined behavior.

We can use variants of the `get` and `set` functions that we used for arrays, to access the fields of records. We just need a new type of literal for that, the symbol.

```
{ 1 => x . 2 => y . } :x get
# Results in `1`.

{ 1 => x . 2 => y .} :x 3 set
# Results in `{ 3 => x . 2 => y . }`.
```

A symbol starts with `:`. What follows must be a valid identifier. If you use `get` or `set` with a symbol that doesn't match a field of the record, that's undefined behavior.

### The Compiler

As I said above, this is a compiled language. And now it's time to talk about the compiler.

The compiler is also an interpreter! We could just feed all of the examples we've seen so far into it directly, without any additional structure, and it would run them. If they return a result, it would display that.

But that's not all it does. Because the code it runs is not just any code. It's compile-time code! It takes the results of that code, and then uses those to do the compiling.

What I said above, about not needing any additional structure to run those examples, is not completely true. _We_ don't need to add anything else. But as far as the compiler is concerned, there is some implicit structure here.

To the compiler, the top-level context, where we write all that code that we don't put into blocks, that top-level context is a block too. It's blocks all the way down!

I said above, if that block has a result, the compiler will display that. But that's not all that happens. It then translates that block into a WebAssembly module. All bindings in the block become exported symbols.

A value is exported as a global. Functions are exported as functions. (And there's going to be a third category of binding, modules. We'll get to those in a moment.)

I hope that this means the compiler can be relatively simple. It tokenizes, it parses, it evaluates, it translates. But that is just dumb infrastructure. There is no intelligence, no decision-making.

Because that's what your program does. The part that's evaluated at compile-time.

### Modules

I said that the top-level context is implicitly a block. But it's not just any block, and not a singular block that the compiler treats in a special way.

It's another example of a block wrapped in something else. Like a function. Except, this one is more like a function that runs at compile-time. It's a module.

And as with functions, we can use a special intrinsic function to define our own.

```
{
  { 2 + } fn => add_two .
  { 3 + } fn => add_three .
}
  mod => SpecialAddition .
```

Here's a module that defines two functions that each do some special kind of addition. When the compiler encounters this module, it treats it like the top-level module: It looks at its contents and translates those into code.

Only, since this is not the top-level module, the bindings in there don't become exported symbols of the WebAssembly module.

They are made available to other code instead. Code that refers to the module via the `SpecialEdition` binding.

### To Be Continued...

That's as far as I made it. Much more left to do, later.


## Roadmap

Once the design is a bit more solid, the goal of this section is to extract a few minimal steps that I can start with, that would lead to the best result with the least amount of work.


## Reference

The introduction above starts with the basics, and builds up from there. It follows a path that maps out what would actually need to be implemented, at a minimum, to make the language useful.

This reference builds on that, by expanding the concepts already introduced with new capabilities. It's more like a menu of things to implement later, should there ever be time and focus to make that happen.

### Number Literals

Once there are multiple number types, but before the compile has become advanced enough to reach sentience, we can create values of those by specifying the type explicitly.

```
1:u32 # 32-bit, unsigned
2:s8  # 8-bit, signed
```

### Arrays

Here's one thing where the introduction is actually a bit too luxurious: Arrays can be replaced with blocks and a built-in function.

```
[ 1 2 3 ]
# equivalent to
{ 1 2 3 } arr
```

Maybe I'll go back later and update the array section in the introduction. But either way, it would make sense to make the nicer array syntax just syntax sugar for the combination of block and `arr`.

### Destructuring Assignment

The introduction goes with rather simple means for working with arrays and records. A different take on that would be destructuring assignment.

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

# Acces only one, drop all others.
vec => { x ... } .
```

And the various forms would be combined.

```
1 array 2 vec => a { x ... } b [ c ... ] .
```

But the assignment needs to always define exactly how many values from the stack it consumes. If it doesn't, that's undefined behavior.

```
1 2 3 => a ...
# Undefined behavior. Do you expect to assign `a` to `1`? What if more values
# were added to the stack earlier? And we know nothing about what the caller of
# the current function put there.
```


## Notes and Questions

This is a list of things that I want to change in this document. Some are clear and specific. Others are things that I'm not happy with, but don't have a better solution for yet.

### Assignments and Bindings

Let's be a bit more precise about the nomenclature. And assignment is what the `=>` operator does. It's an action. What results from an assignment, a name assigned to a value, that's a _binding_.

I started making that change in the newer sections, but need to retrofit the older ones too.

### Private Bindings

As per the design above, all bindings in a block are public. Eventually, there needs to be some way to have private bindings though. But I'd like to avoid adding a new feature for that.

Is there some way to use the existing features to implement that? Maybe a module shouldn't be used whole. It could return a block instead.

That returned block could reference private bindings via lexical scoping, without those private bindings being exposed.

### Types

I want types to be values that are constructed by the compile-time code. But so far, any details are unclear.
