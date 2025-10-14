Previous error format in cdosato:

```
>>> do sayln("Hello world");

ERROR:
E8: Unexpected Token
At line 1:24 in <stdin>

do sayln("Hello world");
                       ^
```

New error format proposal:

```
>>> do sayln("Hello world");

Uncaught Exception:
File <stdin>, line 1, column 24 (1:24)
    do sayln("Hello world");
                           ^
                       
SyntaxError: Unexpected Token
```

Colors:

- `Uncaught Exception:` in red
- `File <stdin>, line 1, column 24 (1:24)` in yellow
- `SyntaxError:` in red
- `Unexpected Token` in white
- Source code line in white