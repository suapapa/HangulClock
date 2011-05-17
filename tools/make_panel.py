#!/usr/bin/python
# -*- coding: utf-8 -*-
 
# make_panel.py - make silkscreen for panel of hangulclock 
#
# Copyright (C) 2011 Homin Lee <ff4500@gmail.com>
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.

import Image
import ImageFont, ImageDraw
import os

panelString = '''
열한다세네
두여섯일곱
여덟아홉시
자정이삼십
사오십오분
'''
t = ''.join(panelString.split())
panelChars = list(t.decode('utf-8'))

DPI = 300
cSize = 20 # 20mm for W & H of one character

cPix = int((DPI * 20 )/25.4) # 1 inch == 25.4 mm
panelSize = (cPix*5, cPix*5) # 236 == 20mm on 300dpi

image = Image.new('RGB', panelSize)
draw = ImageDraw.Draw(image)

# The fonts from http://hangeul.naver.com
# I used it via ttf-nanum package of Ubuntu linux    
fontPath = r'/usr/share/fonts/truetype/nanum/NanumPen.ttf'    
#fontPath = r'/usr/share/fonts/truetype/nanum/NanumBrush.ttf'

'''
# To find proper fontsize fited-in given dimention
for i in range(32, 2000):
    font = ImageFont.truetype(fontPath, i, encoding="unic")
    text = "한".decode('utf-8')
    textSize = font.getsize(text)
    if textSize[0] > 236 or textSize[1] > 236:
        print textSize, i
        break
'''
font = ImageFont.truetype(fontPath, 205, encoding="unic")

for y in range(5):
    for x in range(5):
        panelChar = panelChars[x+(y*5)]
        charSize = font.getsize(panelChar)
        xMargin = (236 - charSize[0])/2 
        yMargin = (236 - charSize[1])/2 
        #print panelChar.encode('utf-8'), charSize, xMargin, yMargin
        draw.text((x*236+xMargin, y*236+yMargin), panelChar, font=font)

image.save('panel_%s.png'%os.path.basename(fontPath), dpi=(DPI, DPI))

# vim: et sw=4 fenc=utf-8:
