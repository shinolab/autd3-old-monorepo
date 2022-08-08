# Go

[autd3-go](https://github.com/shinolab/autd3-go)はgoに対応したラッパーを提供している.

## Installation

go modでインストールできる.

```
go get github.com/shinolab/autd3-go/v2
```

## Usage

基本的には, C++版と同じになるように設計している.

たとえば, [Getting Started](../Users_Manual/getting_started.md)と等価なコードは以下のようになる.

```go
package main

import (
	"fmt"
	"os"

	"autd3-go-example/samples"

	"github.com/shinolab/autd3-go/v2/autd3"
	"github.com/shinolab/autd3-go/v2/soem"
)

func onLost(msg string) {
	println(msg)
	os.Exit(-1)
}

func getAdapter() string {
	adapters := soem.EnumerateAdapters()
	for i, adapter := range adapters {
		fmt.Printf("[%d]: %s, %s\n", i, adapter.Desc, adapter.Name)
	}

	fmt.Print("choose: ")

	var i int
	if _, err := fmt.Scanln(&i); err != nil {
		fmt.Printf("failed to read integer: %s\n", err)
		os.Exit(-1)
	}

	if i >= len(adapters) {
		fmt.Print("index out of range\n")
		os.Exit(-1)
	}

	return adapters[i].Name
}

func main() {
	cnt := autd3.NewController()
	defer cnt.Delete()

	cnt.AddDevice([3]float64{0, 0, 0}, [3]float64{0, 0, 0})

	ifname := getAdapter()
	link := soem.NewSOEM(ifname, cnt.NumDevices()).OnLost(onLost).Build()
 
	if !cnt.Open(link) {
		println(autd3.GetLastError())
		os.Exit(-1)
	}

	cnt.Clear()
	cnt.Synchronize()

	firmList := cnt.FirmwareInfoList()
	for _, info := range firmList {
		println(info)
	}

	config := autd3.NewSilencerConfig()
	defer config.Delete()
	cnt.Send(config)

	g := autd3.NewFocus([3]float64{90, 80, 150})
	defer g.Delete()
	m := autd3.NewSine(150)
	defer m.Delete()

	cnt.Send(m, g)

    var input string
	fmt.Scanln(&input)

    cnt.Close()
}
```

より詳細なサンプルは[autd3-goのexample](https://github.com/shinolab/autd3-go/tree/master/examples)を参照されたい.

その他, 質問があれば[GitHubのissue](https://github.com/shinolab/autd3-go/issues)にてお願いします.
