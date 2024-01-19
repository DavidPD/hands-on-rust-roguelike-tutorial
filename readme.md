# The Rusty Roguelike - From Tutorial: Hands on Rust by Herbert Wolverson

This repo contains my results from following along with the book "Hands on Rust" by Herbert Wolverson. You can find and purchase the book here: https://hands-on-rust.com, and the official source code is published here: https://github.com/thebracket/HandsOnRust. He's also got another large tutorial project here: http://bfnightly.bracketproductions.com/rustbook/. Everything here was copied over by hand and I made some changes whenever I thought it made more sense, so the code won't match 100%.

As someone who has already learned the basics of Rust this book was a fantastic practical exercise and I can't recommend it highly enough.

I'm still pretty new to Rust, so after following through the whole book I decided to extend my learning by updating libraries to their latest versions, writing tests, fixing clippy linter messages, and trying to refactor the code to use some newer less error-prone features added to the Legion ECS library.

## Legion System Attribute Macros

One thing that I found frustratingly error-prone was using attribute macros to declare what components your systems read/write. Since your queries are separate from the attributes it's really easy to forget or misattribute a component and instead of a compiler error your game just crashes, or worse, your system might just fail silently. For these reasons one thing I'm trying to do is refactor systems to use Query parameters introduced I think in Legion v0.4.0.

## Tests

I've never had a Rust project large enough to warrant significant testing until now, so I'm also trying to write unit tests as I refactor different systems. One goal I have is to see if I can make tests easy to bootstrap and write for new systems.
