def pext(bits, mask):
    out = 0
    bit = 1
    while mask != 0:
        b = mask & (-mask)
        if (bits & b) != 0:
            out |= bit
        mask ^= b
        bit <<= 1

    return out

def pdep(bits, mask):
    out = 0
    bit = 1
    while bits != 0:
        if (bits & bit) != 0:
            out |= mask & (-mask)
        bits ^= bit
        bit <<= 1

    return out

def set_bit(x, b):
    return x | (1<<b)

def pack_xy(x,y):
    return (y<<3) + x

def gen_masks_xy(x,y):
    #horizontals
    h = 0
    for i in range(8):
        h = set_bit(h, pack_xy(i, y));


    #verticals
    v = 0
    for i in range(8):
        v = set_bit(v, pack_xy(x, i));


    #main diagonal
    m = 0
    for i in range(-min(x,y), min(8-x,8-y)):
        m = set_bit(m, pack_xy(x+i, y+i));

    #alt diagonal
    a = 0
    for i in range(-min(x,7-y), min(8-x,y+1)):
        a = set_bit(a, pack_xy(x+i, y-i));

    return [h,v,m,a]

def gen_masks():
    masks = [];
    for y in range(8):
        for x in range(8):
            masks += [gen_masks_xy(x,y)]
    return masks

masks = gen_masks()

def print_masks():
    for i in range(64):
        print "[" + ', '.join(["0x{:016x}".format(x) for x in masks[i]]) + "],"

def get_outflank(b, n):
    i = n-1
    m = 0
    while i >= 0 and ((1<<i) & b) != 0:
        i -= 1
    if i >= 0 and i != n-1:
        m |= 1<<i
    i = n+1
    while i < 8 and ((1<<i) & b) != 0:
        i += 1
    if i < 8 and i != n+1:
        m |= 1<<i
    return m


def gen_outflank(n):
    lut = []
    for b in range(64):
        lut += [get_outflank(b<<1, n)]

    return lut


def get_flip(b, n):
    m = 0
    i = n
    tm = 0
    while i >= 0 and ((1<<i) & b) == 0:
        tm |= 1<<i
        i -= 1
    if i >= 0 and i != n:
        m |= tm

    i = n
    tm = 0
    while i < 8 and ((1<<i) & b) == 0:
        tm |= 1<<i
        i += 1

    if i < 8 and i != n:
        m |= tm

    return m

def gen_flip(n):
    lut = []
    for b in range(256):
        lut += [get_flip(b, n)]

    return lut

outflank = [gen_outflank(i) for i in range(8)]
flip = [gen_flip(i) for i in range(8)]

def print_lut(lut):
    print '[' + ',\n'.join(
        ['[' + ',\n'.join([
            ', '.join(
                ["0x{:02x}".format(x[i+j]) for j in range(16)]
            ) for i in range(0,len(x),16)
        ]) + ']' for x in lut]
    ) + ']'

#print_lut(flip)