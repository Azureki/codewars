def int32_to_ip(int32):
    mask = 255
    ip = []
    ip.append(mask&(int32>>24))
    ip.append(mask&(int32>>16))
    ip.append(mask&(int32>>8))
    ip.append(mask&int32)
    return '.'.join(map(str, ip))

print(int32_to_ip(2154959208))
