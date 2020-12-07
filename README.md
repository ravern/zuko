# Zuko

Learn me a Lisp for great good! Zuko is a Lisp-like programming language written in Rust. It is purely 

Zuko was written primarily as a practical way for me to gain experience writing interpreters. For me, it's also a stepping stone towards building more complex programming languages in the future.

## Usage

Zuko can be run in two 

## Implementation

## Missing Features

Zuko is definitely nowhere near complete (it doesn't even have negative numbers). However, with Zuko being an academic project, I have decided to leave them. I'm just too lazy to implement them for now. Of course, I welcome any contributions!

* **No negative numbers.** Support for them would require differentiating between the subtraction operator and the negative sign which means some major reshuffling within the reader.

* **No tail recursion.** Zuko's evaluator is broken down into many different functions so checking for tail recursion would require extra state between those functions. This is probably the one missing feature that I'm not entirely sure how to implement.

* **Macros aren't hygienic.** If a macro returns a symbol, Zuko evaluates it and fetches the corresponding value from the current environment.

* **Pretty shabby error handling.** The entire interpreter just crashes if there is an error like failing to read a file or division by zero. Oh and it doesn't tell you _where_ the error happens.

* **No distinction between whitespace and newline.** Multiple expressions can be placed on the same line which allows for some crazy looking code if you're into that sort of thing.

* **Empty files don't work.** Due to the way expressions are read, a file must consist of at least one expression, which empty files unfortunately don't.


## Resources and Credits

This project wouldn't have been possible without heavy inspiration from other projects and knoweldge from some great books.

* [Klisp](https://github.com/thesephist/klisp) was the main inspiration and motivation behind Zuko. I had started Zuko early this year but only got back to it after witnessing the lightspeed progress of Klisp from Linus's [Twitter feed](https://twitter.com/thesephist). He gave me some [useful tips](https://twitter.com/ravernkoh/status/1334096416382144512?s=20) too!

* [Structure and Interpretation of Computer Programs](https://mitpress.mit.edu/sites/default/files/sicp/full-text/book/book.html) is a classic. A computer science textbook chock full of detailed explanations and guides on building interpreters (especially for Lisp and Scheme), it provided me with most of the knowledge required to build Zuko.

* [Crafting Interpreters](https://craftinginterpreters.com) was how I originally got into building programming languages in the first place. 
Came for the fancy code annoations (seriously they're insane), stayed for the programming languages.