# Zuko

Learn me a Lisp for great good! Zuko is a basic Lisp-like programming language written in Rust. It was written primarily as an academic project — a stepping stone towards writing more complex languages in the future.

## Installation

Zuko is packaged into a single `zuko` binary, which you can download from the [releases](https://github.com/ravern/zuko/releases) page. The binary includes the prelude and standard library.

To start a Zuko REPL, simply run the binary without any arguments.

```
$ ./zuko
Zuko v0.1.0
>
```

To run a file containing some Zuko code, simply supply its path as the first arguments. Zuko only supports running one file at a time.

```
$ ./zuko hello-world.zuko
"Hello, world!"
```

## Usage

There are only six special forms in Zuko. These forms are built into the interpreter and should not be redefined.

* `begin` takes in multiple expressions and runs them in order, returning the result of the last expression.
* `if` evaluates the condition passed and returns either the
* `function` creates a function.
* `macro` creates a macro. It works similar to `function` except that it takes in only one argument — the raw list of terms passed into it as arguments — and evaluates its body twice when called.
* `quote` returns the expression passed to it without evaluation.

Everything else "built into" Zuko is defined in either the [prelude](https://github.com/ravern/zuko/blob/master/src/env/prelude.rs) or the [standard library](https://github.com/ravern/zuko/blob/master/src/lib.zuko). The prelude contains functions defined in Rust, so this is where low-level functionality like I/O can be introducted into Zuko. The standard library, on the other hand, is written in Zuko and contain much higher-level functions like math and data manipulation.

There is also some sample code in the `tests/` directory, like a recursive [Fibonacci](https://github.com/ravern/zuko/blob/master/tests/fibonacci.zuko) function and [Newton's method](https://github.com/ravern/zuko/blob/master/tests/square-root.zuko) for determine the square root of a number.

## Missing Features

Zuko is definitely nowhere near complete (it doesn't even have negative numbers). However, with it being an academic project, I have decided to leave them. I'm just too lazy to implement them for now. Of course, I welcome any contributions!

* **No negative numbers.** Support for them would require differentiating between the subtraction operator and the negative sign which means some major reshuffling within the reader.

* **No tail recursion.** Zuko's evaluator is broken down into many different functions so checking for tail recursion would require extra state between those functions. This is probably the one missing feature that I'm not entirely sure how to implement.

* **Macros aren't hygienic.** If a macro returns a symbol, Zuko evaluates it and fetches the corresponding value from the current environment.

* **Pretty shabby error handling.** The entire interpreter just crashes if there is an error like failing to read a file or division by zero. Oh and it doesn't tell you _where_ errors happen either.

* **No distinction between whitespace and newline.** Multiple expressions can be placed on the same line which allows for some crazy looking code if you're into that sort of thing.

* **Empty files don't work.** Due to the way expressions are read, a file must consist of at least one expression, which empty files... don't.


## Acknowledgements

This project wouldn't have been possible without heavy inspiration from other projects and knoweldge from some great books.

* [Klisp](https://github.com/thesephist/klisp) was the main inspiration and motivation behind Zuko. I had started Zuko early this year but only got back to it after witnessing the lightspeed progress of Klisp from Linus's [Twitter feed](https://twitter.com/thesephist). He gave me some [useful tips](https://twitter.com/ravernkoh/status/1334096416382144512?s=20) too!

* [Structure and Interpretation of Computer Programs](https://mitpress.mit.edu/sites/default/files/sicp/full-text/book/book.html) is a classic. A computer science textbook chock full of detailed explanations and guides on building interpreters (especially for Lisp and Scheme), it provided me with most of the knowledge required to build Zuko.

* [Crafting Interpreters](https://craftinginterpreters.com) was how I originally got into building programming languages in the first place. 
Came for the fancy code annoations (seriously they're insane), stayed for the programming languages.