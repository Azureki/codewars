def cakes(recipe, available):
    # return min(available.get(k, 0)/recipe[k] for k in recipe)
    return min((0 if not available.get(k) else available[k] // recipe[k])
               for k in recipe)
