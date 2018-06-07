class Lexer:
    def __init__(self,xs):
        self.xs = xs
        self.idx = 0

    def peek(self):
        if self.idx < len(self.xs):
            return self.xs[self.idx]
        else:
            return None
            
    def next(self):
        val = self.peek()
        
        if not val is None:
            self.idx += 1
        
        return val

def try_parse(x):
    try:
        return int(x)
    except:
        return None

class Codegen:
    def __init__(self):
        self.pos = 0
        self.code = ""
        self.vars = {
            "__zero" :0, # always zero
            "__temp1":1,
            "__temp2":2,
            "__temp3":3,
            "__temp4":4,
            "__temp5":5,
        }
    
    def eval(self,ast):
        if len(ast) == 0:
            return
        
        first = ast[0]
        rest = ast[1:]
        
        if isinstance(first[0],list):
            if "if" == first[0][0]:
                self.eval_if(first)
            else:
                self.eval_while(first)
        else:
            if first[0] == "set":
                self.eval_set(first)
            elif first[0] == "add":
                self.eval_add(first)
            elif first[0] == "sub":
                self.eval_sub(first)
            elif first[0] == "getc":
                self.eval_getc(first)
            elif first[0] == "putc":
                self.eval_putc(first)
            elif first[0] == "copy":
                self.eval_copy(first)
            elif first[0] == "mul":
                self.eval_mul(first)
        self.eval(rest)
    
    def eval_mul(self,ast):
        # use temp1,2,3,4
        _,to,fr = ast
        val = try_parse(fr)
        if val is None: 
            new_ast = [
                ["set","__temp2",fr],
                ["sub","__temp2","1"],
                ["set","__temp3",to],
                [
                    ["while","__temp2"],
                    ["add",to,"__temp3"],
                    ["sub","__temp2","1"],
                ],
            ]
            self.eval(new_ast)
        else:
            self.eval([
                ["set","__temp4",fr],
                ["mul",to,"__temp4"],
            ])
    
    def eval_copy(self,ast):
        _,to,fr = ast
        new_ast = [
            ["set",to,"0"],
            ["set","__temp1","0"],
            [
                ["while",fr],
                ["add",to,"1"],
                ["add","__temp1","1"],
                ["sub",fr,"1"],
            ],
            [
                ["while","__temp1"],
                ["add",fr,"1"],
                ["sub","__temp1","1"],
            ],
        ]
        self.eval(new_ast)
    
    def eval_if(self,ast):
        _,varname = ast[0]
        rest = ast[1:]
        
        self.move(varname)
        self.add_code("[")
        self.eval(rest)
        self.move("__zero")
        self.add_code("]")
        
        
    def eval_while(self,ast):
        _,varname = ast[0]
        rest = ast[1:]
        
        self.move(varname)
        self.add_code("[")
        self.eval(rest)
        self.move(varname)
        self.add_code("]")
        
    
    def eval_putc(self,ast):
        _,varname = ast
        val = try_parse(varname)
        
        if val is None:
            self.putc(varname)
        else:
            self.assign_val("__temp1",val)
            self.putc("__temp1")
            
    
    def eval_getc(self,ast):
        _,varname = ast
        self.getc(varname)
    
    def eval_sub(self,ast):
        _,to,fr = ast
        val = try_parse(fr)
        if val is None: 
            if to == fr:
                new_ast = ["set",to,"0"]
            else:
                new_ast = [
                    ["set","__temp1","0"],
                    [
                        ["while",fr],
                        ["sub",to,"1"],
                        ["add","__temp1","1"],
                        ["sub",fr,"1"],
                    ],
                    [
                        ["while","__temp1"],
                        ["add",fr,"1"],
                        ["sub","__temp1","1"],
                    ],
                ]
                self.eval(new_ast)
        else:
            self.sub_val(to,val)
    
    def eval_add(self,ast):
        # use temp1
        _,to,fr = ast
        val = try_parse(fr)
        if val is None: 
            if to == fr:
                new_ast = [
                    ["set","__temp1","0"],
                    [
                        ["while",fr],
                        ["add","__temp1","1"],
                        ["sub",fr,"1"],
                    ],
                    [
                        ["while","__temp1"],
                        ["add",fr,"2"],
                        ["sub","__temp1","1"],
                    ],
                ]
            else:
                new_ast = [
                    ["set","__temp1","0"],
                    [
                        ["while",fr],
                        ["add",to,"1"],
                        ["add","__temp1","1"],
                        ["sub",fr,"1"],
                    ],
                    [
                        ["while","__temp1"],
                        ["add",fr,"1"],
                        ["sub","__temp1","1"],
                    ],
                ]
            self.eval(new_ast)
        else:
            self.add_val(to,val)
    
    def eval_set(self,ast):
        _,varname,fr = ast
        val = try_parse(fr)
        if val is None:
            self.eval_copy(["copy",varname,fr])
        else:
            self.assign_val(varname,val)

        
    def move(self,varname):
        if varname.isdigit():
            raise Exception("Digit is can't be varname")
        
        if not varname in self.vars:
            self.vars[varname] = len(self.vars)
        
        n = self.vars[varname]
        
        dx = n - self.pos
        
        if dx > 0:
            self.add_code(">"*dx)
        else:
            dx *= -1
            self.add_code("<"*dx)
        self.pos = n
    
    def add_code(self,s):
        self.code += s
    
    def assign_zero(self,varname):
        self.move(varname)
        self.add_code("[-]")
    
    def assign_val(self,varname,n):
        self.move(varname)
        self.assign_zero(varname)
        self.add_val(varname,n)
    
    def getc(self,varname):
        self.move(varname)
        self.add_code(",")
    
    def putc(self,varname):
        self.move(varname)
        self.add_code(".")
        
    def add_val(self,varname,n):
        self.move(varname)
        n = n % 256
        self.add_code("+"*n)
    
    def sub_val(self,varname,n):
        self.move(varname)
        n = n % 256
        self.add_code("-"*n)
        
