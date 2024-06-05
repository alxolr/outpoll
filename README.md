# OutPoll

A command line tool to poll a command and notify you when a specific output is reached.

## Installation

```bash
cargo install outpoll
```

## Usage

```bash
outpoll -c "echo hello" -w "hello"
```

This is useful when you have some long running command and you want to be notified when a specific output is reached.
For example when you publish a package and you want to be notified when the package is published.


```bash
outpoll -c "npm view {package-name} versions" -w "{version}"
```

