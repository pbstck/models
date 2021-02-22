# Models

This repository contains protobuf messages used by various components at pubstack.

## How to use

### golang

```golang
package main

import (
  "github.com/pbstck/models/golang/models"
  "fmt"
  "encoding/json"
)

func main() {
  impression := models.Impression{
    Cpm: 0.42,
  }
  ser, _ := json.Marshal(impression)
  fmt.Println(string(ser))
}
```

### rust

`Cargo.toml`

*everything else is omitted for clarity*
```toml
[dependencies]
models = { git = "https://github.com/pbstck/models.git", branch = "master" }
```

`enum.rs`
```rust
use models::{Auction, Impression, ViewableImpression};

pub enum Event {
    Impression(Impression),
    Auction(Auction),
    ViewableImpression(ViewableImpression),
}
```