def parse(lexer):
    xs = []
    while lexer.peek() is not None:
        xs.append(inner_parser(lexer))
    return xs

def inner_parser(lexer):
    types = {
        "set"   :3,
        "add"   :3,
        "sub"   :3,
        "getc"  :2,
        "putc"  :2,
        "if"    :2,
        "while" :2,
        "end"   :1,
        "copy"  :3,
        "mul"   :3,
    }

    tokens = lexer.next()
    
    if tokens[0] in types:
        n_arg = types[tokens[0]]
        raise_length(tokens,n_arg )
    else:
        print(tokens)
        raise Exception("unknown token %s" % tokens[0])
    
    if tokens[0] == "if" or tokens[0] == "while":
        ret = [tokens]
        while True:
            if lexer.peek() is None:
                raise Exception("Unexpected eof while reading")
            if lexer.peek()[0] == "end":
                break
            ret.append(inner_parser(lexer))
        lexer.next() # read end
    else:
        ret = tokens

    return ret

def raise_length(tokens,n):
    if len(tokens) != n:
        raise Exception("The lenth of %s 's argument must be %d, we got %d" % (tokens[0],n-1,len(tokens)-1))
    else:
        pass

def tokenize(code):
    lines = code.split("\n")
    ret = []
    for line in lines:
        line = remove_comment(line)
        tokens = line.split()
        if len(tokens) != 0:
            ret.append(tokens)
    return Lexer(ret)

def remove_comment(line):
    idx = line.find(";")
    if idx < 0:
        return line
    else:
        return line[:idx]

def compile(code):
    lexer = tokenize(code)
    ast = parse(lexer)
    c = Codegen()
    c.eval(ast)
    return c.code

def read_code(path):
    with open(path,"r") as f:
        lines = f.readlines()
    return "".join(lines)

def test():
    return compile(read_code("main.rbf"))

