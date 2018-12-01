def tower_builder(n_floors):
    res=[]
    for i in range(n_floors):
        num=n_floors-i-1
        res.append(' '*num+'*'*(2*i+1)+' '*num)
    return res


res=tower_builder(6)
print()
for ln in res:
    print('#'+ln+'#',len(ln))
