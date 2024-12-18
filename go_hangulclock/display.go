package main

import (
	"log"

	"github.com/pkg/errors"
	"github.com/suapapa/go_devices/max7219"
)

var (
	// 패널의 행열이 아래와 같이 뒤섞여 있음
	row = []int{4, 6, 2, 3, 7} // TODO 이거 틀림
	col = []int{6, 1, 5, 0, 4}

	// 화면에 표시될 내용
	buff = make([]byte, 8)
)

func buffClearAll() {
	for i := range buff {
		buff[i] = 0x00
	}
}

func buffSet(r, c int) {
	r, c = row[r], col[c]
	buff[r] |= (1 << c)
}

func updateDisp(dev *max7219.Dev) error {
	for i, r := range buff {
		if err := dev.Write(byte(i)+1, r); err != nil {
			return errors.Wrap(err, "fail to update disp.")
		}
	}
	return nil
}

func dispTime(dev *max7219.Dev, h, m int) {
	log.Printf("dispTime: h=%d m=%d", h, m)
	m10, m1 := m/10, m%10
	switch m1 {
	case 1, 2, 3:
		m1 = 0
	case 4, 5, 6:
		m1 = 5
	case 7, 8, 9:
		m1 = 0
		m10 += 1
	}
	if m10 >= 6 {
		m10 = 0
		h += 1
	}

	buffClearAll()
	defer func() {
		dev.Clear()
		updateDisp(dev)
	}()

	if (h == 0 || h == 24) && (m10+m1) == 0 {
		buffSet(3, 0)
		buffSet(3, 1)
		return
	}

	if h == 12 && (m10+m1) == 0 {
		buffSet(3, 1)
		buffSet(4, 1)
		return
	}

	if h > 12 {
		h -= 12
	}
	switch h {
	case 0, 12:
		buffSet(0, 0)
		buffSet(1, 0)
		buffSet(2, 4)
	case 1:
		buffSet(0, 1)
		buffSet(2, 4)
	case 2:
		buffSet(1, 0)
		buffSet(2, 4)
	case 3:
		buffSet(0, 3)
		buffSet(2, 4)
	case 4:
		buffSet(0, 4)
		buffSet(2, 4)
	case 5:
		buffSet(0, 2)
		buffSet(1, 2)
		buffSet(2, 4)
	case 6:
		buffSet(1, 1)
		buffSet(1, 2)
		buffSet(2, 4)
	case 7:
		buffSet(1, 3)
		buffSet(1, 4)
		buffSet(2, 4)
	case 8:
		buffSet(2, 0)
		buffSet(2, 1)
		buffSet(2, 4)
	case 9:
		buffSet(2, 2)
		buffSet(2, 3)
		buffSet(2, 4)
	case 10:
		buffSet(0, 0)
		buffSet(2, 4)
	case 11:
		buffSet(0, 0)
		buffSet(0, 1)
		buffSet(2, 4)
	}

	if m10+m1 == 0 {
		return
	}

	switch m10 {
	case 1:
		buffSet(3, 4)
		buffSet(4, 4)
	case 2:
		buffSet(3, 2)
		buffSet(4, 2)
		buffSet(4, 4)
	case 3:
		buffSet(3, 3)
		buffSet(3, 4)
		buffSet(4, 4)
	case 4:
		buffSet(4, 0)
		buffSet(4, 2)
		buffSet(4, 4)
	case 5:
		buffSet(4, 1)
		buffSet(4, 2)
		buffSet(4, 4)
	}

	if m1 == 5 {
		buffSet(4, 3)
		buffSet(4, 4)
	}
}
