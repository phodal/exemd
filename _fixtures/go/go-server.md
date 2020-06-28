# Go Server

1. Get Deps

```cli
go get github.com/kyokomi/emoji
```

2. code

```go
package main

import (
	"fmt"

	"github.com/kyokomi/emoji"
)

func main() {
	fmt.Println("Hello World Emoji!")

	emoji.Println(":beer: Beer!!!")

	pizzaMessage := emoji.Sprint("I like a :pizza: and :sushi:!!")
	fmt.Println(pizzaMessage)
}
```