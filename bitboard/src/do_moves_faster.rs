use board::*;

const OUTFLANK_2
    : [u8; 64]
    = [   0x0u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x10u8,
          0x11u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x20u8,
          0x21u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x10u8,
          0x11u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x40u8,
          0x41u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x10u8,
          0x11u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x20u8,
          0x21u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x10u8,
          0x11u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x80u8,
          0x81u8,
          0x0u8,
          0x0u8
      ];

const OUTFLANK_3
    : [u8; 64]
    = [   0x0u8,
          0x0u8,
          0x2u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x20u8,
          0x20u8,
          0x22u8,
          0x21u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x2u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x40u8,
          0x40u8,
          0x42u8,
          0x41u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x2u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x20u8,
          0x20u8,
          0x22u8,
          0x21u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x2u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x80u8,
          0x80u8,
          0x82u8,
          0x81u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8
      ];

const OUTFLANK_4
    : [u8; 64]
    = [   0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x4u8,
          0x4u8,
          0x2u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x40u8,
          0x40u8,
          0x40u8,
          0x40u8,
          0x44u8,
          0x44u8,
          0x42u8,
          0x41u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x4u8,
          0x4u8,
          0x2u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x80u8,
          0x80u8,
          0x80u8,
          0x80u8,
          0x84u8,
          0x84u8,
          0x82u8,
          0x81u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8
      ];

const OUTFLANK_5
    : [u8; 64]
    = [   0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x8u8,
          0x8u8,
          0x8u8,
          0x8u8,
          0x4u8,
          0x4u8,
          0x2u8,
          0x1u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x80u8,
          0x80u8,
          0x80u8,
          0x80u8,
          0x80u8,
          0x80u8,
          0x80u8,
          0x80u8,
          0x88u8,
          0x88u8,
          0x88u8,
          0x88u8,
          0x84u8,
          0x84u8,
          0x82u8,
          0x81u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8
      ];

const OUTFLANK_7
    : [u8; 64]
    = [   0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x0u8,
          0x20u8,
          0x20u8,
          0x20u8,
          0x20u8,
          0x20u8,
          0x20u8,
          0x20u8,
          0x20u8,
          0x20u8,
          0x20u8,
          0x20u8,
          0x20u8,
          0x20u8,
          0x20u8,
          0x20u8,
          0x20u8,
          0x10u8,
          0x10u8,
          0x10u8,
          0x10u8,
          0x10u8,
          0x10u8,
          0x10u8,
          0x10u8,
          0x8u8,
          0x8u8,
          0x8u8,
          0x8u8,
          0x4u8,
          0x4u8,
          0x2u8,
          0x1u8
      ];

const FLIPPED_2_H
    : [u64; 130]
    = [   0x0u64,
          0x202020202020202u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x808080808080808u64,
          0xa0a0a0a0a0a0a0au64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x1818181818181818u64,
          0x1a1a1a1a1a1a1a1au64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x3838383838383838u64,
          0x3a3a3a3a3a3a3a3au64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x7878787878787878u64,
          0x7a7a7a7a7a7a7a7au64
      ];

const FLIPPED_3_H
    : [u64; 131]
    = [   0x0u64,
          0x606060606060606u64,
          0x404040404040404u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x1010101010101010u64,
          0x1616161616161616u64,
          0x1414141414141414u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x3030303030303030u64,
          0x3636363636363636u64,
          0x3434343434343434u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x7070707070707070u64,
          0x7676767676767676u64,
          0x7474747474747474u64
      ];

const FLIPPED_4_H
    : [u64; 133]
    = [   0x0u64,
          0xe0e0e0e0e0e0e0eu64,
          0xc0c0c0c0c0c0c0cu64,
          0x0u64,
          0x808080808080808u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x2020202020202020u64,
          0x2e2e2e2e2e2e2e2eu64,
          0x2c2c2c2c2c2c2c2cu64,
          0x0u64,
          0x2828282828282828u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x6060606060606060u64,
          0x6e6e6e6e6e6e6e6eu64,
          0x6c6c6c6c6c6c6c6cu64,
          0x0u64,
          0x6868686868686868u64
      ];

const FLIPPED_5_H
    : [u64; 137]
    = [   0x0u64,
          0x1e1e1e1e1e1e1e1eu64,
          0x1c1c1c1c1c1c1c1cu64,
          0x0u64,
          0x1818181818181818u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x1010101010101010u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x4040404040404040u64,
          0x5e5e5e5e5e5e5e5eu64,
          0x5c5c5c5c5c5c5c5cu64,
          0x0u64,
          0x5858585858585858u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x5050505050505050u64
      ];

const FLIPPED_2_V
    : [u64; 130]
    = [   0x0u64,
          0xff00u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0xff000000u64,
          0xff00ff00u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0xffff000000u64,
          0xffff00ff00u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0xffffff000000u64,
          0xffffff00ff00u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0xffffffff000000u64,
          0xffffffff00ff00u64
      ];

const FLIPPED_3_V
    : [u64; 131]
    = [   0x0u64,
          0xffff00u64,
          0xff0000u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0xff00000000u64,
          0xff00ffff00u64,
          0xff00ff0000u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0xffff00000000u64,
          0xffff00ffff00u64,
          0xffff00ff0000u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0xffffff00000000u64,
          0xffffff00ffff00u64,
          0xffffff00ff0000u64
      ];

const FLIPPED_4_V
    : [u64; 133]
    = [   0x0u64,
          0xffffff00u64,
          0xffff0000u64,
          0x0u64,
          0xff000000u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0xff0000000000u64,
          0xff00ffffff00u64,
          0xff00ffff0000u64,
          0x0u64,
          0xff00ff000000u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0xffff0000000000u64,
          0xffff00ffffff00u64,
          0xffff00ffff0000u64,
          0x0u64,
          0xffff00ff000000u64
      ];

const FLIPPED_5_V
    : [u64; 137]
    = [   0x0u64,
          0xffffffff00u64,
          0xffffff0000u64,
          0x0u64,
          0xffff000000u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0xff00000000u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0xff000000000000u64,
          0xff00ffffffff00u64,
          0xff00ffffff0000u64,
          0x0u64,
          0xff00ffff000000u64,
          0x0u64,
          0x0u64,
          0x0u64,
          0xff00ff00000000u64
      ];


