#coding:utf-8
import subprocess
import os
from PIL import Image, ImageOps
import glob

def to_images(filename,folder,density=300):
    """
    ImageMagickを呼び出してPDFを画像に変換する関数
    temp_folder内にtemp-[番号].pngとして保存する
    
    filename:   対象のPDFファイル
    folder:     保存先フォルダ
    """
    png_path = folder + "temp.png"
    
    # うちの環境ではconvertではなくmagick convert でやる
    subprocess.call([
        "magick","convert",
        "-monitor",
        "-density",str(density),
        filename,
        #"-resize","1240x1754",
        png_path
    ])

def cut_margin_all(folder,margins,inverse=False,gray=False):
    """
    指定したフォルダのtemp-*.pngという画像のマージンを削除する関数
    
    folder:  この中にあるtemp-*.pngという画像についてマージンを削除する
    margins: 上下左右のマージンのピクセル数． [left, upper right, lower]
    inverse: 色反転するならtrue
    gray: グレーにするならtrue
    """
    files = glob.glob(folder + "temp-*.png")
    
    for f in files:
        cut_margin_img(f,margins,inverse,gray)

def cut_margin_img(f,margins,inverse,gray):
    """
    指定した画像のマージンを削除する関数
    
    f:       操作対象の画像
    margins: 上下左右のマージンのピクセル数． [left, upper right, lower]
    inverse: 色反転するならtrue
    gray: グレーにするならtrue
    """
    img = Image.open(f)
    w,h = img.size
    
    # 切り抜き(left, upper right, lower)
    img = img.crop((margins[0],margins[1],w-margins[2],h-margins[3])).convert('RGB')
    if inverse:
        img = ImageOps.invert(img)
        
    if gray:
        img = ImageOps.grayscale(img)
    img.save(f)
    
def to_pdf(filename,folder):
    """
    folder内にあるtemp-*.pngをPDFに変換する関数
    
    filename:   出力先(PDF)
    folder:     操作対象の画像が入ったフォルダ
    """
    
    # 2桁になるとソートが面倒なので
    n = len(glob.glob(folder + "temp-*.png"))
    imgs = [folder + "temp-%d.png" % i for i in range(n)]
    
    cmd = ["magick","convert","-monitor"] + imgs + [filename]
    subprocess.call(cmd)

if __name__=="__main__":
    # tempフォルダ
    temp_folder = ".\\temp\\"
    if not os.path.exists(temp_folder):
        os.mkdir(temp_folder)
    
    # test.pdf
    print("converting pdf to images...")
    to_images("test.pdf",temp_folder,150)
    print("cutting margins...")
    cut_margin_all(temp_folder,[100,100,100,100],True,True)
    print("converting images to pdf...")
    to_pdf("out.pdf",temp_folder)
    
    """
    # マージンの調整はこれで
    cut_margin_img(f,margins)
    """

