def solve(s):
    xs = list(range(len(s)+1))
    
    flag = True
    while flag:
        flag = False
        for i in range(len(s)):
            c = s[i]
            if (c == '>' and xs[i] < xs[i+1]) or (c == '<' and xs[i] > xs[i+1]):
                temp = xs[i]
                xs[i] = xs[i+1]
                xs[i+1] = temp
                flag = True
    
    for i in range(len(s)):
        print(xs[i],end="")
        print(s[i],end="")
    print(xs[-1])