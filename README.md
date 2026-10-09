# Identicons

This project is inspired by [github post](https://github.blog/news-insights/company-news/identicons/) on Identicons.

Identicons are unique visual representation created from hash value of data. Data includes ip address, userid or any other unique strings.

## Algorithm

1. Take a unique string along with final image height and width as input
2. Hash the contents of string. The current approach uses `SHA-256` hashing to produce unique 32 bytes.
3. Use this 32 bytes to generate a unique image

## Usage

There are two patterns supported: `asymmetrical` and `symmetrical`.

```bash
cargo run -- foo 420 420 --pattern asymmetrical "foo_asym.path"
cargo run -- foo 420 420 --pattern symmetrical "foo_sym.path"
```

## Example

Asymmetrical

![foo_asym](assets/foo_asym.png)

Symmetrical

![foo_sym](assets/foo_sym.png)