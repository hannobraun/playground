# StackAssembly

## About

StackAssembly is a stack-based, assembly-like, weirdly functional programming language. I created it as a platform for my personal experiments with programming language and tooling design.

```stack
1 @plus_two call

:plus_two
  2 +
```

The language is currently interpreted, and the interpreter is available as a Rust library. Freestanding usage is not supported yet.

## Status

Work on StackAssembly's first version is concluding. The language is quite simplistic right now, but may be capable enough to write real code with. This has not been proven, as no code outside of the test suite exists as of yet.

Either way, this is a very simplistic language; just a personal experiment. So capable enough or not, don't expect the development experience to be productive or pleasant.

## Documentation

The article I'm currently writing would be the best introduction to StackAssembly. It's scheduled to come out shortly after this repository becomes public. I'll add a link here, once it's available.

If you're interested in all the details and can bear trudging through something that's written as less of a friendly explainer, check out the [test suite](src/tests/), or even the [source code](src/) itself.

As of this writing, there is nothing else that could pass for documentation. I'm sorry. I tried to write some, but it's just too early.

Once I had a chance to give StackAssembly a real go myself, which will certainly involve the creation of tooling to make it easier to try out , along with examples of real code, I'll give it another go.

## License

This project is open source, licensed under the terms of the [Zero-Clause BSD License][0BSD] (0BSD, for short). This basically means you can do anything with the code, without restrictions, but you can't hold the authors liable for any problems.

See [LICENSE.md] for details.

[0BSD]: https://opensource.org/licenses/0BSD
[LICENSE.md]: LICENSE.md
