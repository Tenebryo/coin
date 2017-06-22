const LINES : [[u64; 4]; 64] = [
    [0x00000000000000ff, 0x0101010101010101, 0x8040201008040201, 0x0000000000000001],
    [0x00000000000000ff, 0x0202020202020202, 0x0080402010080402, 0x0000000000000102],
    [0x00000000000000ff, 0x0404040404040404, 0x0000804020100804, 0x0000000000010204],
    [0x00000000000000ff, 0x0808080808080808, 0x0000008040201008, 0x0000000001020408],
    [0x00000000000000ff, 0x1010101010101010, 0x0000000080402010, 0x0000000102040810],
    [0x00000000000000ff, 0x2020202020202020, 0x0000000000804020, 0x0000010204081020],
    [0x00000000000000ff, 0x4040404040404040, 0x0000000000008040, 0x0001020408102040],
    [0x00000000000000ff, 0x8080808080808080, 0x0000000000000080, 0x0102040810204080],
    [0x000000000000ff00, 0x0101010101010101, 0x4020100804020100, 0x0000000000000102],
    [0x000000000000ff00, 0x0202020202020202, 0x8040201008040201, 0x0000000000010204],
    [0x000000000000ff00, 0x0404040404040404, 0x0080402010080402, 0x0000000001020408],
    [0x000000000000ff00, 0x0808080808080808, 0x0000804020100804, 0x0000000102040810],
    [0x000000000000ff00, 0x1010101010101010, 0x0000008040201008, 0x0000010204081020],
    [0x000000000000ff00, 0x2020202020202020, 0x0000000080402010, 0x0001020408102040],
    [0x000000000000ff00, 0x4040404040404040, 0x0000000000804020, 0x0102040810204080],
    [0x000000000000ff00, 0x8080808080808080, 0x0000000000008040, 0x0204081020408000],
    [0x0000000000ff0000, 0x0101010101010101, 0x2010080402010000, 0x0000000000010204],
    [0x0000000000ff0000, 0x0202020202020202, 0x4020100804020100, 0x0000000001020408],
    [0x0000000000ff0000, 0x0404040404040404, 0x8040201008040201, 0x0000000102040810],
    [0x0000000000ff0000, 0x0808080808080808, 0x0080402010080402, 0x0000010204081020],
    [0x0000000000ff0000, 0x1010101010101010, 0x0000804020100804, 0x0001020408102040],
    [0x0000000000ff0000, 0x2020202020202020, 0x0000008040201008, 0x0102040810204080],
    [0x0000000000ff0000, 0x4040404040404040, 0x0000000080402010, 0x0204081020408000],
    [0x0000000000ff0000, 0x8080808080808080, 0x0000000000804020, 0x0408102040800000],
    [0x00000000ff000000, 0x0101010101010101, 0x1008040201000000, 0x0000000001020408],
    [0x00000000ff000000, 0x0202020202020202, 0x2010080402010000, 0x0000000102040810],
    [0x00000000ff000000, 0x0404040404040404, 0x4020100804020100, 0x0000010204081020],
    [0x00000000ff000000, 0x0808080808080808, 0x8040201008040201, 0x0001020408102040],
    [0x00000000ff000000, 0x1010101010101010, 0x0080402010080402, 0x0102040810204080],
    [0x00000000ff000000, 0x2020202020202020, 0x0000804020100804, 0x0204081020408000],
    [0x00000000ff000000, 0x4040404040404040, 0x0000008040201008, 0x0408102040800000],
    [0x00000000ff000000, 0x8080808080808080, 0x0000000080402010, 0x0810204080000000],
    [0x000000ff00000000, 0x0101010101010101, 0x0804020100000000, 0x0000000102040810],
    [0x000000ff00000000, 0x0202020202020202, 0x1008040201000000, 0x0000010204081020],
    [0x000000ff00000000, 0x0404040404040404, 0x2010080402010000, 0x0001020408102040],
    [0x000000ff00000000, 0x0808080808080808, 0x4020100804020100, 0x0102040810204080],
    [0x000000ff00000000, 0x1010101010101010, 0x8040201008040201, 0x0204081020408000],
    [0x000000ff00000000, 0x2020202020202020, 0x0080402010080402, 0x0408102040800000],
    [0x000000ff00000000, 0x4040404040404040, 0x0000804020100804, 0x0810204080000000],
    [0x000000ff00000000, 0x8080808080808080, 0x0000008040201008, 0x1020408000000000],
    [0x0000ff0000000000, 0x0101010101010101, 0x0402010000000000, 0x0000010204081020],
    [0x0000ff0000000000, 0x0202020202020202, 0x0804020100000000, 0x0001020408102040],
    [0x0000ff0000000000, 0x0404040404040404, 0x1008040201000000, 0x0102040810204080],
    [0x0000ff0000000000, 0x0808080808080808, 0x2010080402010000, 0x0204081020408000],
    [0x0000ff0000000000, 0x1010101010101010, 0x4020100804020100, 0x0408102040800000],
    [0x0000ff0000000000, 0x2020202020202020, 0x8040201008040201, 0x0810204080000000],
    [0x0000ff0000000000, 0x4040404040404040, 0x0080402010080402, 0x1020408000000000],
    [0x0000ff0000000000, 0x8080808080808080, 0x0000804020100804, 0x2040800000000000],
    [0x00ff000000000000, 0x0101010101010101, 0x0201000000000000, 0x0001020408102040],
    [0x00ff000000000000, 0x0202020202020202, 0x0402010000000000, 0x0102040810204080],
    [0x00ff000000000000, 0x0404040404040404, 0x0804020100000000, 0x0204081020408000],
    [0x00ff000000000000, 0x0808080808080808, 0x1008040201000000, 0x0408102040800000],
    [0x00ff000000000000, 0x1010101010101010, 0x2010080402010000, 0x0810204080000000],
    [0x00ff000000000000, 0x2020202020202020, 0x4020100804020100, 0x1020408000000000],
    [0x00ff000000000000, 0x4040404040404040, 0x8040201008040201, 0x2040800000000000],
    [0x00ff000000000000, 0x8080808080808080, 0x0080402010080402, 0x4080000000000000],
    [0xff00000000000000, 0x0101010101010101, 0x0100000000000000, 0x0102040810204080],
    [0xff00000000000000, 0x0202020202020202, 0x0201000000000000, 0x0204081020408000],
    [0xff00000000000000, 0x0404040404040404, 0x0402010000000000, 0x0408102040800000],
    [0xff00000000000000, 0x0808080808080808, 0x0804020100000000, 0x0810204080000000],
    [0xff00000000000000, 0x1010101010101010, 0x1008040201000000, 0x1020408000000000],
    [0xff00000000000000, 0x2020202020202020, 0x2010080402010000, 0x2040800000000000],
    [0xff00000000000000, 0x4040404040404040, 0x4020100804020100, 0x4080000000000000],
    [0xff00000000000000, 0x8080808080808080, 0x8040201008040201, 0x8000000000000000],
];

