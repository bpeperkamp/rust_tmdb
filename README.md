![Build status](https://github.com/bpeperkamp/rust_tmdb/actions/workflows/rust.yml/badge.svg?branch=main?event=push)

## My first experiment in Rust

This command will find a search term in TMDB for you. You can use it to look for series and movies.

In order to build, you'll need to get an API key from TMDB. Start reading here and get an account: [TMDB](https://developer.themoviedb.org/docs/getting-started). After you are done, copy the .env.example file to .env and add your **Bearer token (on the bottom of the [API settings](https://www.themoviedb.org/settings/api) page]).

To use the command (after compiling), type: 

./rust_tmdb -m serie -t "Dark Matter"

or 

./rust_tmdb -m movie -t "Empire of the sun"

-m stands for media_type
-t for title

If you add --help you'll get some more usage tips/parameters.

I like Rust so far!