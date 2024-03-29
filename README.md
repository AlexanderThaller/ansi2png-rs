# ansi2png-rs

Read ansi sequences from a file and generate a png from it.

```
ansi2png-rs --input-path input.ansi --output-path output.png
```

Optionally the desired width of the png can be specified
```
ansi2png-rs --input-path input.ansi --output-path output.png --png-width 1000
```

## Examples

Generated from [resources/out.ansi](resources/out.ansi):
!["example output"](resources/out.png)

Generated from [resources/ansi2png_help.ansi](resources/ansi2png_help.ansi):
!["ansi2png help output"](resources/ansi2png_help.png)

Generated from [resources/sslscan.ansi](resources/sslscan.ansi) width a png width of 2000:
!["example sslscan"](resources/sslscan.png)

Generated from [resources/tests.ansi](resources/tests.ansi):
!["example tests"](resources/tests.png)

Generated from [resources/rsync.ansi](resources/rsync.ansi):
!["example rsync"](resources/rsync.png)
