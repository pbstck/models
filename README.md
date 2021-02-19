# Models

This repository contains protobuf messages used by various components at pubstack oto communicate between each others.

## Components:

* Aggregation processing (golang)
* Reporting consumer (golang)
* Intake:
  * Intake server (golang)
  * Mithrandir (rust)

## How to use

### golang

```golang
package main

import (
  model "github.com/pbstck/models/golang"
  "fmt"
  "encoding/json"
)

func main() {
  impression := model.Impression{
    Cpm: 0.42,
  }
  ser, _ := json.Marshal(impression)
  fmt.Println(string(ser))
}
```

### rust

`Cargo.toml`

*everythig else is ommited for clarity*
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

## Note

Do not move the go.mod file from the root. This is due to go with github not accepting module not declared elsewhere than the root of this repository
