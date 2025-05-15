# 한글시계 - HangulClock

![HangulClock](https://homin.dev/asset/blog/img/hangulclock_2022.jpg)

한글시계는 오픈소스(SW/HW) 입니다. [시연 동영상](https://youtu.be/ApymC7qAVTI)

- Make Korea의 2011 컨테스트 수상 작 (1등 없는 2등)
- [A Word Clock by drj113](http://www.instructables.com/id/A-Word-Clock/)에서 영감을 얻었습니다.

## 소스코드

### Rusty HangulClock

~~Please Refer [rusty_hangulclock/README.md](rusty_hangulclock/README.md)~~

> ⚠️ Moved to seperated repository [suapapa/rusty-hangulclock](https://github.com/suapapa/rusty-hangulclock)

### Go program which using Periph.io

Build for ARM64 SBCs:

    $ cd go_hangulclock
    $ GOARCH=arm64 GOOS=linux go build

Then, install binary and system service config by reference `deploy.sh`

### Arduino Sketch (prototype)

Need update submoduls HT1380 and LedControl to build the sketch;

    $ git submodule update

## 링크

- 블로그 글:
  - [다시만든 한글시계 with 러스트](https://homin.dev/blog/post/20241222_rusty_hangulclock/) - 2024-12
  - [11년 만에 완성한, 원조, 한글시계](https://homin.dev/blog/post/20221009_hangulclock_is_re-written_in_golang/) - 2022-09
  - [한글시계 뒷 이야기](https://homin.dev/blog/p=493/) - 2011-11

## 라이선스

### 5x5 한글 조합

<a rel="license" href="http://creativecommons.org/licenses/by/4.0/"><img alt="크리에이티브 커먼즈 라이선스" style="border-width:0" src="https://i.creativecommons.org/l/by/4.0/88x31.png" /></a><br />이 저작물은 <a rel="license" href="http://creativecommons.org/licenses/by/4.0/">크리에이티브 커먼즈 저작자표시 4.0 국제 라이선스</a>에 따라 이용할 수 있습니다.

### 소스코드 

아두이노 스케치와, 고 프로그램의 라이선스가 각각 다릅니다. 각각의 폴더에서 확인하세요.
- Rust program: [rusty_hangulclock/LICENSE](rusty_hangulclock/LICENSE)
- Go program: [go_hangulclock/LICENSE](go_hangulclock/LICENSE)
- Arduino Sketch: [ardino/HangulClock/LICENSE](arduino/HangulClock/LICENSE)