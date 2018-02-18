# EEPROM 24FC1025-I/P利用メモ


## 基礎情報

[秋月電子販売サイト １ＭｂｉｔＩスケアＣシリアルＥＥＰＲＯＭ　２４ＦＣ１０２５－Ｉ／Ｐ](http://akizukidenshi.com/catalog/g/gI-03570/)  
[データシート en](http://akizukidenshi.com/download/24fc1025-ip.pdf)  
[データシート ja](http://akizukidenshi.com/download/24fc1025-ip.pdf)  
[I2C EEPROM 読み込み/書き出しツール for Arduino](http://www.geocities.jp/bokunimowakaru/diy/arduino/eeprom.html)  

## 追加情報

1 Mbit利用可能！とか書いてあるのにアドレス空間は16bit(256 kbit)なのなんなのみたいなお気持ちになっていたらA0,A1ピンのハイローでチップを変更できるということがわかった．  
I2Cなんだから全部繋いでおけばいいじゃないかと思いながらA0,A1をdigitalWriteとかで操作してうまいことやるプログラムを書いたら一応動作した．

……のだけどGitHubにあげようとしてプログラムを整理してたらI2CScannerすらまともに動作しなくなってEEPROMが悪いのかプルアップ抵抗の抵抗値が悪いのか何なのかわからんので放置．