const OUTFLANK : [[u8; 64]; 8] = [[0x00, 0x04, 0x00, 0x08, 0x00, 0x04, 0x00, 0x10, 0x00, 0x04, 0x00, 0x08, 0x00, 0x04, 0x00, 0x20,
0x00, 0x04, 0x00, 0x08, 0x00, 0x04, 0x00, 0x10, 0x00, 0x04, 0x00, 0x08, 0x00, 0x04, 0x00, 0x40,
0x00, 0x04, 0x00, 0x08, 0x00, 0x04, 0x00, 0x10, 0x00, 0x04, 0x00, 0x08, 0x00, 0x04, 0x00, 0x20,
0x00, 0x04, 0x00, 0x08, 0x00, 0x04, 0x00, 0x10, 0x00, 0x04, 0x00, 0x08, 0x00, 0x04, 0x00, 0x80],
[0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x10, 0x10, 0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x20, 0x20,
0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x10, 0x10, 0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x40, 0x40,
0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x10, 0x10, 0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x20, 0x20,
0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x10, 0x10, 0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x80, 0x80],
[0x00, 0x01, 0x00, 0x01, 0x10, 0x11, 0x10, 0x11, 0x00, 0x01, 0x00, 0x01, 0x20, 0x21, 0x20, 0x21,
0x00, 0x01, 0x00, 0x01, 0x10, 0x11, 0x10, 0x11, 0x00, 0x01, 0x00, 0x01, 0x40, 0x41, 0x40, 0x41,
0x00, 0x01, 0x00, 0x01, 0x10, 0x11, 0x10, 0x11, 0x00, 0x01, 0x00, 0x01, 0x20, 0x21, 0x20, 0x21,
0x00, 0x01, 0x00, 0x01, 0x10, 0x11, 0x10, 0x11, 0x00, 0x01, 0x00, 0x01, 0x80, 0x81, 0x80, 0x81],
[0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x02, 0x01, 0x20, 0x20, 0x22, 0x21, 0x20, 0x20, 0x22, 0x21,
0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x02, 0x01, 0x40, 0x40, 0x42, 0x41, 0x40, 0x40, 0x42, 0x41,
0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x02, 0x01, 0x20, 0x20, 0x22, 0x21, 0x20, 0x20, 0x22, 0x21,
0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x02, 0x01, 0x80, 0x80, 0x82, 0x81, 0x80, 0x80, 0x82, 0x81],
[0x00, 0x00, 0x00, 0x00, 0x04, 0x04, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x04, 0x04, 0x02, 0x01,
0x40, 0x40, 0x40, 0x40, 0x44, 0x44, 0x42, 0x41, 0x40, 0x40, 0x40, 0x40, 0x44, 0x44, 0x42, 0x41,
0x00, 0x00, 0x00, 0x00, 0x04, 0x04, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x04, 0x04, 0x02, 0x01,
0x80, 0x80, 0x80, 0x80, 0x84, 0x84, 0x82, 0x81, 0x80, 0x80, 0x80, 0x80, 0x84, 0x84, 0x82, 0x81],
[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x08, 0x08, 0x08, 0x04, 0x04, 0x02, 0x01,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x08, 0x08, 0x08, 0x04, 0x04, 0x02, 0x01,
0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x88, 0x88, 0x88, 0x88, 0x84, 0x84, 0x82, 0x81,
0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x88, 0x88, 0x88, 0x88, 0x84, 0x84, 0x82, 0x81],
[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x08, 0x08, 0x08, 0x08, 0x04, 0x04, 0x02, 0x01,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x08, 0x08, 0x08, 0x08, 0x04, 0x04, 0x02, 0x01],
[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x08, 0x08, 0x08, 0x08, 0x04, 0x04, 0x02, 0x01]];

