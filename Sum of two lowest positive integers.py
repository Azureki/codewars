def sum_two_smallest_numbers(numbers):
    mask1=mask2=float('inf')
    for n in numbers:
        if n <= mask1:
            mask2=mask1
            mask1=n
        elif n < mask2:
            mask2=n
    return mask1+mask2

numbers=[25, 42, 12, 18, 22]
print(sum_two_smallest_numbers(numbers))
