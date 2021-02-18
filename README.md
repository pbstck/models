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

```
package main

import (
  model "github.com/pbstck/models/golang"
  "fmt"
  "encoding/json"
)

func main() {
  impression := model.Impression{}
  ser, _ := json.Marshal(impression)
  fmt.Println(string(ser))
}
```

## Note

Do not move the go.mod file from the root. This is due to go with github nmot accpeting module not declared at the root of a repository