fn flip_A1(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d9 : u64;
    outflank_v = ((O as (u64) | !0x101010101010100u64).wrapping_add(
                      0x100u64
                  ) & P as (u64) & 0x101010101010000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x1010101010100u64) as (u64);
    outflank_h = ((O & 0x7eu64).wrapping_add(0x2u64) & P) as (u32);
    flipped = flipped | (outflank_h.wrapping_sub(
                             (outflank_h != 0u32) as (u32)
                         ) & 0x7eu32) as (u64);
    outflank_d9 = ((O as (u64) | !0x8040201008040200u64).wrapping_add(
                       0x200u64
                   ) & P as (u64) & 0x8040201008040000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d9.wrapping_sub(
                                      (outflank_d9 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x40201008040200u64) as (u64);
    flipped
}

fn flip_B1(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d9 : u64;
    outflank_v = ((O as (u64) | !0x202020202020200u64).wrapping_add(
                      0x200u64
                  ) & P as (u64) & 0x202020202020000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x2020202020200u64) as (u64);
    outflank_h = ((O & 0x7cu64).wrapping_add(0x4u64) & P) as (u32);
    flipped = flipped | (outflank_h.wrapping_sub(
                             (outflank_h != 0u32) as (u32)
                         ) & 0x7cu32) as (u64);
    outflank_d9 = ((O as (u64) | !0x80402010080400u64).wrapping_add(
                       0x400u64
                   ) & P as (u64) & 0x80402010080000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d9.wrapping_sub(
                                      (outflank_d9 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x402010080400u64) as (u64);
    flipped
}

fn flip_C1(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d9 : u64;
    outflank_v = ((O as (u64) | !0x404040404040400u64).wrapping_add(
                      0x400u64
                  ) & P as (u64) & 0x404040404040400u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x404040404040400u64) as (u64);
    outflank_h = (OUTFLANK_2[(
                      O >> 1i32 & 0x3fu64
                  ) as usize ] as (u64) & P) as (u32);
    flipped = flipped | FLIPPED_2_H[(
                            outflank_h as (usize) ) as usize ] as (u8) as (u64);
    flipped = flipped | (P as (u32) >> 7i32 & 0x200u32 & O as (u32)) as (u64);
    outflank_d9 = ((O as (u64) | !0x804020100800u64).wrapping_add(
                       0x800u64
                   ) & P as (u64) & 0x804020100800u64) as (u64);
    flipped = (flipped as (u64) | outflank_d9.wrapping_sub(
                                      (outflank_d9 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x804020100800u64) as (u64);
    flipped
}

fn flip_D1(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_d7 : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d9 : u64;
    outflank_v = ((O as (u64) | !0x808080808080800u64).wrapping_add(
                      0x800u64
                  ) & P as (u64) & 0x808080808080800u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x808080808080800u64) as (u64);
    outflank_h = (OUTFLANK_3[(
                      O >> 1i32 & 0x3fu64
                  ) as usize ] as (u64) & P) as (u32);
    flipped = flipped | FLIPPED_3_H[(
                            outflank_h as (usize) ) as usize ] as (u8) as (u64);
    outflank_d7 = ((O | !0x1020400u32 as (u64)).wrapping_add(
                       0x400u64
                   ) & P & 0x1020000u64) as (u32);
    flipped = flipped | (outflank_d7.wrapping_sub(
                             (outflank_d7 != 0u32) as (u32)
                         ) & 0x20400u32) as (u64);
    outflank_d9 = ((O as (u64) | !0x8040201000u64).wrapping_add(
                       0x1000u64
                   ) & P as (u64) & 0x8040201000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d9.wrapping_sub(
                                      (outflank_d9 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x8040201000u64) as (u64);
    flipped
}

fn flip_E1(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d7 : u64;
    outflank_v = ((O as (u64) | !0x1010101010101000u64).wrapping_add(
                      0x1000u64
                  ) & P as (u64) & 0x1010101010101000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x1010101010101000u64) as (u64);
    outflank_h = (OUTFLANK_4[(
                      O >> 1i32 & 0x3fu64
                  ) as usize ] as (u64) & P) as (u32);
    flipped = flipped | FLIPPED_4_H[(
                            outflank_h as (usize) ) as usize ] as (u8) as (u64);
    outflank_d7 = ((O as (u64) | !0x102040800u64).wrapping_add(
                       0x800u64
                   ) & P as (u64) & 0x102040800u64) as (u64);
    flipped = (flipped as (u64) | outflank_d7.wrapping_sub(
                                      (outflank_d7 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x102040800u64) as (u64);
    outflank_d9 = ((O | !0x80402000u32 as (u64)).wrapping_add(
                       0x2000u64
                   ) & P & 0x80400000u64) as (u32);
    flipped = flipped | (outflank_d9.wrapping_sub(
                             (outflank_d9 != 0u32) as (u32)
                         ) & 0x402000u32) as (u64);
    flipped
}

fn flip_F1(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d7 : u64;
    outflank_v = ((O as (u64) | !0x2020202020202000u64).wrapping_add(
                      0x2000u64
                  ) & P as (u64) & 0x2020202020202000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x2020202020202000u64) as (u64);
    outflank_h = (OUTFLANK_5[(
                      O >> 1i32 & 0x3fu64
                  ) as usize ] as (u64) & P) as (u32);
    flipped = flipped | FLIPPED_5_H[(
                            outflank_h as (usize) ) as usize ] as (u8) as (u64);
    outflank_d7 = ((O as (u64) | !0x10204081000u64).wrapping_add(
                       0x1000u64
                   ) & P as (u64) & 0x10204080000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d7.wrapping_sub(
                                      (outflank_d7 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x204081000u64) as (u64);
    flipped = flipped | (P as (u32) >> 9i32 & 0x4000u32 & O as (u32)) as (u64);
    flipped
}

fn flip_G1(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d7 : u64;
    outflank_v = ((O as (u64) | !0x4040404040404000u64).wrapping_add(
                      0x4000u64
                  ) & P as (u64) & 0x4040404040400000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x40404040404000u64) as (u64);
    outflank_h = (OUTFLANK_7[(
                      O & 0x3eu64
                  ) as usize ] as (u64) & P << 1i32) as (u32);
    flipped = flipped | ((outflank_h.wrapping_neg(
                          ) & 0x3eu32) << 0i32) as (u64);
    outflank_d7 = ((O as (u64) | !0x1020408102000u64).wrapping_add(
                       0x2000u64
                   ) & P as (u64) & 0x1020408100000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d7.wrapping_sub(
                                      (outflank_d7 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x20408102000u64) as (u64);
    flipped
}

fn flip_H1(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d7 : u64;
    outflank_v = ((O as (u64) | !0x8080808080808000u64).wrapping_add(
                      0x8000u64
                  ) & P as (u64) & 0x8080808080800000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x80808080808000u64) as (u64);
    outflank_h = OUTFLANK_7[(
                     O >> 1i32 & 0x3fu64
                 ) as usize ] as (u32) & P as (u32);
    flipped = flipped | ((outflank_h.wrapping_neg(
                          ) & 0x3fu32) << 1i32) as (u64);
    outflank_d7 = ((O as (u64) | !0x102040810204000u64).wrapping_add(
                       0x4000u64
                   ) & P as (u64) & 0x102040810200000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d7.wrapping_sub(
                                      (outflank_d7 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x2040810204000u64) as (u64);
    flipped
}

fn flip_A2(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d9 : u64;
    outflank_v = ((O as (u64) | !0x101010101010000u64).wrapping_add(
                      0x10000u64
                  ) & P as (u64) & 0x101010101000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x1010101010000u64) as (u64);
    outflank_h = ((O & 0x7e00u64).wrapping_add(
                      0x200u64
                  ) & P) as (u32);
    flipped = flipped | (outflank_h.wrapping_sub(
                             outflank_h >> 8i32
                         ) & 0x7e00u32) as (u64);
    outflank_d9 = ((O as (u64) | !0x4020100804020000u64).wrapping_add(
                       0x20000u64
                   ) & P as (u64) & 0x4020100804000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d9.wrapping_sub(
                                      (outflank_d9 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x20100804020000u64) as (u64);
    flipped
}

fn flip_B2(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d9 : u64;
    outflank_v = ((O as (u64) | !0x202020202020000u64).wrapping_add(
                      0x20000u64
                  ) & P as (u64) & 0x202020202000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x2020202020000u64) as (u64);
    outflank_h = ((O & 0x7c00u64).wrapping_add(
                      0x400u64
                  ) & P) as (u32);
    flipped = flipped | (outflank_h.wrapping_sub(
                             outflank_h >> 8i32
                         ) & 0x7c00u32) as (u64);
    outflank_d9 = ((O as (u64) | !0x8040201008040000u64).wrapping_add(
                       0x40000u64
                   ) & P as (u64) & 0x8040201008000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d9.wrapping_sub(
                                      (outflank_d9 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x40201008040000u64) as (u64);
    flipped
}

fn flip_C2(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d9 : u64;
    outflank_v = ((O as (u64) | !0x404040404040000u64).wrapping_add(
                      0x40000u64
                  ) & P as (u64) & 0x404040404000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x4040404040000u64) as (u64);
    outflank_h = (OUTFLANK_2[(
                      O >> 9i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 8i32) as (u32);
    flipped = flipped | FLIPPED_2_H[(
                            outflank_h as (usize) ) as usize ] & 0xff00u64;
    flipped = flipped | (P as (u32) >> 7i32 & 0x20000u32 & O as (u32)) as (u64);
    outflank_d9 = ((O as (u64) | !0x80402010080000u64).wrapping_add(
                       0x80000u64
                   ) & P as (u64) & 0x80402010080000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d9.wrapping_sub(
                                      (outflank_d9 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x402010080000u64) as (u64);
    flipped
}

fn flip_D2(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d7 : u64;
    let mut outflank_d9 : u64;
    outflank_v = ((O as (u64) | !0x808080808080000u64).wrapping_add(
                      0x80000u64
                  ) & P as (u64) & 0x808080808000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x8080808080000u64) as (u64);
    outflank_h = (OUTFLANK_3[(
                      O >> 9i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 8i32) as (u32);
    flipped = flipped | FLIPPED_3_H[(
                            outflank_h as (usize) ) as usize ] & 0xff00u64;
    outflank_d7 = ((O as (u64) | !0x102040000u64).wrapping_add(
                       0x40000u64
                   ) & P as (u64) & 0x102000000u64) as (u64);
    flipped = flipped | outflank_d7.wrapping_sub(
                            (outflank_d7 != 0u64) as (u32) as (u64)
                        ) & 0x2040000u64;
    outflank_d9 = ((O as (u64) | !0x804020100000u64).wrapping_add(
                       0x100000u64
                   ) & P as (u64) & 0x804020000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d9.wrapping_sub(
                                      (outflank_d9 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x4020100000u64) as (u64);
    flipped
}

fn flip_E2(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d7 : u64;
    let mut outflank_d9 : u64;
    outflank_v = ((O as (u64) | !0x1010101010100000u64).wrapping_add(
                      0x100000u64
                  ) & P as (u64) & 0x1010101010000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x10101010100000u64) as (u64);
    outflank_h = (OUTFLANK_4[(
                      O >> 9i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 8i32) as (u32);
    flipped = flipped | FLIPPED_4_H[(
                            outflank_h as (usize) ) as usize ] & 0xff00u64;
    outflank_d7 = ((O as (u64) | !0x10204080000u64).wrapping_add(
                       0x80000u64
                   ) & P as (u64) & 0x10204000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d7.wrapping_sub(
                                      (outflank_d7 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x204080000u64) as (u64);
    outflank_d9 = ((O as (u64) | !0x8040200000u64).wrapping_add(
                       0x200000u64
                   ) & P as (u64) & 0x8040000000u64) as (u64);
    flipped = flipped | outflank_d9.wrapping_sub(
                            (outflank_d9 != 0u64) as (u32) as (u64)
                        ) & 0x40200000u64;
    flipped
}

fn flip_F2(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d7 : u64;
    outflank_v = ((O as (u64) | !0x2020202020200000u64).wrapping_add(
                      0x200000u64
                  ) & P as (u64) & 0x2020202020000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x20202020200000u64) as (u64);
    outflank_h = OUTFLANK_5[(
                     O >> 9i32 & 0x3fu64
                 ) as usize ] as (u32) & P as (u32) >> 8i32;
    flipped = flipped | (FLIPPED_5_H[(
                             outflank_h as (usize) ) as usize ] as (u32) & 0xff00u32) as (u64);
    outflank_d7 = ((O as (u64) | !0x1020408100000u64).wrapping_add(
                       0x100000u64
                   ) & P as (u64) & 0x1020408000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d7.wrapping_sub(
                                      (outflank_d7 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x20408100000u64) as (u64);
    flipped = flipped | (P as (u32) >> 9i32 & 0x400000u32 & O as (u32)) as (u64);
    flipped
}

fn flip_G2(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d7 : u64;
    outflank_v = ((O as (u64) | !0x4040404040400000u64).wrapping_add(
                      0x400000u64
                  ) & P as (u64) & 0x4040404040000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x40404040400000u64) as (u64);
    outflank_h = OUTFLANK_7[(
                     O >> 8i32 & 0x3eu64
                 ) as usize ] as (u32) & P as (u32) >> 7i32;
    flipped = flipped | ((outflank_h.wrapping_neg(
                          ) & 0x3eu32) << 8i32) as (u64);
    outflank_d7 = ((O as (u64) | !0x102040810200000u64).wrapping_add(
                       0x200000u64
                   ) & P as (u64) & 0x102040810000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d7.wrapping_sub(
                                      (outflank_d7 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x2040810200000u64) as (u64);
    flipped
}

fn flip_H2(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d7 : u64;
    outflank_v = ((O as (u64) | !0x8080808080800000u64).wrapping_add(
                      0x800000u64
                  ) & P as (u64) & 0x8080808080000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x80808080800000u64) as (u64);
    outflank_h = OUTFLANK_7[(
                     O >> 9i32 & 0x3fu64
                 ) as usize ] as (u32) & P as (u32) >> 8i32;
    flipped = flipped | ((outflank_h.wrapping_neg(
                          ) & 0x3fu32) << 9i32) as (u64);
    outflank_d7 = ((O as (u64) | !0x204081020400000u64).wrapping_add(
                       0x400000u64
                   ) & P as (u64) & 0x204081020000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d7.wrapping_sub(
                                      (outflank_d7 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x4081020400000u64) as (u64);
    flipped
}

fn flip_A3(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d9 : u64;
    outflank_v = ((O as (u64) | !0x101010101000000u64).wrapping_add(
                      0x1000000u64
                  ) & P as (u64) & 0x101010101000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x101010101000000u64) as (u64);
    flipped = flipped | (P as (u32) << 8i32 & 0x100u32 & O as (u32)) as (u64);
    outflank_h = ((O & 0x7e0000u64).wrapping_add(
                      0x20000u64
                  ) & P) as (u32);
    flipped = flipped | (outflank_h.wrapping_sub(
                             outflank_h >> 8i32
                         ) & 0x7e0000u32) as (u64);
    flipped = flipped | (P as (u32) << 7i32 & 0x200u32 & O as (u32)) as (u64);
    outflank_d9 = ((O as (u64) | !0x2010080402000000u64).wrapping_add(
                       0x2000000u64
                   ) & P as (u64) & 0x2010080400000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d9.wrapping_sub(
                                      (outflank_d9 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x10080402000000u64) as (u64);
    flipped
}

fn flip_B3(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d9 : u64;
    outflank_v = ((O as (u64) | !0x202020202000000u64).wrapping_add(
                      0x2000000u64
                  ) & P as (u64) & 0x202020202000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x202020202000000u64) as (u64);
    flipped = flipped | (P as (u32) << 8i32 & 0x200u32 & O as (u32)) as (u64);
    outflank_h = ((O & 0x7c0000u64).wrapping_add(
                      0x40000u64
                  ) & P) as (u32);
    flipped = flipped | (outflank_h.wrapping_sub(
                             outflank_h >> 8i32
                         ) & 0x7c0000u32) as (u64);
    flipped = flipped | (P as (u32) << 7i32 & 0x400u32 & O as (u32)) as (u64);
    outflank_d9 = ((O as (u64) | !0x4020100804000000u64).wrapping_add(
                       0x4000000u64
                   ) & P as (u64) & 0x4020100800000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d9.wrapping_sub(
                                      (outflank_d9 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x20100804000000u64) as (u64);
    flipped
}

fn flip_C3(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d9 : u64;
    outflank_v = ((O as (u64) | !0x404040404000000u64).wrapping_add(
                      0x4000000u64
                  ) & P as (u64) & 0x404040404000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x404040404000000u64) as (u64);
    flipped = flipped | (P as (u32) << 8i32 & 0x400u32 & O as (u32)) as (u64);
    outflank_h = OUTFLANK_2[(
                     O >> 17i32 & 0x3fu64
                 ) as usize ] as (u32) & P as (u32) >> 16i32;
    flipped = flipped | (FLIPPED_2_H[(
                             outflank_h as (usize) ) as usize ] as (u32) & 0xff0000u32) as (u64);
    flipped = flipped | (((P >> 32i32) as (u32) << 25i32 | P as (u32) << 7i32) & 0x2000800u32 & O as (u32)) as (u64);
    outflank_d9 = ((O as (u64) | !0x8040201008000000u64).wrapping_add(
                       0x8000000u64
                   ) & P as (u64) & 0x8040201008000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d9.wrapping_sub(
                                      (outflank_d9 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x8040201008000000u64) as (u64);
    flipped = flipped | (P as (u32) << 9i32 & 0x200u32 & O as (u32)) as (u64);
    flipped
}

fn flip_D3(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d7 : u64;
    let mut outflank_d9 : u64;
    outflank_v = ((O as (u64) | !0x808080808000000u64).wrapping_add(
                      0x8000000u64
                  ) & P as (u64) & 0x808080808000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x808080808000000u64) as (u64);
    flipped = flipped | (P as (u32) << 8i32 & 0x800u32 & O as (u32)) as (u64);
    outflank_h = OUTFLANK_3[(
                     O >> 17i32 & 0x3fu64
                 ) as usize ] as (u32) & P as (u32) >> 16i32;
    flipped = flipped | (FLIPPED_3_H[(
                             outflank_h as (usize) ) as usize ] as (u32) & 0xff0000u32) as (u64);
    outflank_d7 = ((O as (u64) | !0x10204000000u64).wrapping_add(
                       0x4000000u64
                   ) & P as (u64) & 0x10204000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d7.wrapping_sub(
                                      outflank_d7 >> 24i32
                                  ) as (u64) & 0x10204000000u64) as (u64);
    flipped = flipped | (P as (u32) << 7i32 & 0x1000u32 & O as (u32)) as (u64);
    outflank_d9 = ((O as (u64) | !0x80402010000000u64).wrapping_add(
                       0x10000000u64
                   ) & P as (u64) & 0x80402010000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d9.wrapping_sub(
                                      (outflank_d9 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x80402010000000u64) as (u64);
    flipped = flipped | (P as (u32) << 9i32 & 0x400u32 & O as (u32)) as (u64);
    flipped
}

fn flip_E3(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d7 : u64;
    let mut outflank_d9 : u64;
    outflank_v = ((O as (u64) | !0x1010101010000000u64).wrapping_add(
                      0x10000000u64
                  ) & P as (u64) & 0x1010101010000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x1010101010000000u64) as (u64);
    flipped = flipped | (P as (u32) << 8i32 & 0x1000u32 & O as (u32)) as (u64);
    outflank_h = OUTFLANK_4[(
                     O >> 17i32 & 0x3fu64
                 ) as usize ] as (u32) & P as (u32) >> 16i32;
    flipped = flipped | (FLIPPED_4_H[(
                             outflank_h as (usize) ) as usize ] as (u32) & 0xff0000u32) as (u64);
    outflank_d7 = ((O as (u64) | !0x1020408000000u64).wrapping_add(
                       0x8000000u64
                   ) & P as (u64) & 0x1020408000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d7.wrapping_sub(
                                      (outflank_d7 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x1020408000000u64) as (u64);
    flipped = flipped | (P as (u32) << 7i32 & 0x2000u32 & O as (u32)) as (u64);
    outflank_d9 = ((O as (u64) | !0x804020000000u64).wrapping_add(
                       0x20000000u64
                   ) & P as (u64) & 0x804020000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d9.wrapping_sub(
                                      outflank_d9 >> 24i32
                                  ) as (u64) & 0x804020000000u64) as (u64);
    flipped = flipped | (P as (u32) << 9i32 & 0x800u32 & O as (u32)) as (u64);
    flipped
}

fn flip_F3(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d7 : u64;
    outflank_v = ((O as (u64) | !0x2020202020000000u64).wrapping_add(
                      0x20000000u64
                  ) & P as (u64) & 0x2020202020000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x2020202020000000u64) as (u64);
    flipped = flipped | (P as (u32) << 8i32 & 0x2000u32 & O as (u32)) as (u64);
    outflank_h = OUTFLANK_5[(
                     O >> 17i32 & 0x3fu64
                 ) as usize ] as (u32) & P as (u32) >> 16i32;
    flipped = flipped | (FLIPPED_5_H[(
                             outflank_h as (usize) ) as usize ] as (u32) & 0xff0000u32) as (u64);
    outflank_d7 = ((O as (u64) | !0x102040810000000u64).wrapping_add(
                       0x10000000u64
                   ) & P as (u64) & 0x102040810000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d7.wrapping_sub(
                                      (outflank_d7 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x102040810000000u64) as (u64);
    flipped = flipped | (P as (u32) << 7i32 & 0x4000u32 & O as (u32)) as (u64);
    flipped = flipped | (((P >> 32i32) as (u32) << 23i32 | P as (u32) << 9i32) & 0x40001000u32 & O as (u32)) as (u64);
    flipped
}

fn flip_G3(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d7 : u64;
    outflank_v = ((O as (u64) | !0x4040404040000000u64).wrapping_add(
                      0x40000000u64
                  ) & P as (u64) & 0x4040404040000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x4040404040000000u64) as (u64);
    flipped = flipped | (P as (u32) << 8i32 & 0x4000u32 & O as (u32)) as (u64);
    outflank_h = OUTFLANK_7[(
                     O >> 16i32 & 0x3eu64
                 ) as usize ] as (u32) & P as (u32) >> 15i32;
    flipped = flipped | ((outflank_h.wrapping_neg(
                          ) & 0x3eu32) << 16i32) as (u64);
    outflank_d7 = ((O as (u64) | !0x204081020000000u64).wrapping_add(
                       0x20000000u64
                   ) & P as (u64) & 0x204081000000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d7.wrapping_sub(
                                      (outflank_d7 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x4081020000000u64) as (u64);
    flipped = flipped | (P as (u32) << 9i32 & 0x2000u32 & O as (u32)) as (u64);
    flipped
}

fn flip_H3(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut flipped : u64;
    let mut outflank_v : u64;
    let mut outflank_d7 : u64;
    outflank_v = ((O as (u64) | !0x8080808080000000u64).wrapping_add(
                      0x80000000u64
                  ) & P as (u64) & 0x8080808080000000u64) as (u64);
    flipped = (outflank_v.wrapping_sub(
                   (outflank_v != 0u64) as (u32) as (u64)
               ) as (u64) & 0x8080808080000000u64) as (u64);
    flipped = flipped | (P as (u32) << 8i32 & 0x8000u32 & O as (u32)) as (u64);
    outflank_h = OUTFLANK_7[(
                     O >> 17i32 & 0x3fu64
                 ) as usize ] as (u32) & P as (u32) >> 16i32;
    flipped = flipped | ((outflank_h.wrapping_neg(
                          ) & 0x3fu32) << 17i32) as (u64);
    outflank_d7 = ((O as (u64) | !0x408102040000000u64).wrapping_add(
                       0x40000000u64
                   ) & P as (u64) & 0x408102000000000u64) as (u64);
    flipped = (flipped as (u64) | outflank_d7.wrapping_sub(
                                      (outflank_d7 != 0u64) as (u32) as (u64)
                                  ) as (u64) & 0x8102040000000u64) as (u64);
    flipped = flipped | (P as (u32) << 9i32 & 0x4000u32 & O as (u32)) as (u64);
    flipped
}

fn flip_A4(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut flip_d7 : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_3[(
                     ((O as (u32) & 0x1010100u32).wrapping_add(
                          ((O >> 32i32) as (u32) & 0x10101u32) << 4i32
                      ).wrapping_mul(
                          0x1020408u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x1010101u32).wrapping_add(
                                  ((P >> 32i32) as (u32) & 0x1010101u32) << 4i32
                              ).wrapping_mul(
                                  0x1020408u32
                              ) >> 24i32;
    flipped = (FLIPPED_3_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x1010101010100u64) as (u64);
    outflank_h = ((O & 0x7e000000u64).wrapping_add(
                      0x2000000u64
                  ) & P) as (u32);
    flipped = flipped | (outflank_h.wrapping_sub(
                             outflank_h >> 8i32
                         ) & 0x7e000000u32) as (u64);
    flip_d7 = O as (u32) & 0x20000u32;
    flip_d7 = flip_d7 | flip_d7 >> 7i32 & O as (u32);
    flipped = flipped | (flip_d7 & (flip_d7 & P as (u32) << 7i32).wrapping_neg(
                                   )) as (u64);
    outflank_d9 = ((O >> 32i32) as (u32) | !0x10080402u32).wrapping_add(
                      0x2u32
                  ) & (P >> 32i32) as (u32) & 0x10080400u32;
    flipped = flipped | (outflank_d9.wrapping_sub(
                             (outflank_d9 != 0u32) as (u32)
                         ) & 0x80402u32) as (u64) << 32i32;
    flipped
}

fn flip_B4(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut flip_d7 : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_3[(
                     ((O as (u32) & 0x2020200u32).wrapping_add(
                          ((O >> 32i32) as (u32) & 0x20202u32) << 4i32
                      ).wrapping_mul(
                          0x810204u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x2020202u32).wrapping_add(
                                  ((P >> 32i32) as (u32) & 0x2020202u32) << 4i32
                              ).wrapping_mul(
                                  0x810204u32
                              ) >> 24i32;
    flipped = (FLIPPED_3_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x2020202020200u64) as (u64);
    outflank_h = ((O & 0x7c000000u64).wrapping_add(
                      0x4000000u64
                  ) & P) as (u32);
    flipped = flipped | (outflank_h.wrapping_sub(
                             outflank_h >> 8i32
                         ) & 0x7c000000u32) as (u64);
    flip_d7 = O as (u32) & 0x40000u32;
    flip_d7 = flip_d7 | flip_d7 >> 7i32 & O as (u32);
    flipped = flipped | (flip_d7 & (flip_d7 & P as (u32) << 7i32).wrapping_neg(
                                   )) as (u64);
    outflank_d9 = ((O >> 32i32) as (u32) | !0x20100804u32).wrapping_add(
                      0x4u32
                  ) & (P >> 32i32) as (u32) & 0x20100800u32;
    flipped = flipped | (outflank_d9.wrapping_sub(
                             (outflank_d9 != 0u32) as (u32)
                         ) & 0x100804u32) as (u64) << 32i32;
    flipped
}

fn flip_C4(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_3[(
                     ((O as (u32) & 0x4040400u32).wrapping_add(
                          ((O >> 32i32) as (u32) & 0x40404u32) << 4i32
                      ).wrapping_mul(
                          0x408102u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x4040404u32).wrapping_add(
                                  ((P >> 32i32) as (u32) & 0x4040404u32) << 4i32
                              ).wrapping_mul(
                                  0x408102u32
                              ) >> 24i32;
    flipped = (FLIPPED_3_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x4040404040400u64) as (u64);
    outflank_h = OUTFLANK_2[(
                     O >> 25i32 & 0x3fu64
                 ) as usize ] as (u32) & P as (u32) >> 24i32;
    flipped = flipped | (FLIPPED_2_H[(
                             outflank_h as (usize) ) as usize ] as (u32) & 0xff000000u32) as (u64);
    outflank_d7 = OUTFLANK_2[(
                      ((O as (u32) & 0x4081000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x2u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x4081020u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x102u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_2_H[(
                                      outflank_d7 as (usize) ) as usize ] as (u64) & 0x204081000u64) as (u64);
    outflank_d9 = OUTFLANK_2[(
                      ((O as (u32) & 0x4020000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x201008u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x4020100u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x40201008u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_2_H[(
                                      outflank_d9 as (usize) ) as usize ] as (u64) & 0x20100804020000u64) as (u64);
    flipped
}

fn flip_D4(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_3[(
                     ((O as (u32) & 0x8080800u32).wrapping_add(
                          ((O >> 32i32) as (u32) & 0x80808u32) << 4i32
                      ).wrapping_mul(
                          0x204081u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x8080808u32).wrapping_add(
                                  ((P >> 32i32) as (u32) & 0x8080808u32) << 4i32
                              ).wrapping_mul(
                                  0x204081u32
                              ) >> 24i32;
    flipped = (FLIPPED_3_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x8080808080800u64) as (u64);
    outflank_h = OUTFLANK_3[(
                     O >> 25i32 & 0x3fu64
                 ) as usize ] as (u32) & P as (u32) >> 24i32;
    flipped = flipped | (FLIPPED_3_H[(
                             outflank_h as (usize) ) as usize ] as (u32) & 0xff000000u32) as (u64);
    outflank_d7 = OUTFLANK_3[(
                      ((O as (u32) & 0x8102000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x204u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x8102040u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x10204u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_3_H[(
                                      outflank_d7 as (usize) ) as usize ] as (u64) & 0x20408102000u64) as (u64);
    outflank_d9 = OUTFLANK_3[(
                      ((O as (u32) & 0x8040200u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x402010u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x8040201u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x80402010u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_3_H[(
                                      outflank_d9 as (usize) ) as usize ] as (u64) & 0x40201008040200u64) as (u64);
    flipped
}

fn flip_E4(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_3[(
                     (((O as (u32) & 0x10101000u32) >> 4i32).wrapping_add(
                          (O >> 32i32) as (u32) & 0x101010u32
                      ).wrapping_mul(
                          0x1020408u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P as (u32) & 0x10101010u32) >> 4i32).wrapping_add(
                                  (P >> 32i32) as (u32) & 0x10101010u32
                              ).wrapping_mul(
                                  0x1020408u32
                              ) >> 24i32;
    flipped = (FLIPPED_3_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x10101010101000u64) as (u64);
    outflank_h = OUTFLANK_4[(
                     O >> 25i32 & 0x3fu64
                 ) as usize ] as (u32) & P as (u32) >> 24i32;
    flipped = flipped | (FLIPPED_4_H[(
                             outflank_h as (usize) ) as usize ] as (u32) & 0xff000000u32) as (u64);
    outflank_d7 = OUTFLANK_4[(
                      ((O as (u32) & 0x10204000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x20408u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x10204080u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x1020408u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_4_H[(
                                      outflank_d7 as (usize) ) as usize ] as (u64) & 0x2040810204000u64) as (u64);
    outflank_d9 = OUTFLANK_4[(
                      ((O as (u32) & 0x10080400u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x4020u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x10080402u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x804020u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_4_H[(
                                      outflank_d9 as (usize) ) as usize ] as (u64) & 0x402010080400u64) as (u64);
    flipped
}

fn flip_F4(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_3[(
                     (((O as (u32) & 0x20202000u32) >> 4i32).wrapping_add(
                          (O >> 32i32) as (u32) & 0x202020u32
                      ).wrapping_mul(
                          0x810204u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P as (u32) & 0x20202020u32) >> 4i32).wrapping_add(
                                  (P >> 32i32) as (u32) & 0x20202020u32
                              ).wrapping_mul(
                                  0x810204u32
                              ) >> 24i32;
    flipped = (FLIPPED_3_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x20202020202000u64) as (u64);
    outflank_h = OUTFLANK_5[(
                     O >> 25i32 & 0x3fu64
                 ) as usize ] as (u32) & P as (u32) >> 24i32;
    flipped = flipped | (FLIPPED_5_H[(
                             outflank_h as (usize) ) as usize ] as (u32) & 0xff000000u32) as (u64);
    outflank_d7 = OUTFLANK_5[(
                      ((O as (u32) & 0x20400000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x40810u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x20408000u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x2040810u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_5_H[(
                                      outflank_d7 as (usize) ) as usize ] as (u64) & 0x4081020400000u64) as (u64);
    outflank_d9 = OUTFLANK_5[(
                      ((O as (u32) & 0x20100800u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x40u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x20100804u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x8040u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_5_H[(
                                      outflank_d9 as (usize) ) as usize ] as (u64) & 0x4020100800u64) as (u64);
    flipped
}

fn flip_G4(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut flip_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_3[(
                     (((O as (u32) & 0x40404000u32) >> 4i32).wrapping_add(
                          (O >> 32i32) as (u32) & 0x404040u32
                      ).wrapping_mul(
                          0x408102u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P as (u32) & 0x40404040u32) >> 4i32).wrapping_add(
                                  (P >> 32i32) as (u32) & 0x40404040u32
                              ).wrapping_mul(
                                  0x408102u32
                              ) >> 24i32;
    flipped = (FLIPPED_3_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x40404040404000u64) as (u64);
    outflank_h = OUTFLANK_7[(
                     O >> 24i32 & 0x3eu64
                 ) as usize ] as (u32) & P as (u32) >> 23i32;
    flipped = flipped | ((outflank_h.wrapping_neg(
                          ) & 0x3eu32) << 24i32) as (u64);
    outflank_d7 = ((O >> 32i32) as (u32) | !0x4081020u32).wrapping_add(
                      0x20u32
                  ) & (P >> 32i32) as (u32) & 0x4081000u32;
    flipped = flipped | (outflank_d7.wrapping_sub(
                             (outflank_d7 != 0u32) as (u32)
                         ) & 0x81020u32) as (u64) << 32i32;
    flip_d9 = O as (u32) & 0x200000u32;
    flip_d9 = flip_d9 | flip_d9 >> 9i32 & O as (u32);
    flipped = flipped | (flip_d9 & (flip_d9 & P as (u32) << 9i32).wrapping_neg(
                                   )) as (u64);
    flipped
}

fn flip_H4(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut flip_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_3[(
                     (((O as (u32) & 0x80808000u32) >> 4i32).wrapping_add(
                          (O >> 32i32) as (u32) & 0x808080u32
                      ).wrapping_mul(
                          0x204081u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P as (u32) & 0x80808080u32) >> 4i32).wrapping_add(
                                  (P >> 32i32) as (u32) & 0x80808080u32
                              ).wrapping_mul(
                                  0x204081u32
                              ) >> 24i32;
    flipped = (FLIPPED_3_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x80808080808000u64) as (u64);
    outflank_h = OUTFLANK_7[(
                     O >> 25i32 & 0x3fu64
                 ) as usize ] as (u32) & P as (u32) >> 24i32;
    flipped = flipped | ((outflank_h.wrapping_neg(
                          ) & 0x3fu32) << 25i32) as (u64);
    outflank_d7 = ((O >> 32i32) as (u32) | !0x8102040u32).wrapping_add(
                      0x40u32
                  ) & (P >> 32i32) as (u32) & 0x8102000u32;
    flipped = flipped | (outflank_d7.wrapping_sub(
                             (outflank_d7 != 0u32) as (u32)
                         ) & 0x102040u32) as (u64) << 32i32;
    flip_d9 = O as (u32) & 0x400000u32;
    flip_d9 = flip_d9 | flip_d9 >> 9i32 & O as (u32);
    flipped = flipped | (flip_d9 & (flip_d9 & P as (u32) << 9i32).wrapping_neg(
                                   )) as (u64);
    flipped
}

fn flip_A5(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut flip_d7 : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_4[(
                     ((O as (u32) & 0x1010100u32).wrapping_add(
                          ((O >> 32i32) as (u32) & 0x10101u32) << 4i32
                      ).wrapping_mul(
                          0x1020408u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x1010101u32).wrapping_add(
                                  ((P >> 32i32) as (u32) & 0x1010101u32) << 4i32
                              ).wrapping_mul(
                                  0x1020408u32
                              ) >> 24i32;
    flipped = (FLIPPED_4_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x1010101010100u64) as (u64);
    outflank_h = ((O >> 32i32) as (u32) & 0x7eu32).wrapping_add(
                     0x2u32
                 ) & (P >> 32i32) as (u32);
    flipped = flipped | ((outflank_h << 8i32).wrapping_sub(
                             outflank_h
                         ) >> 8i32 & 0x7eu32) as (u64) << 32i32;
    flip_d7 = O as (u32) & (O as (u32) >> 7i32 | 0x2000000u32);
    flip_d7 = flip_d7 & ((flip_d7 & 0x2040000u32) >> 14i32 | 0x2040000u32);
    flipped = flipped | (flip_d7 & (flip_d7 & P as (u32) << 7i32).wrapping_neg(
                                   )) as (u64);
    outflank_d9 = ((O >> 32i32) as (u32) | !0x8040200u32).wrapping_add(
                      0x200u32
                  ) & (P >> 32i32) as (u32) & 0x8040000u32;
    flipped = flipped | (outflank_d9.wrapping_sub(
                             (outflank_d9 != 0u32) as (u32)
                         ) & 0x40200u32) as (u64) << 32i32;
    flipped
}

fn flip_B5(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut flip_d7 : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_4[(
                     ((O as (u32) & 0x2020200u32).wrapping_add(
                          ((O >> 32i32) as (u32) & 0x20202u32) << 4i32
                      ).wrapping_mul(
                          0x810204u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x2020202u32).wrapping_add(
                                  ((P >> 32i32) as (u32) & 0x2020202u32) << 4i32
                              ).wrapping_mul(
                                  0x810204u32
                              ) >> 24i32;
    flipped = (FLIPPED_4_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x2020202020200u64) as (u64);
    outflank_h = (((O >> 32i32) as (u32) & 0x7cu32).wrapping_add(
                      0x4u32
                  ) as (u64) & P >> 32i32) as (u32);
    flipped = flipped | ((outflank_h << 8i32).wrapping_sub(
                             outflank_h
                         ) >> 8i32 & 0x7cu32) as (u64) << 32i32;
    flip_d7 = O as (u32) & (O as (u32) >> 7i32 | 0x4000000u32);
    flip_d7 = flip_d7 & ((flip_d7 & 0x4080000u32) >> 14i32 | 0x4080000u32);
    flipped = flipped | (flip_d7 & (flip_d7 & P as (u32) << 7i32).wrapping_neg(
                                   )) as (u64);
    outflank_d9 = ((O >> 32i32) as (u32) | !0x10080400u32).wrapping_add(
                      0x400u32
                  ) & (P >> 32i32) as (u32) & 0x10080000u32;
    flipped = flipped | (outflank_d9.wrapping_sub(
                             (outflank_d9 != 0u32) as (u32)
                         ) & 0x80400u32) as (u64) << 32i32;
    flipped
}

fn flip_C5(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_4[(
                     ((O as (u32) & 0x4040400u32).wrapping_add(
                          ((O >> 32i32) as (u32) & 0x40404u32) << 4i32
                      ).wrapping_mul(
                          0x408102u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x4040404u32).wrapping_add(
                                  ((P >> 32i32) as (u32) & 0x4040404u32) << 4i32
                              ).wrapping_mul(
                                  0x408102u32
                              ) >> 24i32;
    flipped = (FLIPPED_4_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x4040404040400u64) as (u64);
    outflank_h = (OUTFLANK_2[(
                      O >> 33i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 32i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_2_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff00000000u64) as (u64);
    outflank_d7 = OUTFLANK_2[(
                      ((O as (u32) & 0x8102000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x204u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x8102040u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x10204u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_2_H[(
                                      outflank_d7 as (usize) ) as usize ] as (u64) & 0x20408102000u64) as (u64);
    outflank_d9 = OUTFLANK_2[(
                      ((O as (u32) & 0x2000000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x100804u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x2010000u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x20100804u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_2_H[(
                                      outflank_d9 as (usize) ) as usize ] as (u64) & 0x10080402000000u64) as (u64);
    flipped
}

fn flip_D5(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_4[(
                     ((O as (u32) & 0x8080800u32).wrapping_add(
                          ((O >> 32i32) as (u32) & 0x80808u32) << 4i32
                      ).wrapping_mul(
                          0x204081u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x8080808u32).wrapping_add(
                                  ((P >> 32i32) as (u32) & 0x8080808u32) << 4i32
                              ).wrapping_mul(
                                  0x204081u32
                              ) >> 24i32;
    flipped = (FLIPPED_4_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x8080808080800u64) as (u64);
    outflank_h = (OUTFLANK_3[(
                      O >> 33i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 32i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_3_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff00000000u64) as (u64);
    outflank_d7 = OUTFLANK_3[(
                      ((O as (u32) & 0x10204000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x20408u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x10204080u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x1020408u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_3_H[(
                                      outflank_d7 as (usize) ) as usize ] as (u64) & 0x2040810204000u64) as (u64);
    outflank_d9 = OUTFLANK_3[(
                      ((O as (u32) & 0x4020000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x201008u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x4020100u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x40201008u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_3_H[(
                                      outflank_d9 as (usize) ) as usize ] as (u64) & 0x20100804020000u64) as (u64);
    flipped
}

fn flip_E5(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_4[(
                     (((O as (u32) & 0x10101000u32) >> 4i32).wrapping_add(
                          (O >> 32i32) as (u32) & 0x101010u32
                      ).wrapping_mul(
                          0x1020408u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P as (u32) & 0x10101010u32) >> 4i32).wrapping_add(
                                  (P >> 32i32) as (u32) & 0x10101010u32
                              ).wrapping_mul(
                                  0x1020408u32
                              ) >> 24i32;
    flipped = (FLIPPED_4_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x10101010101000u64) as (u64);
    outflank_h = (OUTFLANK_4[(
                      O >> 33i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 32i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_4_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff00000000u64) as (u64);
    outflank_d7 = OUTFLANK_4[(
                      ((O as (u32) & 0x20400000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x40810u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x20408000u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x2040810u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_4_H[(
                                      outflank_d7 as (usize) ) as usize ] as (u64) & 0x4081020400000u64) as (u64);
    outflank_d9 = OUTFLANK_4[(
                      ((O as (u32) & 0x8040200u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x402010u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x8040201u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x80402010u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_4_H[(
                                      outflank_d9 as (usize) ) as usize ] as (u64) & 0x40201008040200u64) as (u64);
    flipped
}

fn flip_F5(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_4[(
                     (((O as (u32) & 0x20202000u32) >> 4i32).wrapping_add(
                          (O >> 32i32) as (u32) & 0x202020u32
                      ).wrapping_mul(
                          0x810204u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P as (u32) & 0x20202020u32) >> 4i32).wrapping_add(
                                  (P >> 32i32) as (u32) & 0x20202020u32
                              ).wrapping_mul(
                                  0x810204u32
                              ) >> 24i32;
    flipped = (FLIPPED_4_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x20202020202000u64) as (u64);
    outflank_h = (OUTFLANK_5[(
                      O >> 33i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 32i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_5_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff00000000u64) as (u64);
    outflank_d7 = OUTFLANK_5[(
                      ((O as (u32) & 0x40000000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x81020u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x40800000u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x4081020u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_5_H[(
                                      outflank_d7 as (usize) ) as usize ] as (u64) & 0x8102040000000u64) as (u64);
    outflank_d9 = OUTFLANK_5[(
                      ((O as (u32) & 0x10080400u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x4020u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x10080402u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x804020u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_5_H[(
                                      outflank_d9 as (usize) ) as usize ] as (u64) & 0x402010080400u64) as (u64);
    flipped
}

fn flip_G5(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut flip_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_4[(
                     (((O as (u32) & 0x40404000u32) >> 4i32).wrapping_add(
                          (O >> 32i32) as (u32) & 0x404040u32
                      ).wrapping_mul(
                          0x408102u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P as (u32) & 0x40404040u32) >> 4i32).wrapping_add(
                                  (P >> 32i32) as (u32) & 0x40404040u32
                              ).wrapping_mul(
                                  0x408102u32
                              ) >> 24i32;
    flipped = (FLIPPED_4_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x40404040404000u64) as (u64);
    outflank_h = (OUTFLANK_7[(
                      O >> 32i32 & 0x3eu64
                  ) as usize ] as (u64) & P >> 31i32) as (u32);
    flipped = flipped | (outflank_h.wrapping_neg(
                         ) & 0x3eu32) as (u64) << 32i32;
    outflank_d7 = ((O >> 32i32) as (u32) | !0x8102000u32).wrapping_add(
                      0x2000u32
                  ) & (P >> 32i32) as (u32) & 0x8100000u32;
    flipped = flipped | (outflank_d7.wrapping_sub(
                             (outflank_d7 != 0u32) as (u32)
                         ) & 0x102000u32) as (u64) << 32i32;
    flip_d9 = O as (u32) & (O as (u32) >> 9i32 | 0x20000000u32);
    flip_d9 = flip_d9 & ((flip_d9 & 0x20100000u32) >> 18i32 | 0x20100000u32);
    flipped = flipped | (flip_d9 & (flip_d9 & P as (u32) << 9i32).wrapping_neg(
                                   )) as (u64);
    flipped
}

fn flip_H5(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut flip_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_4[(
                     (((O as (u32) & 0x80808000u32) >> 4i32).wrapping_add(
                          (O >> 32i32) as (u32) & 0x808080u32
                      ).wrapping_mul(
                          0x204081u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P as (u32) & 0x80808080u32) >> 4i32).wrapping_add(
                                  (P >> 32i32) as (u32) & 0x80808080u32
                              ).wrapping_mul(
                                  0x204081u32
                              ) >> 24i32;
    flipped = (FLIPPED_4_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x80808080808000u64) as (u64);
    outflank_h = (OUTFLANK_7[(
                      O >> 33i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 32i32) as (u32);
    flipped = flipped | (outflank_h.wrapping_neg(
                         ) & 0x3fu32) as (u64) << 33i32;
    outflank_d7 = ((O >> 32i32) as (u32) | !0x10204000u32).wrapping_add(
                      0x4000u32
                  ) & (P >> 32i32) as (u32) & 0x10200000u32;
    flipped = flipped | (outflank_d7.wrapping_sub(
                             (outflank_d7 != 0u32) as (u32)
                         ) & 0x204000u32) as (u64) << 32i32;
    flip_d9 = O as (u32) & (O as (u32) >> 9i32 | 0x40000000u32);
    flip_d9 = flip_d9 & ((flip_d9 & 0x40200000u32) >> 18i32 | 0x40200000u32);
    flipped = flipped | (flip_d9 & (flip_d9 & P as (u32) << 9i32).wrapping_neg(
                                   )) as (u64);
    flipped
}

fn flip_A6(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut flip_d7 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_5[(
                     ((O as (u32) & 0x1010100u32).wrapping_add(
                          ((O >> 32i32) as (u32) & 0x10101u32) << 4i32
                      ).wrapping_mul(
                          0x1020408u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x1010101u32).wrapping_add(
                                  ((P >> 32i32) as (u32) & 0x1010101u32) << 4i32
                              ).wrapping_mul(
                                  0x1020408u32
                              ) >> 24i32;
    flipped = (FLIPPED_5_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x1010101010100u64) as (u64);
    outflank_h = ((O >> 32i32) as (u32) & 0x7e00u32).wrapping_add(
                     0x200u32
                 ) & (P >> 32i32) as (u32);
    flipped = flipped | (outflank_h.wrapping_sub(
                             outflank_h >> 8i32
                         ) & 0x7e00u32) as (u64) << 32i32;
    flip_d7 = (O >> 8i32) as (u32);
    flip_d7 = flip_d7 & (flip_d7 >> 7i32 | 0x2000000u32);
    flip_d7 = flip_d7 & ((flip_d7 & 0x2040000u32) >> 14i32 | 0x2040000u32);
    flipped = flipped | (flip_d7 & (flip_d7 & P as (u32) >> 1i32).wrapping_neg(
                                   )) as (u64) << 8i32;
    flipped = flipped | ((P >> 32i32) as (u32) >> 9i32 & 0x20000u32 & (O >> 32i32) as (u32)) as (u64) << 32i32;
    flipped
}

fn flip_B6(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut flip_d7 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_5[(
                     ((O as (u32) & 0x2020200u32).wrapping_add(
                          ((O >> 32i32) as (u32) & 0x20202u32) << 4i32
                      ).wrapping_mul(
                          0x810204u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x2020202u32).wrapping_add(
                                  ((P >> 32i32) as (u32) & 0x2020202u32) << 4i32
                              ).wrapping_mul(
                                  0x810204u32
                              ) >> 24i32;
    flipped = (FLIPPED_5_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x2020202020200u64) as (u64);
    outflank_h = ((O >> 32i32) as (u32) & 0x7c00u32).wrapping_add(
                     0x400u32
                 ) & (P >> 32i32) as (u32);
    flipped = flipped | (outflank_h.wrapping_sub(
                             outflank_h >> 8i32
                         ) & 0x7c00u32) as (u64) << 32i32;
    flip_d7 = (O >> 8i32) as (u32);
    flip_d7 = flip_d7 & (flip_d7 >> 7i32 | 0x4000000u32);
    flip_d7 = flip_d7 & ((flip_d7 & 0x4080000u32) >> 14i32 | 0x4080000u32);
    flipped = flipped | (flip_d7 & (flip_d7 & P as (u32) >> 1i32).wrapping_neg(
                                   )) as (u64) << 8i32;
    flipped = flipped | ((P >> 32i32) as (u32) >> 9i32 & 0x40000u32 & (O >> 32i32) as (u32)) as (u64) << 32i32;
    flipped
}

fn flip_C6(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_5[(
                     ((O as (u32) & 0x4040400u32).wrapping_add(
                          ((O >> 32i32) as (u32) & 0x40404u32) << 4i32
                      ).wrapping_mul(
                          0x408102u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x4040404u32).wrapping_add(
                                  ((P >> 32i32) as (u32) & 0x4040404u32) << 4i32
                              ).wrapping_mul(
                                  0x408102u32
                              ) >> 24i32;
    flipped = (FLIPPED_5_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x4040404040400u64) as (u64);
    outflank_h = (OUTFLANK_2[(
                      O >> 41i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 40i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_2_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff0000000000u64) as (u64);
    outflank_d7 = OUTFLANK_2[(
                      ((O as (u32) & 0x10204000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x20408u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x10204080u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x1020408u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_2_H[(
                                      outflank_d7 as (usize) ) as usize ] as (u64) & 0x2040810204000u64) as (u64);
    flipped = flipped | (((P >> 32i32) as (u32) >> 9i32 | P as (u32) >> 23i32) & 0x80002u32 & (O >> 32i32) as (u32)) as (u64) << 32i32;
    flipped
}

fn flip_D6(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_5[(
                     ((O as (u32) & 0x8080800u32).wrapping_add(
                          ((O >> 32i32) as (u32) & 0x80808u32) << 4i32
                      ).wrapping_mul(
                          0x204081u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x8080808u32).wrapping_add(
                                  ((P >> 32i32) as (u32) & 0x8080808u32) << 4i32
                              ).wrapping_mul(
                                  0x204081u32
                              ) >> 24i32;
    flipped = (FLIPPED_5_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x8080808080800u64) as (u64);
    outflank_h = (OUTFLANK_3[(
                      O >> 41i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 40i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_3_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff0000000000u64) as (u64);
    outflank_d7 = OUTFLANK_3[(
                      ((O as (u32) & 0x20400000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x40810u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x20408000u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x2040810u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_3_H[(
                                      outflank_d7 as (usize) ) as usize ] as (u64) & 0x4081020400000u64) as (u64);
    outflank_d9 = OUTFLANK_3[(
                      ((O as (u32) & 0x2000000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x100804u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x2010000u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x20100804u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_3_H[(
                                      outflank_d9 as (usize) ) as usize ] as (u64) & 0x10080402000000u64) as (u64);
    flipped
}

fn flip_E6(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d7 : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_5[(
                     (((O as (u32) & 0x10101000u32) >> 4i32).wrapping_add(
                          (O >> 32i32) as (u32) & 0x101010u32
                      ).wrapping_mul(
                          0x1020408u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P as (u32) & 0x10101010u32) >> 4i32).wrapping_add(
                                  (P >> 32i32) as (u32) & 0x10101010u32
                              ).wrapping_mul(
                                  0x1020408u32
                              ) >> 24i32;
    flipped = (FLIPPED_5_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x10101010101000u64) as (u64);
    outflank_h = (OUTFLANK_4[(
                      O >> 41i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 40i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_4_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff0000000000u64) as (u64);
    outflank_d7 = OUTFLANK_4[(
                      ((O as (u32) & 0x40000000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x81020u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x40800000u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x4081020u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_4_H[(
                                      outflank_d7 as (usize) ) as usize ] as (u64) & 0x8102040000000u64) as (u64);
    outflank_d9 = OUTFLANK_4[(
                      ((O as (u32) & 0x4020100u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x201008u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x4020100u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x40201008u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_4_H[(
                                      outflank_d9 as (usize) ) as usize ] as (u64) & 0x20100804020100u64) as (u64);
    flipped
}

fn flip_F6(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_5[(
                     (((O as (u32) & 0x20202000u32) >> 4i32).wrapping_add(
                          (O >> 32i32) as (u32) & 0x202020u32
                      ).wrapping_mul(
                          0x810204u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P as (u32) & 0x20202020u32) >> 4i32).wrapping_add(
                                  (P >> 32i32) as (u32) & 0x20202020u32
                              ).wrapping_mul(
                                  0x810204u32
                              ) >> 24i32;
    flipped = (FLIPPED_5_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x20202020202000u64) as (u64);
    outflank_h = (OUTFLANK_5[(
                      O >> 41i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 40i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_5_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff0000000000u64) as (u64);
    flipped = flipped | (((P >> 32i32) as (u32) >> 7i32 | P as (u32) >> 25i32) & 0x100040u32 & (O >> 32i32) as (u32)) as (u64) << 32i32;
    outflank_d9 = OUTFLANK_5[(
                      ((O as (u32) & 0x8040200u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x402010u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x8040201u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x80402010u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_5_H[(
                                      outflank_d9 as (usize) ) as usize ] as (u64) & 0x40201008040200u64) as (u64);
    flipped
}

fn flip_G6(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut flip_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_5[(
                     (((O as (u32) & 0x40404000u32) >> 4i32).wrapping_add(
                          (O >> 32i32) as (u32) & 0x404040u32
                      ).wrapping_mul(
                          0x408102u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P as (u32) & 0x40404040u32) >> 4i32).wrapping_add(
                                  (P >> 32i32) as (u32) & 0x40404040u32
                              ).wrapping_mul(
                                  0x408102u32
                              ) >> 24i32;
    flipped = (FLIPPED_5_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x40404040404000u64) as (u64);
    outflank_h = (OUTFLANK_7[(
                      O >> 40i32 & 0x3eu64
                  ) as usize ] as (u64) & P >> 39i32) as (u32);
    flipped = flipped | (outflank_h.wrapping_neg(
                         ) & 0x3eu32) as (u64) << 40i32;
    flipped = flipped | ((P >> 32i32) as (u32) >> 7i32 & 0x200000u32 & (O >> 32i32) as (u32)) as (u64) << 32i32;
    flip_d9 = (O >> 8i32) as (u32);
    flip_d9 = flip_d9 & (flip_d9 >> 9i32 | 0x20000000u32);
    flip_d9 = flip_d9 & ((flip_d9 & 0x20100000u32) >> 18i32 | 0x20100000u32);
    flipped = flipped | (flip_d9 & (flip_d9 & P as (u32) << 1i32).wrapping_neg(
                                   )) as (u64) << 8i32;
    flipped
}

fn flip_H6(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut flip_d9 : u32;
    let mut flipped : u64;
    outflank_v = OUTFLANK_5[(
                     (((O as (u32) & 0x80808000u32) >> 4i32).wrapping_add(
                          (O >> 32i32) as (u32) & 0x808080u32
                      ).wrapping_mul(
                          0x204081u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P as (u32) & 0x80808080u32) >> 4i32).wrapping_add(
                                  (P >> 32i32) as (u32) & 0x80808080u32
                              ).wrapping_mul(
                                  0x204081u32
                              ) >> 24i32;
    flipped = (FLIPPED_5_V[(
                   outflank_v as (usize) ) as usize ] as (u64) & 0x80808080808000u64) as (u64);
    outflank_h = (OUTFLANK_7[(
                      O >> 41i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 40i32) as (u32);
    flipped = flipped | (outflank_h.wrapping_neg(
                         ) & 0x3fu32) as (u64) << 41i32;
    flipped = flipped | ((P >> 32i32) as (u32) >> 7i32 & 0x400000u32 & (O >> 32i32) as (u32)) as (u64) << 32i32;
    flip_d9 = (O >> 8i32) as (u32);
    flip_d9 = flip_d9 & (flip_d9 >> 9i32 | 0x40000000u32);
    flip_d9 = flip_d9 & ((flip_d9 & 0x40200000u32) >> 18i32 | 0x40200000u32);
    flipped = flipped | (flip_d9 & (flip_d9 & P as (u32) << 1i32).wrapping_neg(
                                   )) as (u64) << 8i32;
    flipped
}

fn flip_A7(P : u64, O : u64) -> u64 {
    let mut outflank_v : u32;
    let mut outflank_h : u32;
    let mut outflank_d7 : u32;
    let mut flipped : u64;
    outflank_v = (((O as (u32) & 0x1010100u32) << 4i32).wrapping_add(
                      (O >> 32i32) as (u32) & 0x101u32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     4u32
                 ) & ((P as (u32) & 0x1010101u32) << 4i32).wrapping_add(
                         (P >> 32i32) as (u32) & 0x1u32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0x3e1e0e0602u64
               ) & 0x10101010100u64) as (u64);
    outflank_h = ((O >> 32i32) as (u32) & 0x7e0000u32).wrapping_add(
                     0x20000u32
                 ) & (P >> 32i32) as (u32);
    flipped = flipped | (outflank_h.wrapping_sub(
                             outflank_h >> 8i32
                         ) & 0x7e0000u32) as (u64) << 32i32;
    outflank_d7 = ((O as (u32) & 0x8102000u32).wrapping_add(
                       (O >> 32i32) as (u32) & 0x204u32
                   ).wrapping_mul(
                       0x1010101u32
                   ) >> 24i32).wrapping_add(
                      2u32
                  ) & (P as (u32) & 0x8102040u32).wrapping_add(
                          (P >> 32i32) as (u32) & 0x4u32
                      ).wrapping_mul(
                          0x1010101u32
                      ) >> 24i32;
    flipped = (flipped as (u64) | (outflank_d7 as (u64)).wrapping_mul(
                                      0xf8f0e0c080u64
                                  ) & 0x20408102000u64) as (u64);
    flipped
}

fn flip_B7(P : u64, O : u64) -> u64 {
    let mut outflank_v : u32;
    let mut outflank_h : u32;
    let mut outflank_d7 : u32;
    let mut flipped : u64;
    outflank_v = (((O as (u32) & 0x2020200u32) << 3i32).wrapping_add(
                      ((O >> 32i32) as (u32) & 0x202u32) >> 1i32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     4u32
                 ) & ((P as (u32) & 0x2020202u32) << 3i32).wrapping_add(
                         ((P >> 32i32) as (u32) & 0x2u32) >> 1i32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0x7c3c1c0c04u64
               ) & 0x20202020200u64) as (u64);
    outflank_h = ((O >> 32i32) as (u32) & 0x7c0000u32).wrapping_add(
                     0x40000u32
                 ) & (P >> 32i32) as (u32);
    flipped = flipped | (outflank_h.wrapping_sub(
                             outflank_h >> 8i32
                         ) & 0x7c0000u32) as (u64) << 32i32;
    outflank_d7 = ((O as (u32) & 0x10204000u32).wrapping_add(
                       (O >> 32i32) as (u32) & 0x408u32
                   ).wrapping_mul(
                       0x1010101u32
                   ) >> 25i32).wrapping_add(
                      2u32
                  ) & (P as (u32) & 0x10204080u32).wrapping_add(
                          (P >> 32i32) as (u32) & 0x8u32
                      ).wrapping_mul(
                          0x1010101u32
                      ) >> 25i32;
    flipped = (flipped as (u64) | (outflank_d7 as (u64)).wrapping_mul(
                                      0x1f1e1c18100u64
                                  ) & 0x40810204000u64) as (u64);
    flipped
}

fn flip_C7(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d : u32;
    let mut flipped : u64;
    outflank_v = (((O as (u32) & 0x4040400u32) << 2i32).wrapping_add(
                      ((O >> 32i32) as (u32) & 0x404u32) >> 2i32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     4u32
                 ) & ((P as (u32) & 0x4040404u32) << 2i32).wrapping_add(
                         ((P >> 32i32) as (u32) & 0x4u32) >> 2i32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0xf878381808u64
               ) & 0x40404040400u64) as (u64);
    outflank_h = (OUTFLANK_2[(
                      O >> 49i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 48i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_2_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff000000000000u64) as (u64);
    outflank_d = OUTFLANK_2[(
                     (((O >> 32i32) as (u32) & 0xa10u32).wrapping_add(
                          O as (u32) & 0x20400000u32
                      ).wrapping_mul(
                          0x1010101u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P >> 32i32) as (u32) & 0xa11u32).wrapping_add(
                                  P as (u32) & 0x20408000u32
                              ).wrapping_mul(
                                  0x1010101u32
                              ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_2_H[(
                                      outflank_d as (usize) ) as usize ] as (u64) & 0xa1020400000u64) as (u64);
    flipped
}

fn flip_D7(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d : u32;
    let mut flipped : u64;
    outflank_v = (((O as (u32) & 0x8080800u32) << 1i32).wrapping_add(
                      ((O >> 32i32) as (u32) & 0x808u32) >> 3i32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     4u32
                 ) & ((P as (u32) & 0x8080808u32) << 1i32).wrapping_add(
                         ((P >> 32i32) as (u32) & 0x8u32) >> 3i32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0x1f0f0703010u64
               ) & 0x80808080800u64) as (u64);
    outflank_h = (OUTFLANK_3[(
                      O >> 49i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 48i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_3_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff000000000000u64) as (u64);
    outflank_d = OUTFLANK_3[(
                     (((O >> 32i32) as (u32) & 0x1422u32).wrapping_add(
                          O as (u32) & 0x40000000u32
                      ).wrapping_mul(
                          0x1010101u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P >> 32i32) as (u32) & 0x1422u32).wrapping_add(
                                  P as (u32) & 0x41800000u32
                              ).wrapping_mul(
                                  0x1010101u32
                              ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_3_H[(
                                      outflank_d as (usize) ) as usize ] as (u64) & 0x142240000000u64) as (u64);
    flipped
}

fn flip_E7(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d : u32;
    let mut flipped : u64;
    outflank_v = ((O as (u32) & 0x10101000u32).wrapping_add(
                      ((O >> 32i32) as (u32) & 0x1010u32) >> 4i32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     4u32
                 ) & (P as (u32) & 0x10101010u32).wrapping_add(
                         ((P >> 32i32) as (u32) & 0x10u32) >> 4i32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0x3e1e0e06020u64
               ) & 0x101010101000u64) as (u64);
    outflank_h = (OUTFLANK_4[(
                      O >> 49i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 48i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_4_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff000000000000u64) as (u64);
    outflank_d = OUTFLANK_4[(
                     (((O >> 32i32) as (u32) & 0x2844u32).wrapping_add(
                          O as (u32) & 0x2000000u32
                      ).wrapping_mul(
                          0x1010101u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P >> 32i32) as (u32) & 0x2844u32).wrapping_add(
                                  P as (u32) & 0x82010000u32
                              ).wrapping_mul(
                                  0x1010101u32
                              ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_4_H[(
                                      outflank_d as (usize) ) as usize ] as (u64) & 0x284402000000u64) as (u64);
    flipped
}

fn flip_F7(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d : u32;
    let mut flipped : u64;
    outflank_v = (((O as (u32) & 0x20202000u32) >> 1i32).wrapping_add(
                      ((O >> 32i32) as (u32) & 0x2020u32) >> 5i32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     4u32
                 ) & ((P as (u32) & 0x20202020u32) >> 1i32).wrapping_add(
                         ((P >> 32i32) as (u32) & 0x20u32) >> 5i32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0x7c3c1c0c040u64
               ) & 0x202020202000u64) as (u64);
    outflank_h = (OUTFLANK_5[(
                      O >> 49i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 48i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_5_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff000000000000u64) as (u64);
    outflank_d = OUTFLANK_5[(
                     (((O >> 32i32) as (u32) & 0x5008u32).wrapping_add(
                          O as (u32) & 0x4020000u32
                      ).wrapping_mul(
                          0x1010101u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P >> 32i32) as (u32) & 0x5088u32).wrapping_add(
                                  P as (u32) & 0x4020100u32
                              ).wrapping_mul(
                                  0x1010101u32
                              ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_5_H[(
                                      outflank_d as (usize) ) as usize ] as (u64) & 0x500804020000u64) as (u64);
    flipped
}

fn flip_G7(P : u64, O : u64) -> u64 {
    let mut outflank_v : u32;
    let mut outflank_h : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = (((O as (u32) & 0x40404000u32) >> 2i32).wrapping_add(
                      ((O >> 32i32) as (u32) & 0x4040u32) >> 6i32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     4u32
                 ) & ((P as (u32) & 0x40404040u32) >> 2i32).wrapping_add(
                         ((P >> 32i32) as (u32) & 0x40u32) >> 6i32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0xf8783818080u64
               ) & 0x404040404000u64) as (u64);
    outflank_h = (OUTFLANK_7[(
                      O >> 48i32 & 0x3eu64
                  ) as usize ] as (u64) & P >> 47i32) as (u32);
    flipped = flipped | (outflank_h.wrapping_neg(
                         ) & 0x3eu32) as (u64) << 48i32;
    outflank_d9 = OUTFLANK_7[(
                      ((O as (u32) & 0x8040200u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x2010u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 24i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x8040201u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x10u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 23i32;
    flipped = (flipped as (u64) | (outflank_d9 as (u64)).wrapping_mul(
                                      0x1f0f07030100u64
                                  ) & 0x201008040200u64) as (u64);
    flipped
}

fn flip_H7(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = (((O as (u32) & 0x80808000u32) >> 3i32).wrapping_add(
                      ((O >> 32i32) as (u32) & 0x8080u32) >> 7i32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     4u32
                 ) & ((P as (u32) & 0x80808080u32) >> 3i32).wrapping_add(
                         ((P >> 32i32) as (u32) & 0x80u32) >> 7i32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0x1f0f07030100u64
               ) & 0x808080808000u64) as (u64);
    outflank_h = (OUTFLANK_7[(
                      O >> 49i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 48i32) as (u32);
    flipped = flipped | (outflank_h.wrapping_neg(
                         ) & 0x3fu32) as (u64) << 49i32;
    outflank_d9 = OUTFLANK_7[(
                      ((O as (u32) & 0x10080400u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x4020u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x10080402u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x20u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | (outflank_d9 as (u64)).wrapping_mul(
                                      0x3e1e0e060200u64
                                  ) & 0x402010080400u64) as (u64);
    flipped
}

fn flip_A8(P : u64, O : u64) -> u64 {
    let mut outflank_v : u32;
    let mut outflank_h : u32;
    let mut outflank_d7 : u32;
    let mut flipped : u64;
    outflank_v = (((O as (u32) & 0x1010100u32) << 4i32).wrapping_add(
                      (O >> 32i32) as (u32) & 0x10101u32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     2u32
                 ) & ((P as (u32) & 0x1010101u32) << 4i32).wrapping_add(
                         (P >> 32i32) as (u32) & 0x101u32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0x7e3e1e0e0602u64
               ) & 0x1010101010100u64) as (u64);
    outflank_h = ((O >> 32i32) as (u32) & 0x7e000000u32).wrapping_add(
                     0x2000000u32
                 ) & (P >> 32i32) as (u32);
    flipped = flipped | (outflank_h.wrapping_sub(
                             outflank_h >> 8i32
                         ) & 0x7e000000u32) as (u64) << 32i32;
    outflank_d7 = ((O as (u32) & 0x10204000u32).wrapping_add(
                       (O >> 32i32) as (u32) & 0x20408u32
                   ).wrapping_mul(
                       0x1010101u32
                   ) >> 24i32).wrapping_add(
                      2u32
                  ) & (P as (u32) & 0x10204080u32).wrapping_add(
                          (P >> 32i32) as (u32) & 0x408u32
                      ).wrapping_mul(
                          0x1010101u32
                      ) >> 24i32;
    flipped = (flipped as (u64) | (outflank_d7 as (u64)).wrapping_mul(
                                      0xfcf8f0e0c080u64
                                  ) & 0x2040810204000u64) as (u64);
    flipped
}

fn flip_B8(P : u64, O : u64) -> u64 {
    let mut outflank_v : u32;
    let mut outflank_h : u32;
    let mut outflank_d7 : u32;
    let mut flipped : u64;
    outflank_v = (((O as (u32) & 0x2020200u32) << 3i32).wrapping_add(
                      ((O >> 32i32) as (u32) & 0x20202u32) >> 1i32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     2u32
                 ) & ((P as (u32) & 0x2020202u32) << 3i32).wrapping_add(
                         ((P >> 32i32) as (u32) & 0x202u32) >> 1i32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0xfc7c3c1c0c04u64
               ) & 0x2020202020200u64) as (u64);
    outflank_h = ((O >> 32i32) as (u32) & 0x7c000000u32).wrapping_add(
                     0x4000000u32
                 ) & (P >> 32i32) as (u32);
    flipped = flipped | (outflank_h.wrapping_sub(
                             outflank_h >> 8i32
                         ) & 0x7c000000u32) as (u64) << 32i32;
    outflank_d7 = ((O as (u32) & 0x20400000u32).wrapping_add(
                       (O >> 32i32) as (u32) & 0x40810u32
                   ).wrapping_mul(
                       0x1010101u32
                   ) >> 25i32).wrapping_add(
                      2u32
                  ) & (P as (u32) & 0x20408000u32).wrapping_add(
                          (P >> 32i32) as (u32) & 0x810u32
                      ).wrapping_mul(
                          0x1010101u32
                      ) >> 25i32;
    flipped = (flipped as (u64) | (outflank_d7 as (u64)).wrapping_mul(
                                      0x1f9f1e1c18000u64
                                  ) & 0x4081020400000u64) as (u64);
    flipped
}

fn flip_C8(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d : u32;
    let mut flipped : u64;
    outflank_v = (((O as (u32) & 0x4040400u32) << 2i32).wrapping_add(
                      ((O >> 32i32) as (u32) & 0x40404u32) >> 2i32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     2u32
                 ) & ((P as (u32) & 0x4040404u32) << 2i32).wrapping_add(
                         ((P >> 32i32) as (u32) & 0x404u32) >> 2i32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0x1f8f878381808u64
               ) & 0x4040404040400u64) as (u64);
    outflank_h = (OUTFLANK_2[(
                      O >> 57i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 56i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_2_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff00000000000000u64) as (u64);
    outflank_d = OUTFLANK_2[(
                     (((O >> 32i32) as (u32) & 0xa1020u32).wrapping_add(
                          O as (u32) & 0x40000000u32
                      ).wrapping_mul(
                          0x1010101u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P >> 32i32) as (u32) & 0xa1120u32).wrapping_add(
                                  P as (u32) & 0x40800000u32
                              ).wrapping_mul(
                                  0x1010101u32
                              ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_2_H[(
                                      outflank_d as (usize) ) as usize ] as (u64) & 0xa102040000000u64) as (u64);
    flipped
}

fn flip_D8(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d : u32;
    let mut flipped : u64;
    outflank_v = (((O as (u32) & 0x8080800u32) << 1i32).wrapping_add(
                      ((O >> 32i32) as (u32) & 0x80808u32) >> 3i32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     2u32
                 ) & ((P as (u32) & 0x8080808u32) << 1i32).wrapping_add(
                         ((P >> 32i32) as (u32) & 0x808u32) >> 3i32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0x3f1f0f0703010u64
               ) & 0x8080808080800u64) as (u64);
    outflank_h = (OUTFLANK_3[(
                      O >> 57i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 56i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_3_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff00000000000000u64) as (u64);
    outflank_d = OUTFLANK_3[(
                     (((O >> 32i32) as (u32) & 0x142240u32).wrapping_mul(
                          0x1010101u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P >> 32i32) as (u32) & 0x142241u32).wrapping_add(
                                  P as (u32) & 0x80000000u32
                              ).wrapping_mul(
                                  0x1010101u32
                              ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_3_H[(
                                      outflank_d as (usize) ) as usize ] as (u64) & 0x14224000000000u64) as (u64);
    flipped
}

fn flip_E8(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d : u32;
    let mut flipped : u64;
    outflank_v = ((O as (u32) & 0x10101000u32).wrapping_add(
                      ((O >> 32i32) as (u32) & 0x101010u32) >> 4i32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     2u32
                 ) & (P as (u32) & 0x10101010u32).wrapping_add(
                         ((P >> 32i32) as (u32) & 0x1010u32) >> 4i32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0x7e3e1e0e06020u64
               ) & 0x10101010101000u64) as (u64);
    outflank_h = (OUTFLANK_4[(
                      O >> 57i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 56i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_4_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff00000000000000u64) as (u64);
    outflank_d = OUTFLANK_4[(
                     (((O >> 32i32) as (u32) & 0x284402u32).wrapping_mul(
                          0x1010101u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P >> 32i32) as (u32) & 0x284482u32).wrapping_add(
                                  P as (u32) & 0x1000000u32
                              ).wrapping_mul(
                                  0x1010101u32
                              ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_4_H[(
                                      outflank_d as (usize) ) as usize ] as (u64) & 0x28440200000000u64) as (u64);
    flipped
}

fn flip_F8(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d : u32;
    let mut flipped : u64;
    outflank_v = (((O as (u32) & 0x20202000u32) >> 1i32).wrapping_add(
                      ((O >> 32i32) as (u32) & 0x202020u32) >> 5i32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     2u32
                 ) & ((P as (u32) & 0x20202020u32) >> 1i32).wrapping_add(
                         ((P >> 32i32) as (u32) & 0x2020u32) >> 5i32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0xfc7c3c1c0c040u64
               ) & 0x20202020202000u64) as (u64);
    outflank_h = (OUTFLANK_5[(
                      O >> 57i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 56i32) as (u32);
    flipped = (flipped as (u64) | FLIPPED_5_H[(
                                      outflank_h as (usize) ) as usize ] as (u64) & 0xff00000000000000u64) as (u64);
    outflank_d = OUTFLANK_5[(
                     (((O >> 32i32) as (u32) & 0x500804u32).wrapping_add(
                          O as (u32) & 0x2000000u32
                      ).wrapping_mul(
                          0x1010101u32
                      ) >> 25i32) as (usize) ) as usize ] as (u32) & ((P >> 32i32) as (u32) & 0x508804u32).wrapping_add(
                                  P as (u32) & 0x2010000u32
                              ).wrapping_mul(
                                  0x1010101u32
                              ) >> 24i32;
    flipped = (flipped as (u64) | FLIPPED_5_H[(
                                      outflank_d as (usize) ) as usize ] as (u64) & 0x50080402000000u64) as (u64);
    flipped
}

fn flip_G8(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = (((O as (u32) & 0x40404000u32) >> 2i32).wrapping_add(
                      ((O >> 32i32) as (u32) & 0x404040u32) >> 6i32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     2u32
                 ) & ((P as (u32) & 0x40404040u32) >> 2i32).wrapping_add(
                         ((P >> 32i32) as (u32) & 0x4040u32) >> 6i32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0x1f8f8783818080u64
               ) & 0x40404040404000u64) as (u64);
    outflank_h = (OUTFLANK_7[(
                      O >> 56i32 & 0x3eu64
                  ) as usize ] as (u64) & P >> 55i32) as (u32);
    flipped = flipped | (outflank_h.wrapping_neg(
                         ) & 0x3eu32) as (u64) << 56i32;
    outflank_d9 = OUTFLANK_7[(
                      ((O as (u32) & 0x4020000u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x201008u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 24i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x4020100u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x1008u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 23i32;
    flipped = (flipped as (u64) | (outflank_d9 as (u64)).wrapping_mul(
                                      0x1f0f0703010000u64
                                  ) & 0x20100804020000u64) as (u64);
    flipped
}

fn flip_H8(P : u64, O : u64) -> u64 {
    let mut outflank_h : u32;
    let mut outflank_v : u32;
    let mut outflank_d9 : u32;
    let mut flipped : u64;
    outflank_v = (((O as (u32) & 0x80808000u32) >> 3i32).wrapping_add(
                      ((O >> 32i32) as (u32) & 0x808080u32) >> 7i32
                  ).wrapping_mul(
                      0x8040201u32
                  ) >> 24i32).wrapping_add(
                     2u32
                 ) & ((P as (u32) & 0x80808080u32) >> 3i32).wrapping_add(
                         ((P >> 32i32) as (u32) & 0x8080u32) >> 7i32
                     ).wrapping_mul(
                         0x8040201u32
                     ) >> 24i32;
    flipped = ((outflank_v as (u64)).wrapping_mul(
                   0x3f1f0f07030100u64
               ) & 0x80808080808000u64) as (u64);
    outflank_h = (OUTFLANK_7[(
                      O >> 57i32 & 0x3fu64
                  ) as usize ] as (u64) & P >> 56i32) as (u32);
    flipped = flipped | (outflank_h.wrapping_neg(
                         ) & 0x3fu32) as (u64) << 57i32;
    outflank_d9 = OUTFLANK_7[(
                      ((O as (u32) & 0x8040200u32).wrapping_add(
                           (O >> 32i32) as (u32) & 0x402010u32
                       ).wrapping_mul(
                           0x1010101u32
                       ) >> 25i32) as (usize) ) as usize ] as (u32) & (P as (u32) & 0x8040201u32).wrapping_add(
                                   (P >> 32i32) as (u32) & 0x2010u32
                               ).wrapping_mul(
                                   0x1010101u32
                               ) >> 24i32;
    flipped = (flipped as (u64) | (outflank_d9 as (u64)).wrapping_mul(
                                      0x7e3e1e0e060200u64
                                  ) & 0x40201008040200u64) as (u64);
    flipped
}

fn flip_pass(P : u64, O : u64) -> u64 {
    P;
    O;
    0u64
}

const MOVE_FN : [fn(u64, u64) -> u64; 256] = [
    flip_A1, flip_B1, flip_C1, flip_D1, flip_E1, flip_F1, flip_G1, flip_H1,
    flip_A2, flip_B2, flip_C2, flip_D2, flip_E2, flip_F2, flip_G2, flip_H2,
    flip_A3, flip_B3, flip_C3, flip_D3, flip_E3, flip_F3, flip_G3, flip_H3,
    flip_A4, flip_B4, flip_C4, flip_D4, flip_E4, flip_F4, flip_G4, flip_H4,
    flip_A5, flip_B5, flip_C5, flip_D5, flip_E5, flip_F5, flip_G5, flip_H5,
    flip_A6, flip_B6, flip_C6, flip_D6, flip_E6, flip_F6, flip_G6, flip_H6,
    flip_A7, flip_B7, flip_C7, flip_D7, flip_E7, flip_F7, flip_G7, flip_H7,
    flip_A8, flip_B8, flip_C8, flip_D8, flip_E8, flip_F8, flip_G8, flip_H8,
    
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass,
    flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass, flip_pass 
];

pub fn do_move(m : u8, ps : u64, os : u64) -> u64 {
    MOVE_FN[(m as usize) as usize ](ps, os)
}


