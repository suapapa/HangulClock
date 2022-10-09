# 한글시계 - HangulClock

![HangulClock](https://homin.dev/asset/blog/img/hangulclock_2022.jpg)

한글시계는 오픈소스(SW/HW) 입니다. [시연 동영상](https://youtu.be/ApymC7qAVTI)

- Make Korea의 2011 컨테스트 수상 작 (1등 없는 2등)
- [A Word Clock by drj113](http://www.instructables.com/id/A-Word-Clock/)에서 영감을 얻었습니다.

## 소스코드

### Go program which using Periph.io

Build for ARM64 SBCs:

    $ cd sbc_hangulclock
    $ GOARCH=arm64 GOOS=linux go build

Then, install binary and system service config by reference `deploy.sh`

### Arduino Sketch (prototype)

Need update submoduls HT1380 and LedControl to build the sketch;

    $ git submodule update

## 링크

- 블로그 글:
  - [한글시계 뒷 이야기](https://homin.dev/blog/p=493/)
  - [11년 만에 완성한, 원조, 한글시계](https://homin.dev/blog/post/20221009_hangulclock_is_re-written_in_golang/)
- [하드웨어 제작 사진](https://picasaweb.google.com/118040095502884745897/KoreanWordClockWithArduino#)
- [다른 한글시계들](http://hangulclocks.suapapa.net)

## 라이선스

### 5x5 한글 조합

<a rel="license" href="http://creativecommons.org/licenses/by/4.0/"><img alt="크리에이티브 커먼즈 라이선스" style="border-width:0" src="https://i.creativecommons.org/l/by/4.0/88x31.png" /></a><br />이 저작물은 <a rel="license" href="http://creativecommons.org/licenses/by/4.0/">크리에이티브 커먼즈 저작자표시 4.0 국제 라이선스</a>에 따라 이용할 수 있습니다.

### 소스코드 

아두이노 스케치와, 고 프로그램의 라이선스가 각각 다릅니다. 각각의 폴더에서 확인하세요.
- Arduino Sketch: [ardino/HangulClock/LICENSE](arduino/HangulClock/LICENSE)
- Go program: [sbc_hangulclock/LICENSE](sbc_hangulclock/LICENSE)