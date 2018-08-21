import socket
from datetime import datetime
from time import sleep

# https://qiita.com/msrks/items/0550603efc59f6e8ba09


def main():
    port = 5000
    
    with socket.socket() as s:
        s.bind(('', port))
        s.listen(1)
        conn, addr = s.accept()
    
        with conn:
            print('Connected by', addr)
            while True:
                data = conn.recv(1024)
                if not data: break
                conn.sendall(b"yes im ok")
        
    