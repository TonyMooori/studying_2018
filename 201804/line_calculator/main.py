import re
import math
from inspect import signature
import sys
import traceback

LEFT = True
RIGHT = False

# トークンに分割する際に使う正規表現関係
REGEX_PATTERN = r'[\s]*(\d+\.\d+|\d+|\w+|.)'
SPLITTER = re.compile(REGEX_PATTERN)

# 関数一覧
FUNCTIONS = {
    "sin" : lambda x: math.sin(x),
    "cos" : lambda x: math.cos(x),
    "pow" : lambda x,y: math.pow(x,y),
}

# 変数一覧
VARIABLES = {
    "pi" : math.pi,
}

# 演算子一覧
# 順に優先順位，右結合性か左結合せいか，関数
# =については環境を変化させるのでNoneとなっている
OPERATORS = {
    "^" : (4,RIGHT,lambda x,y:x**y),
    "*" : (3,LEFT ,lambda x,y:x*y),
    "/" : (3,LEFT ,lambda x,y:x/y),
    "+" : (2,LEFT ,lambda x,y:x+y),
    "-" : (2,LEFT ,lambda x,y:x-y),
    "=" : (1,RIGHT,None)
}

def tokenize(line):
    """
    トークナイザ．コードをトークンに分割する．
    """
    return SPLITTER.findall(line)

def try_parse(token):
    """
    トークンが数字ならfloat型に，そうでないならstrのまま返す
    """
    try:
        return float(token)
    except:
        return token

def parse(toknes):
    """
    中置記法で表されたtokensを逆ポーランド記法に変換する
    REF: https://ja.wikipedia.org/wiki/操車場アルゴリズム
    """

    # 逆ポーランド記法となったトークン列
    ast = []
    # 演算子を一時的に保存するスタック
    park = []

    for token in toknes:
        token = try_parse(token)
            
        # print("i am reading %s, park = %s" % (str(token),str(park)))
        if isinstance(token,float):
            ast.append(token)
        elif token in FUNCTIONS:
            park.append(token)
        elif token == ",":
            # コンマが出たら左括弧が出るまでpop
            while True:
                if len(park) == 0:
                    raise RuntimeError("The balance of parenthesis is incorrect")
                if park[-1] == "(":
                    break
                ast.append(park.pop())
        elif token in OPERATORS:
            # 演算子は条件を満たす限りpopし続ける
            # 詳細はWikipedia参照
            while len(park)!=0:
                if park[-1] not in OPERATORS:
                    break
                
                op1 = OPERATORS[token]
                op2 = OPERATORS[park[-1]]
                if op1[1] == LEFT and op1[0] <= op2[0]:
                    ast.append(park.pop())  
                elif op1[0] < op2[0]:
                    ast.append(park.pop())  
                else:
                    break
            park.append(token)
        elif token == "(":
            # 左括弧はとりあえずpush
            park.append(token)
        elif token == ")":
            # 左括弧が出るまでpop
            while True:
                if len(park)==0:
                    raise RuntimeError("The balance of parenthesis is incorrect.")
                if park[-1] == "(":
                    park.pop()
                    break
                elif park[-1] in OPERATORS:
                    ast.append(park.pop())
                else:
                    raise RuntimeError("Found unknown operator")
            
            # 関数があったら1つだけpop
            if len(park) != 0 and park[-1] in FUNCTIONS:
                ast.append(park.pop())
            pass
        else:
            # 多分変数
            ast.append(token)

    # 残った演算子をpop
    while len(park) != 0:
        if park[-1] in ["(",")"]:
            raise RuntimeError("The balance of parenthesis is incorrect!")

        if park[-1] in OPERATORS:
            ast.append(park.pop())
        else:
            raise RuntimeError("Found unknown operator")

    return ast

def find_value(varname):
    """
    変数名varnameに代入された値を取り出す
    """
    if varname in VARIABLES:
        return VARIABLES[varname]
    else:
        raise RuntimeError("The variable '%s' is not found" % varname)


def to_values(xs):
    """
    変数名が含まれるリストxsを数字のみのリストに変換する
    """
    ret = []
    for x in xs:
        if isinstance(x,float):
            y = x
        else:
            y = find_value(x)
        ret.append(y)
    return ret

def evaluate(ast):
    """
    逆ポーランド記法で表されたトークン列astを評価する
    """

    # スタック
    park = []
    
    for token in ast:
        # print("i am reading %s, park = %s" % (str(token),str(park)))
        if token == "=":
            # '='についてはスタックから代入する値と，変数名を取り出す
            if len(park) < 2:
                raise RuntimeError("The usage of '=' is incorrect")
            value = to_values([park.pop()])
            varname = park.pop()

            # 変数名が数字だったり関数名とかぶってない場合，代入する
            if isinstance(varname,float) or varname in FUNCTIONS \
                    or varname in OPERATORS:
                raise RuntimeError("Cannot assign value to %s" % varname)

            VARIABLES[varname] = value[0]
            park.append(varname)
        elif token in FUNCTIONS or token in OPERATORS:
            # 関数や演算子が来た場合
            f = FUNCTIONS[token] if token in FUNCTIONS else OPERATORS[token][2]

            # 関数の引数の数を取得し，その数だけスタックから値を取り出す
            n_param = len(signature(f).parameters)
            if n_param > len(park):
                if token in FUNCTIONS:
                    raise RuntimeError("The number of argument is incorrect")
                else:
                    raise RuntimeError("The usage of operator is incorrect")
            xs = park[len(park)-n_param:]
            park = park[:len(park)-n_param]

            # 変数名を変数に代入されている値に変換し，関数を評価する
            park.append(f(*to_values(xs)))
        else:
            park.append(token)

    if len(park) == 0:
        raise RuntimeError("The result is none.")
    if isinstance(park[-1],float):
        return park[-1]
    else:
        return find_value(park[-1])

def repl(line):
    tokens = tokenize(line)
    ast = parse(tokens)
    val = evaluate(ast)
    # print(line)
    # print(tokens)
    # print(ast)
    # print(val)

    return val

def main():
    print("Hello, calculator!")
    while True:
        line = input(">> ")

        if line =="exit":
            break
        try:
            print(repl(line))
        except:
            _, ms, tb = sys.exc_info()
            print("Error: %s" % ms)
            traceback.print_tb(tb)
    print("Bye!")

if __name__=="__main__":
    main()
