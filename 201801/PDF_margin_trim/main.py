#coding:utf-8
import subprocess
import os
from PIL import Image
import glob

def to_images(filename,folder):
    """
    ImageMagickを呼び出してPDFを画像に変換する関数
    temp_folder内にtemp-[番号].pngとして保存する
    
    filename:   対象のPDFファイル
    folder:     保存先フォルダ
    """
    png_path = folder + "temp.png"
    
    # うちの環境ではconvertではなくmagic convert でやる
    subprocess.call([
        "magick","convert", 
        "-density", "300", 
        filename,
        "-resize","1240x1754",
        png_path
    ])

def cut_margin_all(folder,margins):
    """
    指定したフォルダのtemp-*.pngという画像のマージンを削除する関数
    
    folder:  この中にあるtemp-*.pngという画像についてマージンを削除する
    margins: 上下左右のマージンのピクセル数． [left, upper right, lower]
    """
    files = glob.glob(folder + "temp-*.png")
    
    for f in files:
        cut_margin_img(f,margins)

def cut_margin_img(f,margins):
    """
    指定した画像のマージンを削除する関数
    
    f:       操作対象の画像
    margins: 上下左右のマージンのピクセル数． [left, upper right, lower]
    """
    img = Image.open(f)
    w,h = img.size
    
    # 切り抜き(left, upper right, lower)
    img.crop((margins[0],margins[1],w-margins[2],h-margins[3])).save(f)
    
def to_pdf(filename,folder):
    """
    folder内にあるtemp-*.pngをPDFに変換する関数
    
    filename:   出力先(PDF)
    folder:     操作対象の画像が入ったフォルダ
    """
    
    # 2桁になるとソートが面倒なので
    n = len(glob.glob(folder + "temp-*.png"))
    imgs = [folder + "temp-%d.png" % i for i in range(n)]
    
    cmd = ["magick","convert"] + imgs + [filename]
    subprocess.call(cmd)

if __name__=="__main__":
    # tempフォルダ
    temp_folder = ".\\temp\\"
    if not os.path.exists(temp_folder):
        os.mkdir(temp_folder)
    
    # test.pdf
    print("converting pdf to images...")
    to_images("test.pdf",temp_folder)
    print("cutting margins...")
    cut_margin_all(temp_folder,[100,100,100,100])
    print("converting images to pdf...")
    to_pdf("out.pdf",temp_folder)
    
    """
    # マージンの調整はこれで
    cut_margin_img(f,margins)
    """
    
"""
# https://qiita.com/gyu-don/items/329de5e9b7f7d345dc90
import sys
import os.path
from contextlib import closing

from PyQt4 import QtCore
from popplerqt4 import Poppler

FORMAT = "PNG"
EXT = ".png"

def dump_image(path):
    doc = Poppler.Document.load(path)
    doc.setRenderHint(Poppler.Document.TextAntialiasing)
    filename_fmt = os.path.splitext(path)[0]+"_{0}" + EXT
    for n,page in ((i+1,doc.page(i)) for i in range(doc.numPages())):
        img = page.renderToImage()
        img.save(filename_fmt.format(n),FORMAT)

if __name__ == "__main__":
    app = QtCore.QCoreApplication(sys.argv)
    if len(sys.argv) != 2:
        print("Usage: {0} pdf_path".format(sys.argv[0]))
    else:
        dump_image(sys.argv[1])
    
    """