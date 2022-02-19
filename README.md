# Rusty Server

Learning exercise to implement HTTP server in rust from scratch.

## HTTP Message

```
    | Start Line    | : POST /path HTTP/1.1
    | HTTP Header   | : Multi-line header
    | HTTP Header   | : Accept: txt/html
    | HTTP Header   | : Host: localhost:8080
    |               | : A Blank Line
    | Body          | : Multi-line body
```


## Resources

- [HTTP Messages](https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages)