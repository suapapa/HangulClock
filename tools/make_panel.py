#!/usr/bin/env python3

# make_panel.py - make silkscreen for panel of hangulclock
#
# Copyright (C) 2024 Homin Lee <ff4500@gmail.com>
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.

import os
from PIL import Image
from PIL import ImageFont, ImageDraw, ImageOps

def make_panel(
        charSquareSizeMM = 33,
        charMargin = 80,
        panelMarginMM = 2.5,
        drawXOffset = 0,
        drawYOffset = 0,
        DPI = 300,
        fontPath = r'./Hakgyoansim Byeolbichhaneul TTF B.ttf',
    ):
    panelString = '''
    열한다세네
    두여섯일곱
    여덟아홉시
    자정이삼십
    사오십오분
    '''
    t = ''.join(panelString.split())
    panelChars = list(t)

    cPix = int((DPI * charSquareSizeMM )/25.4) # 1 inch == 25.4 mm
    cSize = cPix*5
    panelMargin = 2.5
    panelMarginX = int((DPI * panelMarginMM )/25.4)
    panelMarginY = int((DPI * panelMarginMM )/25.4)
    panelSize = (cSize+panelMarginX*2, cSize+panelMarginY*2) # 236 == 20mm on 300dpi

    print(f"panelSize = {panelSize}")

    image = Image.new('RGB', panelSize)
    draw = ImageDraw.Draw(image)

    # To find proper fontsize fited-in given dimention
    fontSize = 0
    for i in range(12, 2000):
        font = ImageFont.truetype(fontPath, i, encoding="unic")
        text = "한"
        textSize = font.getlength(text)
        # if textSize[0] > cPix or textSize[1] > cPix:
        if textSize > cPix:
            print (f"Font size {textSize} {i}")
            fontSize = i - charMargin
            break

    font = ImageFont.truetype(fontPath, fontSize, encoding="unic")

    print (f"cPix = {cPix}")
    for y in range(5):
        for x in range(5):
            panelChar = panelChars[x+(y*5)]
            charSize = font.getlength(panelChar)
            xMargin = (cPix - charSize)/2
            yMargin = (cPix - charSize)/2
            #print panelChar.encode('utf-8'), charSize, xMargin, yMargin
            draw.text(
                (x*cPix+xMargin+panelMarginX+drawXOffset, y*cPix+yMargin+panelMarginY+drawYOffset),
                panelChar, font=font,
            )
            
    # mirror it to toner transfer
    image = ImageOps.mirror(image)
    image.save(f'panel_{os.path.basename(fontPath)}.png', dpi=(DPI, DPI))

if __name__ == '__main__':
    make_panel(drawYOffset=-30)

# vim: et sw=4 fenc=utf-8:
