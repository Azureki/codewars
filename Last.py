from collections import Iterable
def last(*lst):
  if len(lst)==1:
    lst=lst[0]
  if isinstance(lst,Iterable):
    return lst[-1]
  else:
    return lst

# bionikspoon, crimsujii, acjoker, Lion Noir
# def last(*args):
#     return args[-1] if not hasattr(args[-1], "__getitem__") else args[-1][-1]
