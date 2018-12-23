# xs = [100, 76, 56, 44, 89, 73, 68, 56, 64, 123, 2333, 144, 50, 132, 123, 34, 89]
# Test.assert_equals(choose_best_sum(230, 4, xs), 230)
# Test.assert_equals(choose_best_sum(430, 5, xs), 430)
# Test.assert_equals(choose_best_sum(430, 8, xs), None)

# K-sum problem 
# O(n^(k-1)logn)?

from itertools import combinations

def choose_best_sum(t, k, ls):
    best_sum = 0
    best_set = None
    combs = combinations(ls, k)
    for combination in combs:
        combi_sum = sum(combination)
        if combi_sum <= t and combi_sum > best_sum:
             best_set = combination
             best_sum = combi_sum
    # return (best_set, best_sum)
    return best_sum if best_sum else None

xs = [100, 76, 56, 44, 89, 73, 68, 56, 64, 123, 2333, 144, 50, 132, 123, 34, 89]
xs = [item for item in xs if item < 430]
print(choose_best_sum(230, 4, xs))#, 230)
choose_best_sum(430, 5, xs)#, 430)
print(choose_best_sum(430, 8, xs))#, None)

# ===================
# Solution of
# betegelse, mastersago, AndreiBamboi

from itertools import combinations

def choose_best_sum(t, k, ls):
    return max((s for s in (sum(dists) for dists in combinations(ls, k)) if s <= t), default=None)
