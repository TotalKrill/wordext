# wordext

Extracts the nth word of the input, either of the entire input, or per line

## example

This can be used to create a worse edition of killall using kill, example, killing all vim processes

```
ps -e | grep vim | wordext -r -n 0 | xargs kill -9
```

## installation

I am not going to learn how to package this, so it will be crates.io only, i doubt it will be useful
enough for me to do otherwise

cargo install wordext

## why

I have found myself trying to pipe a single word from an input, and wanting to pipe this to another
command one or two times to often, and thus wrote this, so I can install it easily on any computer I
use regularly, and not have to learn regex :)
