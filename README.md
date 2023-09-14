# Epoch CLI

## Overview

This Rust CLI application provides a way to convert epoch timestamps to human-readable local time and vice versa. It also supports optional timezone adjustments.

## Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/ztroop/epoch.git && cd epoch
cargo install --path .
```

## Usage

### Convert epoch to local time:

```bash
epoch -e 1631560200
```

### Convert epoch to a specific timezone (UTC+3):

```bash
epoch -e 1631560200 -z UTC+3
```

### Convert a date string to epoch:

```bash
epoch -d "2023-09-14T12:34:56" -f "%Y-%m-%dT%H:%M:%S"
```

### Convert a date string to epoch with a specific timezone (UTC-3):

```bash
epoch -d "2023-09-14T12:34:56" -f "%Y-%m-%dT%H:%M:%S" -z UTC-3
```

### Timezone Format

The timezone offset should be specified as `UTC±N`, where `N` is the hour offset from UTC. For example:

- For UTC+3: `UTC+3`
- For UTC-3: `UTC-3`
