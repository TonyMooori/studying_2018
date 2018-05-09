import numpy as np
from numba import jit
import random

N_MAX_STEP = 500
N_GEN = 64

@jit
def emulate(gen,arr,display=False):
    pc = 0
    memory = [0] * 16
    i = 0
    
    if display:
        print("gen=%s" % str(gen))
    
    while i < N_MAX_STEP:
        pc = pc % len(gen)
        cmd = gen[pc]
        opcode,imm = cmd // 16 , cmd % 16
        x0 = (memory[imm] + 0 )% len(arr)
        x1 = (memory[imm] + 1 )% len(arr)
        
        if display:
            print("step %d" % i)
            print("gen[%04d] = %d" % (pc,gen[pc]))
            print("opcode,imm = %d, %d" % (opcode,imm))
            print("x0,x1 = %d, %d" % (x0,x1))
            print("memory = %s" % str(memory))
            print("array  = %s" % str(arr))
        
        if opcode == 0:
            arr[x0],arr[x1]=arr[x1],arr[x0]
        elif opcode == 1:
            if arr[x0]>arr[x1]:
                pc = pc + 20
        elif opcode == 3:
            memory[imm%8] = 0
        elif opcode == 4:
            memory[imm%8] += 1
        elif opcode == 5:
            memory[imm%8] -= 1
        elif opcode == 6:
            memory[imm%8] += imm // 8
        elif opcode == 7:
            if arr[x0]<arr[x1]:
                pc = pc + 20
        elif opcode == 8:
            if arr[imm%8]==arr[imm//8]:
                pc = pc + 20
        elif opcode == 9:
            memory[imm%8] += memory[imm//8]
        elif opcode == 10:
            pc = pc + 10
        elif opcode == 11:
            pc = pc - 10
        elif opcode == 12:
            break
        else:
            pass
        i+=1
        pc = pc + 1
    
    return 100 * evaluate(arr) #+ (i/N_MAX_STEP)

@jit
def evaluate(arr):
    n_bad = 0
    for i in range(len(arr)-1):
        if arr[i] > arr[i+1]:
            n_bad += 1
    return n_bad / len(arr)

def main():
    gens = (np.random.random_sample( (N_GEN,256) ) * 256).astype(np.int)
    
    for generation in range(10000000):
        print("generation = %d" % generation)
        
        point = [0] * len(gens)
        arr = list(range(32))[::-1]
        
        for i in range(10):
            #print("----- %dth test -----" % i)
            random.shuffle(arr)
            for i,gen in enumerate(gens):
                #print("** %dth gen **" % i)
                point[i] += emulate(gen,arr.copy(),False)
        
        best = np.argmin(point)
        #emulate(gens[best],np.random.random_sample(32),False)
        print("best = %d"%best)
        print("score = %lf" % point[best])
        print("gen = %s" % str(gens[best]))
        gens = generate_gens(gens[best])

@jit
def generate_gens(gen):
    ret = []
    ret.append(gen)
    for i in range(N_GEN - 1):
        gen2 = []
        for j in range(len(gen)):
            cmd = int(np.random.random_sample() * 50)
            
            if cmd == 0:
                pass
            elif cmd == 1:
                gen2.append(int(np.random.random_sample() * 256))
            elif cmd == 2:
                gen2.append(int(np.random.random_sample() * 256))
                gen2.append(gen[j])
            elif cmd == 3:
                gen2.append(gen[j])
                gen2.append(gen[j])
            elif cmd == 4:
                pass
            else:
                gen2.append(gen[j])
        
        ret.append(gen2)
    return ret

if __name__=="__main__":
    main()