const FLIP : [[u8;256]; 8] = [[0x00, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x0f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x1f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x0f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x3f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x0f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x1f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x0f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x7f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x0f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x1f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x0f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x3f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x0f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x1f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00,
0x0f, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00],
[0x00, 0x02, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x0e, 0x0e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x1e, 0x1e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x0e, 0x0e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x3e, 0x3e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x0e, 0x0e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x1e, 0x1e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x0e, 0x0e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x7e, 0x7e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x0e, 0x0e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x1e, 0x1e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x0e, 0x0e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x3e, 0x3e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x0e, 0x0e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x1e, 0x1e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
0x0e, 0x0e, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00],
[0x00, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x0c, 0x0e, 0x0c, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x1c, 0x1e, 0x1c, 0x1c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x0c, 0x0e, 0x0c, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x3c, 0x3e, 0x3c, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x0c, 0x0e, 0x0c, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x1c, 0x1e, 0x1c, 0x1c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x0c, 0x0e, 0x0c, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x7c, 0x7e, 0x7c, 0x7c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x0c, 0x0e, 0x0c, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x1c, 0x1e, 0x1c, 0x1c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x0c, 0x0e, 0x0c, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x3c, 0x3e, 0x3c, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x0c, 0x0e, 0x0c, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x1c, 0x1e, 0x1c, 0x1c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00,
0x0c, 0x0e, 0x0c, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00],
[0x00, 0x0e, 0x0c, 0x0c, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x08, 0x0e, 0x0c, 0x0c, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x18, 0x1e, 0x1c, 0x1c, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x08, 0x0e, 0x0c, 0x0c, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x38, 0x3e, 0x3c, 0x3c, 0x38, 0x38, 0x38, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x08, 0x0e, 0x0c, 0x0c, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x18, 0x1e, 0x1c, 0x1c, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x08, 0x0e, 0x0c, 0x0c, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x78, 0x7e, 0x7c, 0x7c, 0x78, 0x78, 0x78, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x08, 0x0e, 0x0c, 0x0c, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x18, 0x1e, 0x1c, 0x1c, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x08, 0x0e, 0x0c, 0x0c, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x38, 0x3e, 0x3c, 0x3c, 0x38, 0x38, 0x38, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x08, 0x0e, 0x0c, 0x0c, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x18, 0x1e, 0x1c, 0x1c, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x08, 0x0e, 0x0c, 0x0c, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
[0x00, 0x1e, 0x1c, 0x1c, 0x18, 0x18, 0x18, 0x18, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x10, 0x1e, 0x1c, 0x1c, 0x18, 0x18, 0x18, 0x18, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x30, 0x3e, 0x3c, 0x3c, 0x38, 0x38, 0x38, 0x38, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x10, 0x1e, 0x1c, 0x1c, 0x18, 0x18, 0x18, 0x18, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x70, 0x7e, 0x7c, 0x7c, 0x78, 0x78, 0x78, 0x78, 0x70, 0x70, 0x70, 0x70, 0x70, 0x70, 0x70, 0x70,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x10, 0x1e, 0x1c, 0x1c, 0x18, 0x18, 0x18, 0x18, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x30, 0x3e, 0x3c, 0x3c, 0x38, 0x38, 0x38, 0x38, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x10, 0x1e, 0x1c, 0x1c, 0x18, 0x18, 0x18, 0x18, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
[0x00, 0x3e, 0x3c, 0x3c, 0x38, 0x38, 0x38, 0x38, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x20, 0x3e, 0x3c, 0x3c, 0x38, 0x38, 0x38, 0x38, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x60, 0x7e, 0x7c, 0x7c, 0x78, 0x78, 0x78, 0x78, 0x70, 0x70, 0x70, 0x70, 0x70, 0x70, 0x70, 0x70,
0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x20, 0x3e, 0x3c, 0x3c, 0x38, 0x38, 0x38, 0x38, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
[0x00, 0x7e, 0x7c, 0x7c, 0x78, 0x78, 0x78, 0x78, 0x70, 0x70, 0x70, 0x70, 0x70, 0x70, 0x70, 0x70,
0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60,
0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40,
0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x40, 0x7e, 0x7c, 0x7c, 0x78, 0x78, 0x78, 0x78, 0x70, 0x70, 0x70, 0x70, 0x70, 0x70, 0x70, 0x70,
0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x60,
0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40,
0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
[0x00, 0xfe, 0xfc, 0xfc, 0xf8, 0xf8, 0xf8, 0xf8, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0,
0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0,
0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0,
0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0,
0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]];

///x -> the offset of the move 
///p -> the player's stones
///o -> the opponent's stones
pub fn fast_do_move(x : u8, y : u8, p : u64, o : u64) -> u64 {
    #![allow(unused_assignments)]

    use std::cmp;
    let mut ph : u64 = 0;
    let mut pv : u64 = 0;
    let mut pd : u64 = 0;
    let mut pa : u64 = 0;

    let mut oh : u64 = 0;
    let mut ov : u64 = 0;
    let mut od : u64 = 0;
    let mut oa : u64 = 0;

    let mut ofh = 0;
    let mut ofv = 0;
    let mut ofd = 0;
    let mut ofa = 0;

    let mut fh = 0;
    let mut fv = 0;
    let mut fd = 0;
    let mut fa = 0;

    let mut mh : u64 = 0;
    let mut mv : u64 = 0;
    let mut md : u64 = 0;
    let mut ma : u64 = 0;

    let masks = LINES[((y<<3) + x) as usize];
    const NOT_EDGES_H : u64 = !0x8181818181818181;
    const NOT_EDGES_V : u64 = !0xFF000000000000FF;
    const NOT_EDGES_D : u64 = !0xFF818181818181FF;

    unsafe {
        asm!("PEXT $2, $1, $0" : "=r"(ph) : "r"(p) , "r"(masks[0]));
        asm!("PEXT $2, $1, $0" : "=r"(pv) : "r"(p) , "r"(masks[1]));
        asm!("PEXT $2, $1, $0" : "=r"(pd) : "r"(p) , "r"(masks[2]));
        asm!("PEXT $2, $1, $0" : "=r"(pa) : "r"(p) , "r"(masks[3]));

        asm!("PEXT $2, $1, $0" : "=r"(oh) : "r"(o) , "r"(masks[0] & NOT_EDGES_H));
        asm!("PEXT $2, $1, $0" : "=r"(ov) : "r"(o) , "r"(masks[1] & NOT_EDGES_V));
        asm!("PEXT $2, $1, $0" : "=r"(od) : "r"(o) , "r"(masks[2] & NOT_EDGES_D));
        asm!("PEXT $2, $1, $0" : "=r"(oa) : "r"(o) , "r"(masks[3] & NOT_EDGES_D));
    }

    let x1 = x as usize;
    let y1 = y as usize;
    let x2 = (if x < y {x} else {y}) as usize;
    let y2 = (if 7-x < y {7-x} else {y}) as usize;

    fh |= FLIP[x1][(OUTFLANK[x1][oh as usize] & ph as u8) as usize];
    fv |= FLIP[y1][(OUTFLANK[y1][ov as usize] & pv as u8) as usize];
    fd |= FLIP[x2][(OUTFLANK[x2][od as usize] & pd as u8) as usize];
    fa |= FLIP[y2][(OUTFLANK[y2][oa as usize] & pa as u8) as usize];

    unsafe {
        asm!("PDEP $2, $1, $0" : "=r"(mh) : "r"(fh as u64) , "r"(masks[0]));
        asm!("PDEP $2, $1, $0" : "=r"(mv) : "r"(fv as u64) , "r"(masks[1]));
        asm!("PDEP $2, $1, $0" : "=r"(md) : "r"(fd as u64) , "r"(masks[2]));
        asm!("PDEP $2, $1, $0" : "=r"(ma) : "r"(fa as u64) , "r"(masks[3]));
    }

    (mh | mv | md | ma) as u64
}