package main

import (
	"fmt"
	"os"
	"strings"

	svg "github.com/ajstarks/svgo"
)

var (
	width    = 500
	height   = 500
	charSize = 100
	canvas   = svg.New(os.Stdout)

	font     = "BusanBada" // "BaramExtraBold"
	fontsize = 50
	color    = "black" // "blue"
	opacity  = 1.0     // 0.5
)

func main() {
	canvas.Start(width, height)

	canvas.Grid(0, 0, width, height, charSize, "fill:none;stroke:black")

	canvas.Gstyle(
		fmt.Sprintf("font-family:%s;font-size:%dpt;text-anchor:middle;fill:%s;fill-opacity:%.2f",
			font, fontsize, color, opacity),
	)

	chars := "열한다세네\n두여섯일곱\n여덟아홉시\n자정이삼십\n사오십오분"
	for i, l := range strings.Split(chars, "\n") {

		for j, r := range []rune(l) {
			x := 100 * j
			x += (charSize / 2)
			y := 100*i + (fontsize / 2)
			y += (charSize / 2)
			canvas.Text(x, y, string(r))
		}
	}

	canvas.Gend()

	// canvas.Gend()
	canvas.End()
}
