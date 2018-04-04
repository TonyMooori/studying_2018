import math
import operator as op
from functools import reduce

# REF: http://norvig.com/lispy.html

Symbol = str
Number = (int,float)
Atom = (Symbol,Number)
List = list
Exp = (Atom,List)

class Env(dict):
    def __init__(self,parms=(),args=(),outer=None):
        # print("ENV")
        # print(parms)
        # print(args)
        self.update(zip(parms,args))
        self.outer = outer
        # print(self)
    def find(self,var):
        if var in self:
            return self
        elif self.outer is None:
            raise ValueError("%s is not declared." % var)
        else:
            return self.outer.find(var)

class Procedure(object):
    def __init__(self,parms,body,env):
        self.parms = parms
        self.body = body
        self.env = env
    
    def __call__(self,*args):
        new_env = Env(self.parms,args,self.env)
        return evaluate(self.body,new_env)


def tokenize(chars:str) -> list:
    return chars.replace("("," ( ").replace(")"," ) ").split()

def parse(program : str) -> Exp:
    return read_from_tokens(tokenize(program))

def read_from_tokens(tokens:list) -> Exp:
    if len(tokens) == 0:
        raise SyntaxError("Unexpected EOF")
    token = tokens.pop(0)
    if token == '(':
        L = []
        while tokens[0] != ')':
            L.append(read_from_tokens(tokens))
        tokens.pop(0) # pop ')'
        return L
    elif token == ')':
        raise SyntaxError("Unexpected )")
    else:
        return atom(token)

def atom(token:str)->Atom:
    try:
        ret = int(token)
    except:
        try:
            ret = float(token)
        except:
            ret = Symbol(token)
    return ret

def standard_env() -> Env:
    env = Env()
    env.update(vars(math))
    env.update({
        '+':lambda *xs: reduce(lambda x,y:x+y,xs),
        '-':lambda *xs: reduce(lambda x,y:x-y,xs),
        '*':lambda *xs: reduce(lambda x,y:x*y,xs), 
        '/':lambda *xs: reduce(lambda x,y:x/y,xs),
        '>':op.gt, '<':op.lt, '>=':op.ge, '<=':op.le, '=':op.eq, 
        'abs':     abs,
        'append':  op.add,  
        'apply':   lambda proc, args: proc(*args),
        'begin':   lambda *x: x[-1],
        'first':     lambda x: x[0],
        'rest':     lambda x: x[1:], 
        'cons':    lambda x,y: [x] + y,
        'eq?':     op.is_, 
        'pow':    pow,
        'equal?':  op.eq, 
        'length':  len, 
        'list':    lambda *x: List(x), 
        'list?':   lambda x: isinstance(x, List), 
        'map':     lambda f,xs: list(map(f,xs)),
        'max':     max,
        'min':     min,
        'not':     op.not_,
        'empty?':   lambda x: x == [], 
        'number?': lambda x: isinstance(x, Number),  
		'print':   print,
        'procedure?': callable,
        'round':   round,
        'symbol?': lambda x: isinstance(x, Symbol),
        'int' : int,
        'float' : float,
    })
    return env

global_env = standard_env()

def evaluate(x:Exp,env=global_env) -> Exp:
    # print("evaluate: %s"  % x)
    # print("env=" )
    # print(env)

    if isinstance(x,Symbol):
        return env.find(x)[x]
    elif not isinstance(x,List):
        return x
    
    op,*args = x

    if op == "quote":
        return args[0]
    elif op == "if":
        _,test,coseq,alt = x
        exp = (coseq if evaluate(test,env) else alt)
        return evaluate(exp,env)
    elif op == "do":
        last = None
        for v in x[1:]:
            last = evaluate(v,env)
        return last
    elif op == "def":
        _,symbol,exp = x
        env[symbol] = evaluate(exp,env)
    elif op == "fn":
        parms,body = args
        return Procedure(parms, body, env)
    else:
        func = evaluate(op,env)
        args = [evaluate(arg,env) for arg in x[1:]]
        return func(*args)


def main():
    with open("lib.lisp","r") as f:
        lines = f.readlines()
    code = "".join(lines)
    evaluate(parse(code))

    while True:
        code = ""
        while code == "" or code.count("(") > code.count(")"):
            code += input(">>" if code == "" else "")
        
        if code =="exit":
            break
        else:
            val = evaluate(parse(code))
        
        if val is not None:
            print(val)

if __name__=="__main__":
    main()
