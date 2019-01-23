def zeros(n):
    ans, i = 0, 5
    while i <= n:
        ans += n // i
        i *= 5
    return ans

# 可以递归
