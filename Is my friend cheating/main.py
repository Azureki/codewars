"""
SUM=n^n/2+n/2
SUM-a-b=a*b
SUM=a*b+a+b=(a+1)(b+1)-1
SUM+1=(a+1)(b+1)
"""
import math
def removNb(n):
    NUM = (n*n+n)//2+1
    left = [(i,NUM//i) for i in range(2,math.sqrt(NUM)) if NUM%i==0 and NUM//i<n]
    right = [(j,i) for i,j in left]
    return left+right
