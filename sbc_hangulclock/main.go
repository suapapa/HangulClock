// Copyright 2020, Homin Lee <homin.lee@suapapa.net>. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package main

import (
	"flag"
	"time"

	"github.com/suapapa/go_devices/max7219"
	"periph.io/x/periph/conn/spi/spireg"
	"periph.io/x/periph/host"
)

var (
	h, m int
	demo bool
)

func main() {
	flag.IntVar(&h, "hour", -1, "hour")
	flag.IntVar(&m, "min", -1, "min")
	flag.BoolVar(&demo, "demo", false, "show demo in shot time")
	flag.Parse()

	_, err := host.Init()
	chk(err)

	bus, err := spireg.Open("")
	chk(err)

	dev, err := max7219.New(bus)
	chk(err)

	dev.DisplayTest(true)
	time.Sleep(1 * time.Second)
	dev.DisplayTest(false)

	if h > 0 && m > 0 {
		dispTime(dev, h, m)
		return
	}

	if demo {
		for h = 0; h < 24; h++ {
			for m = 0; m < 60; m += 5 {
				dispTime(dev, h, m)
				time.Sleep(500 * time.Millisecond)
			}
		}
		return
	}

	lastH, lastM := -1, -1
	tk := time.NewTicker(1 * time.Second)
	defer tk.Stop()
	for t := range tk.C {
		h, m := t.Hour(), t.Minute()
		if h == lastH && m == lastM {
			continue
		}
		lastH, lastM = h, m
		dispTime(dev, h, m)
	}
}

func chk(err error) {
	if err != nil {
		panic(err)
	}
}
