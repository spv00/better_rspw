# RSPW

uhhh, a password generator with cmdline parser and CLI frontend or sum in rust. this is neither fast, nor safe, nor useful

## Installation
```
git clone https://github.com/spv00/better_rspw && cd better_rspw
```

```
cargo build --release
```

## Usage
`rspw <OPTIONS>`

### Options
| Flag | Option | Notes | Usage |
|------|--------|-------|-------|
| -l | Set the length | Must be a 32bit signed integer | `rspw -l 12` |
| -p | Select which characters to include | `u`: Uppercase, `l`: Lowercase, `d`: Digits, `s`: Special | `rspw -p uld` |
| -e | Exclude specific characters from the generator |  | `rspw -e abcdefgHIJKLMNOP123!?()` |
| -i | Interactive Mode | This overwrites all commandline flags set | `rspw -i` |