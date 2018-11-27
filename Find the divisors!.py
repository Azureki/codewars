from math import sqrt

def divisors(integer):
    n = int(sqrt(integer))+1
    res=[]
    for i in range(2, n):
        if integer % i == 0:
            res.append(i)
            
    lng = len(res)
    for i in range(lng-1, -1, -1):
        res.append(integer//res[i])
        

    return res if len(res) else f'{integer} is prime' 

        

print(divisors(12))
