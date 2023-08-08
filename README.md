# scry: an opinionated `stat` and `wc` clone

scry is an opinionated command-line utility that combines the metadata-focused features of `stat` with the word-, line-, and paragraph-counting capabilities of `wc`.

## how it differs from `stat` and `wc`

scry differs from its predecessors in that it's written in Rust (btw) and is more
selective about the information it provides than `stat` and `wc`. By default,
scry offers less information than `stat` and more information than `wc`.

## performance

Benchmarks conducted via hyperfine put scry at roughly 1.91x the performance of `stat`
and roughly 3.11x the performance of `wc` on small files. (Note that the exact figures vary based on the size of the file, the arguments supplied to hyperfine, etc.)

---

## why does this exist?

Because I wanted to learn Rust, don't always want all of the information `stat`
provides, and occasionally want to be able to view a file's metadata as well as
information about the contents of the file without having to use multiple tools.

## why is it called 'scry'?

Because programming is spellcasting, it sounds cool, and it's the same length as
"stat"... until you take the mandatory arguments into consideration. Whoops.

## will this work on my system?

Beats me! 

## can i change what data is printed?

Absolutely! The printing is handled by two `println!` statements in main.rs; you can
modify them and build from source to get access to other fields. The full list of
available fields, which is similar to those offered by `stat`, can be found in the
metadata.rs module. (Note that the available data varies by platform.)

---

## roadmap

scry is a learning project; its features are dictated by my progress through learning
Rust, my expectations for a utility of this type, and a certain amount of whimsy. 

That said, the following features at least seem interesting:

* the ability to analyze multiple files at once
* a database that can be used to track invocations of scry on a file over time
* more in-depth analysis of a file's contents

These features might never arrive, however, or might be part of a different project.

