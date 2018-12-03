import re


def count_smileys(arr):
    res=0
    pattern=r'[:;]([-~])?[)D]'
    pattern=re.compile(pattern)
    for s in arr:
        if pattern.match(s):
            res+=1
    return res

arr=[':D',':~)',';~D',':)']
print(count_smileys(arr))

