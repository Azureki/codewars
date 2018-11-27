def is_isogram(string):
    if not len(string):
        return True
    d={}
    
    for s in string:
        s=s.lower()
        if d.get(s):
            return False
        else:
            d[s]=1
    return True


string="aba"
print(is_isogram(string))

