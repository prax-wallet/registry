#pragma once

#include <stdint.h>

#define ASSET_ID_LEN 32
#define NUM_SUPPORTED_ASSETS 79

typedef struct {
    uint8_t asset_id[ASSET_ID_LEN];
    const char symbol[80];
    const char name[120];
    uint8_t decimals;
} asset_info_t;

static const asset_info_t supported_assets[NUM_SUPPORTED_ASSETS] = {
    {{0x01, 0xD9, 0x53, 0xEB, 0xBA, 0x54, 0x0F, 0x1D, 0x30, 0xB0, 0x81, 0x41, 0x0B, 0x8A, 0x29, 0x6E,
        0xC9, 0x58, 0x10, 0xE9, 0x3A, 0xF9, 0x00, 0xC8, 0xCD, 0x65, 0x1C, 0x56, 0xD4, 0x25, 0x5E, 0x04},
     "transfer/channel-4/ckBTC", "transfer/channel-4/factory/osmo10c4y9csfs8q7mtvfg4p9gd8d0acx0hpc2mte9xqzthd7rd3348tsfhaesm/sICP-icrc-ckBTC", 8},
    {{0x02, 0xBE, 0x8C, 0x84, 0x57, 0x05, 0x93, 0xD4, 0x16, 0x96, 0xD2, 0x76, 0xA2, 0x64, 0x98, 0xDF,
        0x18, 0x17, 0xA5, 0xEC, 0x6D, 0xB9, 0x75, 0xD3, 0xB1, 0x0B, 0x2E, 0x6E, 0x64, 0xB2, 0xEE, 0x09},
     "transfer/channel-4/sail", "transfer/channel-4/factory/osmo1rckme96ptawr4zwexxj5g5gej9s2dmud8r2t9j0k0prn5mch5g4snzzwjv/sail", 6},
    {{0x04, 0xAF, 0xC7, 0xCF, 0xCD, 0xBA, 0xB3, 0xDE, 0x2D, 0x2A, 0xFE, 0xC0, 0xC5, 0x23, 0xD3, 0x1A,
        0xAA, 0x23, 0x83, 0x34, 0xE7, 0x65, 0x2B, 0x19, 0x8E, 0x95, 0x13, 0x62, 0xEA, 0x64, 0x67, 0x0B},
     "transfer/channel-4/ton", "transfer/channel-4/factory/osmo12lnwf54yd30p6amzaged2atln8k0l32n7ncxf04ctg7u7ymnsy7qkqgsw4/alloyed/allTON", 9},
    {{0x07, 0xEF, 0x66, 0x01, 0x32, 0xA4, 0xC3, 0x23, 0x5F, 0xAB, 0x27, 0x2D, 0x43, 0xD9, 0xB9, 0x75,
        0x2A, 0x83, 0x37, 0xB2, 0xD1, 0x08, 0x59, 0x7A, 0xBF, 0xFA, 0xFF, 0x5F, 0x24, 0x6D, 0x0F, 0x0F},
     "transfer/channel-0/atom", "transfer/channel-0/uatom", 6},
    {{0x10, 0xCB, 0x6D, 0x51, 0x3C, 0xE7, 0x5D, 0xA6, 0x6E, 0xD9, 0xDE, 0xDE, 0x70, 0x74, 0xB9, 0x50,
        0x2D, 0x11, 0xBC, 0x9B, 0x01, 0x87, 0xD1, 0x98, 0x73, 0xBF, 0x5A, 0x6F, 0x29, 0xB6, 0x7B, 0x07},
     "transfer/channel-4/sqbtc", "transfer/channel-4/factory/osmo1g8qypve6l95xmhgc0fddaecerffymsl7kn9muw/sqbtc", 6},
    {{0x1E, 0x23, 0x1F, 0x6E, 0x12, 0xDB, 0x4D, 0x30, 0x0F, 0xD1, 0xB0, 0xC8, 0xD6, 0xCA, 0x70, 0x9A,
        0x46, 0x54, 0x0C, 0x09, 0x7A, 0xE7, 0xC5, 0xFC, 0xFF, 0xC5, 0xA9, 0xB5, 0xEB, 0x61, 0x4F, 0x0C},
     "transfer/channel-4/stibcx", "transfer/channel-4/factory/osmo1xqw2sl9zk8a6pch0csaw78n4swg5ws8t62wc5qta4gnjxfqg6v2qcs243k/stuibcx", 6},
    {{0x1F, 0x1E, 0xBE, 0x1B, 0xBE, 0xC2, 0x53, 0x6D, 0xB2, 0x7C, 0x4C, 0x5F, 0x0F, 0xB5, 0xFC, 0x4C,
        0xED, 0xF5, 0x42, 0x9A, 0x9C, 0x00, 0x16, 0x00, 0x63, 0x35, 0xD2, 0x71, 0xA7, 0x4B, 0x86, 0x05},
     "transfer/channel-4/fGECK", "transfer/channel-4/factory/osmo1dywfmhyc8y0wga7qpzej0x0mgwqg25fj4eccp494w8yafzdpgamsx9ryyv/fGECK", 9},
    {{0x25, 0xF2, 0x5C, 0xDF, 0x9C, 0x60, 0xB5, 0x43, 0xE6, 0x29, 0xF8, 0x9D, 0x0E, 0x38, 0x07, 0x0E,
        0x72, 0x18, 0x43, 0x69, 0x25, 0x9A, 0xEF, 0x31, 0xE0, 0xDF, 0x25, 0x6C, 0xAA, 0xF2, 0x0F, 0x06},
     "transfer/channel-4/CRAZYHORSE", "transfer/channel-4/factory/osmo1q77cw0mmlluxu0wr29fcdd0tdnh78gzhkvhe4n6ulal9qvrtu43qtd0nh8/crazyhorse", 6},
    {{0x27, 0x8C, 0x3D, 0xAE, 0x59, 0x38, 0x1F, 0x1E, 0x95, 0xA5, 0x52, 0xC8, 0xFE, 0x4A, 0xAD, 0xC5,
        0xFF, 0x8A, 0x73, 0x11, 0x30, 0xC8, 0x63, 0x64, 0x4E, 0xDC, 0xF1, 0x61, 0x4A, 0x4A, 0xAB, 0x06},
     "transfer/channel-4/sqatom", "transfer/channel-4/factory/osmo1g8qypve6l95xmhgc0fddaecerffymsl7kn9muw/sqatom", 6},
    {{0x29, 0x23, 0xA0, 0xA8, 0x7B, 0x3A, 0x24, 0x21, 0xF1, 0x65, 0xCC, 0x85, 0x3D, 0xBF, 0x73, 0xA9,
        0xBD, 0xAF, 0xB5, 0xDA, 0x0D, 0x94, 0x85, 0x64, 0xB6, 0x05, 0x9C, 0xB0, 0x21, 0x7C, 0x44, 0x07},
     "transfer/channel-4/osmo", "transfer/channel-4/uosmo", 6},
    {{0x29, 0x32, 0x7D, 0x89, 0x9F, 0xDE, 0x34, 0xDE, 0x36, 0x57, 0x68, 0x61, 0xCE, 0xCE, 0x8D, 0x96,
        0x06, 0xFC, 0xCF, 0x37, 0x49, 0x9A, 0x50, 0x59, 0x66, 0xF2, 0x9F, 0x09, 0xCE, 0x06, 0x88, 0x0C},
     "transfer/channel-4/BADKID", "transfer/channel-4/factory/osmo10n8rv8npx870l69248hnp6djy6pll2yuzzn9x8/BADKID", 6},
    {{0x29, 0xEA, 0x9C, 0x2F, 0x33, 0x71, 0xF6, 0xA4, 0x87, 0xE7, 0xE9, 0x5C, 0x24, 0x70, 0x41, 0xF4,
        0xA3, 0x56, 0xF9, 0x83, 0xEB, 0x06, 0x4E, 0x5D, 0x2B, 0x3B, 0xCF, 0x32, 0x2C, 0xA9, 0x6A, 0x10},
     "upenumbra", "upenumbra", 0},
    {{0x2D, 0xD7, 0xB1, 0x09, 0xC2, 0x12, 0xEE, 0x1E, 0x63, 0x6D, 0x28, 0xE2, 0x78, 0xC5, 0x7E, 0x32,
        0x80, 0xB6, 0x20, 0x1D, 0x57, 0x9A, 0xE2, 0xB1, 0xCC, 0xD3, 0x5E, 0x46, 0x0A, 0xB8, 0x40, 0x04},
     "transfer/channel-4/allSOL", "transfer/channel-4/factory/osmo1n3n75av8awcnw4jl62n3l48e6e4sxqmaf97w5ua6ddu4s475q5qq9udvx4/alloyed/allSOL", 9},
    {{0x30, 0xC0, 0xDB, 0x0B, 0x88, 0xD0, 0xEC, 0x65, 0x0F, 0xEB, 0x21, 0x14, 0xF9, 0x8A, 0x36, 0xC6,
        0x45, 0xBC, 0xE4, 0xBC, 0x91, 0x28, 0x90, 0x12, 0x24, 0xDE, 0x0C, 0x3C, 0x70, 0x0F, 0x9A, 0x11},
     "transfer/channel-4/lvn", "transfer/channel-4/factory/osmo1mlng7pz4pnyxtpq0akfwall37czyk9lukaucsrn30ameplhhshtqdvfm5c/ulvn", 6},
    {{0x31, 0xC0, 0x9A, 0x7E, 0x83, 0x5B, 0xEB, 0x49, 0x41, 0xB4, 0xBD, 0x48, 0xEF, 0xB0, 0x03, 0x87,
        0x1D, 0x06, 0x20, 0x8F, 0x44, 0x1C, 0x7F, 0x22, 0x0A, 0xA9, 0xA9, 0x1A, 0xBA, 0x37, 0x3E, 0x09},
     "transfer/channel-4/pepe", "transfer/channel-4/factory/osmo1nnlxegt0scm9qkzys9c874t0ntapv4epfjy2w49c0xdrp3dr0v4ssmelzx/alloyed/allPEPE", 12},
    {{0x31, 0xDE, 0xB1, 0x5E, 0xE8, 0xE0, 0x93, 0x48, 0xEF, 0x75, 0x43, 0xE1, 0x95, 0xFF, 0xF4, 0xE7,
        0x0B, 0xD3, 0x79, 0x70, 0xC4, 0x87, 0xC5, 0xFF, 0x6C, 0x3B, 0xAB, 0xAD, 0x52, 0x70, 0xDA, 0x05},
     "transfer/channel-4/BVT0", "transfer/channel-4/factory/osmo1xu0gk9aggv79597xwazyfzaggv2pze9z7cq3p9p72tkkux9a7xaqufa792/BVT", 18},
    {{0x34, 0xE1, 0x12, 0xA7, 0xB2, 0x5D, 0x7C, 0x50, 0x1B, 0xAA, 0x3B, 0xE3, 0x16, 0xA3, 0x00, 0x93,
        0xF9, 0x70, 0x17, 0xE7, 0x88, 0x8A, 0x2F, 0x95, 0x21, 0x4E, 0x59, 0x8C, 0x28, 0xFA, 0xD9, 0x0A},
     "transfer/channel-4/fMAD", "transfer/channel-4/factory/osmo1dywfmhyc8y0wga7qpzej0x0mgwqg25fj4eccp494w8yafzdpgamsx9ryyv/fMAD", 9},
    {{0x37, 0xA7, 0x23, 0x6C, 0x5F, 0xBD, 0xFF, 0x3B, 0x42, 0x9E, 0x61, 0x1B, 0xD3, 0x7E, 0x57, 0x46,
        0x50, 0xC9, 0x0A, 0x0C, 0x70, 0x9B, 0x84, 0x3E, 0x6A, 0x9B, 0xF9, 0x9B, 0xBD, 0xD9, 0x1A, 0x00},
     "transfer/channel-2/stake", "transfer/channel-2/ustake", 6},
    {{0x39, 0x47, 0x17, 0x1B, 0xB6, 0x04, 0xC7, 0xC6, 0xB9, 0x36, 0x57, 0x7F, 0xC0, 0x23, 0x0A, 0xD5,
        0xA3, 0x8A, 0x01, 0x19, 0x6F, 0xA0, 0x9B, 0x46, 0xA5, 0x71, 0xE4, 0x8E, 0xEE, 0x0B, 0x56, 0x04},
     "transfer/channel-4/WIHA", "transfer/channel-4/factory/osmo1q77cw0mmlluxu0wr29fcdd0tdnh78gzhkvhe4n6ulal9qvrtu43qtd0nh8/wiha", 6},
    {{0x3A, 0xB7, 0x38, 0xF2, 0x58, 0x27, 0xDE, 0x7F, 0x9F, 0x3E, 0x02, 0x99, 0x12, 0xFB, 0x22, 0xA1,
        0xAE, 0x4D, 0xCA, 0x8B, 0x13, 0xF1, 0x71, 0x50, 0x74, 0xEA, 0xF5, 0xA3, 0x57, 0xE2, 0xB0, 0x0D},
     "transfer/channel-4/fATLAS", "transfer/channel-4/factory/osmo1dywfmhyc8y0wga7qpzej0x0mgwqg25fj4eccp494w8yafzdpgamsx9ryyv/fATLAS", 9},
    {{0x3D, 0x3E, 0xAA, 0x2B, 0x2E, 0x0F, 0x8B, 0xB3, 0x69, 0xCD, 0xCB, 0x37, 0x7B, 0x60, 0x54, 0xB6,
        0x15, 0xA2, 0x7B, 0x5E, 0xFD, 0x0B, 0xFA, 0xE9, 0x42, 0xFB, 0x65, 0xAD, 0xC4, 0x90, 0x87, 0x05},
     "transfer/channel-4/fCRYPTONIUM", "transfer/channel-4/factory/osmo1dywfmhyc8y0wga7qpzej0x0mgwqg25fj4eccp494w8yafzdpgamsx9ryyv/fCRYPTONIUM", 9},
    {{0x3F, 0x1D, 0x41, 0x8C, 0x61, 0x1E, 0x68, 0xFB, 0xE0, 0x48, 0xF6, 0x57, 0xFF, 0xC8, 0x03, 0xC0,
        0x7C, 0x30, 0x9F, 0x60, 0x56, 0x0F, 0xE7, 0xE3, 0x32, 0xB7, 0x6E, 0xE7, 0xF6, 0x61, 0x2E, 0x12},
     "transfer/channel-4/BERNESE", "transfer/channel-4/factory/osmo1s6ht8qrm8x0eg8xag5x3ckx9mse9g4se248yss/BERNESE", 6},
    {{0x40, 0x0A, 0xCC, 0x44, 0x8A, 0x72, 0x88, 0xC4, 0xDA, 0xC0, 0xC9, 0x2D, 0x9B, 0x98, 0xF9, 0xDC,
        0x77, 0xCB, 0x34, 0xCA, 0xC8, 0xF1, 0x32, 0x01, 0x43, 0x6C, 0x2A, 0xC8, 0x27, 0xE0, 0xB5, 0x06},
     "transfer/channel-1/dydx", "transfer/channel-1/adydx", 18},
    {{0x41, 0x4E, 0x72, 0x3F, 0x74, 0xBD, 0x98, 0x7C, 0x02, 0xCC, 0xBC, 0x99, 0x75, 0x85, 0xED, 0x52,
        0xB1, 0x96, 0xE2, 0xFF, 0xE7, 0x5B, 0x37, 0x93, 0xAA, 0x68, 0xCC, 0x29, 0x96, 0x62, 0x69, 0x10},
     "transfer/channel-4/allBTC", "transfer/channel-4/factory/osmo1z6r6qdknhgsc0zeracktgpcxf43j6sekq07nw8sxduc9lg0qjjlqfu25e3/alloyed/allBTC", 8},
    {{0x46, 0xF8, 0x79, 0xE4, 0x52, 0x6B, 0x77, 0xCA, 0xB7, 0x91, 0x3F, 0x31, 0x60, 0x89, 0x79, 0x02,
        0x94, 0xCE, 0x73, 0xD1, 0x2B, 0xC7, 0x22, 0xFC, 0x9B, 0x53, 0x52, 0xE5, 0x10, 0x2B, 0x1F, 0x11},
     "transfer/channel-4/op", "transfer/channel-4/factory/osmo1nufyzqlm8qhu2w7lm0l4rrax0ec8rsk69mga4tel8eare7c7ljaqpk2lyg/alloyed/allOP", 12},
    {{0x48, 0x16, 0x0D, 0x2C, 0xC5, 0x2C, 0x09, 0x04, 0x09, 0x69, 0x96, 0x58, 0x5D, 0xBA, 0xC3, 0x1C,
        0x3E, 0x6E, 0xFE, 0xAE, 0xAB, 0x76, 0x8E, 0xAC, 0x51, 0x12, 0xB2, 0x06, 0x51, 0x8C, 0xB7, 0x11},
     "transfer/channel-4/dot", "transfer/channel-4/factory/osmo1r53fx9fvcdzncrs7zkn4gw5vfelx5gk8k5wc6wqha2jpkh992rusr5tk02/alloyed/allDOT", 10},
    {{0x48, 0x28, 0x4D, 0x9F, 0x36, 0xBD, 0xA8, 0x34, 0x0C, 0x34, 0x6A, 0xB0, 0x4A, 0x84, 0x0C, 0xCC,
        0x1D, 0xB1, 0xD8, 0xCE, 0x67, 0x98, 0x40, 0x6D, 0x49, 0x3D, 0x44, 0xA1, 0x2C, 0xB6, 0xF6, 0x03},
     "transfer/channel-4/allUNI", "transfer/channel-4/factory/osmo1eqjda4pc6e09jtxzxggf6jl3jye2yn453ja58we5gxwzmf5ah28qvlnaz8/alloyed/allUNI", 12},
    {{0x4C, 0xD1, 0xF0, 0xFF, 0xEE, 0x4F, 0x3C, 0xDD, 0x01, 0x0B, 0x65, 0x35, 0x43, 0xCD, 0x02, 0x9E,
        0x1A, 0x22, 0x2B, 0x1D, 0xC6, 0x23, 0xC8, 0xAF, 0x99, 0xBD, 0x8E, 0x1B, 0xA9, 0xB9, 0xC8, 0x02},
     "transfer/channel-4/trx", "transfer/channel-4/factory/osmo1myv2g72h8dan7n4hx7stt3mmust6ws03zh6gxc7vz4hpmgp5z3lq9aunm9/TRX.rt", 6},
    {{0x51, 0x61, 0x08, 0xD0, 0xD0, 0xBB, 0xA3, 0xF7, 0x6E, 0x1F, 0x98, 0x2D, 0x0A, 0x7C, 0xDE, 0x11,
        0x88, 0x33, 0x30, 0x7B, 0x03, 0xC0, 0xCD, 0x4C, 0xCB, 0x94, 0xE8, 0x82, 0xB5, 0x3C, 0x1F, 0x0F},
     "transfer/channel-4/wbtc", "transfer/channel-4/factory/osmo1z0qrq605sjgcqpylfl4aa6s90x738j7m58wyatt0tdzflg2ha26q67k743/wbtc", 8},
    {{0x53, 0x14, 0xB3, 0x3E, 0xEC, 0xFD, 0x5C, 0xA2, 0xE9, 0x9C, 0x0B, 0x6D, 0x1E, 0x0C, 0xCA, 0xFE,
        0x3D, 0x2D, 0xD5, 0x81, 0xC9, 0x52, 0xD8, 0x14, 0xFB, 0x64, 0xFD, 0xF5, 0x1F, 0x85, 0xC4, 0x11},
     "transfer/channel-3/tia", "transfer/channel-3/utia", 6},
    {{0x53, 0x75, 0x34, 0x3F, 0xE7, 0xD5, 0xD9, 0x4E, 0x44, 0xA9, 0x4D, 0x3E, 0x92, 0x0A, 0xBB, 0xA9,
        0xDF, 0xAC, 0x8C, 0x26, 0xD5, 0x84, 0x67, 0x81, 0xE8, 0x5F, 0x22, 0xC7, 0x8D, 0xB5, 0x58, 0x07},
     "transfer/channel-4/factory/osmo13gu58hzw3e9aqpj25h67m7snwcjuccd7v4p55w/brnz", "transfer/channel-4/factory/osmo13gu58hzw3e9aqpj25h67m7snwcjuccd7v4p55w/brnz", 0},
    {{0x54, 0x31, 0x18, 0x55, 0x46, 0x7C, 0x64, 0xA9, 0x61, 0xB0, 0x04, 0xF6, 0x9E, 0xBE, 0x8D, 0x81,
        0xD3, 0x5D, 0x8B, 0xB8, 0xD1, 0x12, 0xC2, 0xA6, 0x15, 0x45, 0x74, 0x53, 0x9B, 0x94, 0xC7, 0x05},
     "transfer/channel-4/IBC", "transfer/channel-4/factory/osmo1kqdw6pvn0xww6tyfv2sqvkkencdz0qw406x54r/IBC", 6},
    {{0x57, 0x9E, 0xF6, 0x6D, 0x5F, 0x72, 0x88, 0xAF, 0x67, 0x25, 0x63, 0x0C, 0xF7, 0xF3, 0xF9, 0x9B,
        0x00, 0x9E, 0x88, 0x67, 0xD3, 0xE0, 0x96, 0xCC, 0xCB, 0xA8, 0xF6, 0x45, 0x48, 0xBD, 0xDE, 0x0E},
     "transfer/channel-4/fBAD", "transfer/channel-4/factory/osmo1dywfmhyc8y0wga7qpzej0x0mgwqg25fj4eccp494w8yafzdpgamsx9ryyv/fBAD", 9},
    {{0x59, 0xD1, 0xDE, 0x1C, 0x39, 0xA4, 0x95, 0x62, 0xB1, 0x15, 0xFD, 0x20, 0xF3, 0xA3, 0x22, 0x63,
        0x2E, 0x8C, 0xB7, 0xA9, 0x54, 0x43, 0x36, 0xB9, 0x83, 0xE3, 0x5F, 0x36, 0xE2, 0xB6, 0xA0, 0x01},
     "transfer/channel-4/cdt", "transfer/channel-4/factory/osmo1s794h9rxggytja3a4pmwul53u98k06zy2qtrdvjnfuxruh7s8yjs6cyxgd/ucdt", 6},
    {{0x64, 0x07, 0xF4, 0xBF, 0x07, 0xD0, 0x74, 0xC8, 0xCB, 0xC8, 0x91, 0xFA, 0x38, 0x84, 0x2B, 0xB7,
        0xAB, 0x76, 0xCB, 0xA5, 0x54, 0xDF, 0xBA, 0xF3, 0x5C, 0x19, 0xBC, 0x5D, 0xE1, 0x7A, 0x75, 0x06},
     "transfer/channel-4/ibcx", "transfer/channel-4/factory/osmo14klwqgkmackvx2tqa0trtg69dmy0nrg4ntq4gjgw2za4734r5seqjqm4gm/uibcx", 6},
    {{0x64, 0xCB, 0xF3, 0x35, 0xED, 0x28, 0x6A, 0x84, 0x63, 0x84, 0x12, 0x0F, 0x7F, 0x42, 0xA0, 0x50,
        0x61, 0xF4, 0x11, 0x56, 0x84, 0xE9, 0xD7, 0xAB, 0xE4, 0x56, 0xAF, 0x44, 0xD7, 0xCC, 0x6E, 0x04},
     "transfer/channel-4/fNUT", "transfer/channel-4/factory/osmo1dywfmhyc8y0wga7qpzej0x0mgwqg25fj4eccp494w8yafzdpgamsx9ryyv/fNUT", 9},
    {{0x66, 0x92, 0x1B, 0xC4, 0x5C, 0x2F, 0x95, 0x4E, 0x2E, 0x76, 0x1F, 0x66, 0xBA, 0xD1, 0xBF, 0xDA,
        0x12, 0xC1, 0x22, 0x97, 0x4D, 0x5E, 0x16, 0xAD, 0xDD, 0x30, 0x91, 0x6E, 0xCD, 0x64, 0x6E, 0x02},
     "transfer/channel-4/rbtc", "transfer/channel-4/factory/osmo1myv2g72h8dan7n4hx7stt3mmust6ws03zh6gxc7vz4hpmgp5z3lq9aunm9/BTC.rt", 18},
    {{0x76, 0xB3, 0xE4, 0xB1, 0x06, 0x81, 0x35, 0x8C, 0x12, 0x3B, 0x38, 0x1F, 0x90, 0x63, 0x84, 0x76,
        0xB7, 0x78, 0x90, 0x40, 0xE4, 0x78, 0x02, 0xDE, 0x87, 0x9F, 0x0F, 0xB3, 0xEE, 0xDC, 0x8D, 0x0B},
     "transfer/channel-2/usdc", "transfer/channel-2/uusdc", 6},
    {{0x77, 0x6F, 0xF2, 0x05, 0xD8, 0xE4, 0xCF, 0xFF, 0xE1, 0xB6, 0xB4, 0x30, 0x1D, 0x85, 0xD5, 0xC7,
        0x75, 0xCD, 0xD9, 0x02, 0x0B, 0x75, 0x9A, 0x12, 0xE6, 0x27, 0x24, 0x0C, 0x07, 0x11, 0x5A, 0x0B},
     "transfer/channel-4/bwh", "transfer/channel-4/factory/osmo1q77cw0mmlluxu0wr29fcdd0tdnh78gzhkvhe4n6ulal9qvrtu43qtd0nh8/bwh", 6},
    {{0x7D, 0xE5, 0x4F, 0xEE, 0x9B, 0x2D, 0xEE, 0xF8, 0x0F, 0x86, 0x8C, 0x4B, 0x2E, 0x26, 0x2D, 0x34,
        0x71, 0x70, 0x52, 0x9A, 0xEC, 0xB0, 0xB4, 0x7C, 0x35, 0xF9, 0xFB, 0xC1, 0x45, 0x36, 0x5B, 0x00},
     "transfer/channel-4/sqtia", "transfer/channel-4/factory/osmo1g8qypve6l95xmhgc0fddaecerffymsl7kn9muw/sqtia", 6},
    {{0x7F, 0xF2, 0x00, 0x37, 0x4E, 0xB9, 0x3A, 0xEE, 0x97, 0xC1, 0x87, 0x56, 0x7B, 0x79, 0xB7, 0x4A,
        0x08, 0x8B, 0x14, 0x36, 0x0E, 0x01, 0x8E, 0x3B, 0xBE, 0x98, 0x6B, 0xF7, 0x2F, 0x9B, 0xA3, 0x04},
     "transfer/channel-4/fWIZ", "transfer/channel-4/factory/osmo1dywfmhyc8y0wga7qpzej0x0mgwqg25fj4eccp494w8yafzdpgamsx9ryyv/fWIZ", 9},
    {{0x80, 0xEE, 0x2A, 0x47, 0xB9, 0x98, 0x6A, 0xB0, 0xB8, 0x00, 0x03, 0xC2, 0x72, 0x4E, 0xD7, 0x1E,
        0x1B, 0xA3, 0x34, 0x15, 0x16, 0x1A, 0xC6, 0xB1, 0x45, 0x6E, 0x94, 0xDC, 0x99, 0x2B, 0x4A, 0x03},
     "transfer/channel-4/RAPTR", "transfer/channel-4/factory/osmo1279xudevmf5cw83vkhglct7jededp86k90k2le/RAPTR", 6},
    {{0x83, 0x89, 0xA8, 0xA7, 0xCE, 0x12, 0x05, 0xE2, 0xCE, 0x7E, 0x38, 0x87, 0x34, 0x4A, 0x67, 0x30,
        0xA5, 0x29, 0x3F, 0x64, 0x98, 0xEC, 0x10, 0x28, 0x09, 0x77, 0xC2, 0xC5, 0x8E, 0xF0, 0xB2, 0x02},
     "transfer/channel-2/eure", "transfer/channel-2/ueure", 6},
    {{0x84, 0xE9, 0xCC, 0xA9, 0x06, 0xE9, 0x7B, 0x90, 0x1B, 0x7B, 0xBB, 0xEF, 0x7C, 0x76, 0x1D, 0x6B,
        0xBE, 0xDF, 0x83, 0x76, 0x62, 0xB7, 0xCF, 0x1F, 0x6A, 0xDA, 0x83, 0x7C, 0xF3, 0xB5, 0x6E, 0x04},
     "transfer/channel-4/shib", "transfer/channel-4/factory/osmo1f588gk9dazpsueevdl2w6wfkmfmhg5gdvg2uerdlzl0atkasqhsq59qc6a/alloyed/allSHIB", 12},
    {{0x87, 0x87, 0x3E, 0x38, 0x01, 0xA8, 0x9C, 0x65, 0x51, 0x10, 0x2F, 0x59, 0x69, 0x8C, 0xCB, 0x6E,
        0xBA, 0x67, 0x4A, 0x45, 0x29, 0x97, 0x42, 0x61, 0x1E, 0xA1, 0xF0, 0x06, 0x88, 0x1E, 0x5F, 0x0D},
     "transfer/channel-4/link", "transfer/channel-4/factory/osmo18zdw5yvs6gfp95rp74qqwug9yduw2fyr8kplk2xgs726s9axc5usa2vpgw/alloyed/allLINK", 12},
    {{0x88, 0x8F, 0x35, 0x3B, 0x9F, 0x36, 0xD6, 0x3A, 0x81, 0x22, 0xEE, 0x05, 0xD5, 0x2D, 0x84, 0xE3,
        0xEF, 0x94, 0x97, 0x37, 0x2F, 0x2B, 0xEC, 0x88, 0xF1, 0xC2, 0xC4, 0x29, 0x92, 0x02, 0x80, 0x0B},
     "transfer/channel-4/ampOSMO", "transfer/channel-4/factory/osmo1dv8wz09tckslr2wy5z86r46dxvegylhpt97r9yd6qc3kyc6tv42qa89dr9/ampOSMO", 6},
    {{0x8A, 0x71, 0x32, 0xE5, 0x73, 0x50, 0x04, 0xFE, 0x6E, 0xEE, 0x8C, 0xFB, 0x79, 0x47, 0x11, 0x2E,
        0x1B, 0xAE, 0x05, 0x68, 0x82, 0xD4, 0x54, 0xE7, 0x24, 0x47, 0xC1, 0x85, 0x72, 0x17, 0xEC, 0x0F},
     "transfer/channel-4/bOSMO", "transfer/channel-4/factory/osmo1s3l0lcqc7tu0vpj6wdjz9wqpxv8nk6eraevje4fuwkyjnwuy82qsx3lduv/boneOsmo", 6},
    {{0x93, 0x67, 0xE5, 0x46, 0xE2, 0x30, 0x8B, 0xC2, 0x00, 0xBF, 0x20, 0xDA, 0xB0, 0x02, 0xFF, 0xF5,
        0xF8, 0xE5, 0xF2, 0x61, 0xA9, 0x3C, 0x23, 0xC2, 0x32, 0x16, 0x26, 0xB2, 0x1F, 0x00, 0x95, 0x04},
     "transfer/channel-4/CosmoUSD", "transfer/channel-4/factory/osmo104jtrwcljnxfljhml8mxrw7qetcsdmqvy3sprw/ucosmousd", 6},
    {{0x95, 0xA4, 0xEA, 0x9A, 0xA5, 0x74, 0xAF, 0x73, 0xC4, 0x44, 0x71, 0x11, 0x90, 0xF4, 0xB2, 0x36,
        0xEE, 0xDD, 0xEB, 0xDE, 0xC5, 0xE0, 0x2F, 0x26, 0xBA, 0x92, 0xB2, 0x7E, 0xC2, 0xA7, 0x8C, 0x0A},
     "transfer/channel-4/AVAIL", "transfer/channel-4/factory/osmo1myv2g72h8dan7n4hx7stt3mmust6ws03zh6gxc7vz4hpmgp5z3lq9aunm9/AVAIL.rt", 18},
    {{0x9F, 0x76, 0xDA, 0x75, 0x91, 0x99, 0xFF, 0x2E, 0x6C, 0x03, 0x28, 0xB7, 0xB6, 0x76, 0x18, 0x56,
        0xE2, 0x57, 0xFA, 0xCA, 0xAE, 0xCE, 0x27, 0x36, 0x18, 0xE5, 0x55, 0xF4, 0x5A, 0xF5, 0x6C, 0x10},
     "transfer/channel-4/fBULLS", "transfer/channel-4/factory/osmo1dywfmhyc8y0wga7qpzej0x0mgwqg25fj4eccp494w8yafzdpgamsx9ryyv/fBULLS", 9},
    {{0xA5, 0xF8, 0x6E, 0x48, 0x0A, 0xDF, 0xC1, 0x80, 0x91, 0xA5, 0x12, 0x3E, 0x3D, 0x8D, 0xDB, 0x33,
        0x7C, 0x16, 0xA6, 0x62, 0x9D, 0x82, 0xA6, 0x65, 0x9B, 0x1A, 0xEA, 0x08, 0xF8, 0x6B, 0x5E, 0x10},
     "transfer/channel-2/frienzies", "transfer/channel-2/ufrienzies", 6},
    {{0xA7, 0xA3, 0x39, 0xF4, 0x2E, 0x67, 0x1B, 0x2D, 0xB1, 0xDE, 0x22, 0x6D, 0x44, 0x83, 0xD3, 0xE6,
        0x30, 0x36, 0x66, 0x1C, 0xAD, 0x15, 0x54, 0xD7, 0x5F, 0x5F, 0x76, 0xFE, 0x04, 0xEC, 0x1E, 0x00},
     "transfer/channel-4/SHITMOS", "transfer/channel-4/factory/osmo1q77cw0mmlluxu0wr29fcdd0tdnh78gzhkvhe4n6ulal9qvrtu43qtd0nh8/shitmos", 6},
    {{0xA9, 0x36, 0x9D, 0xA3, 0x22, 0xFA, 0x30, 0x94, 0x3D, 0xF6, 0xD9, 0x57, 0x8E, 0x6C, 0x23, 0x71,
        0xCB, 0xB2, 0xB5, 0x2C, 0x52, 0x34, 0x1C, 0x8C, 0x2E, 0x95, 0x73, 0x66, 0xD5, 0x5A, 0xAD, 0x0C},
     "transfer/channel-4/mbrn", "transfer/channel-4/factory/osmo1s794h9rxggytja3a4pmwul53u98k06zy2qtrdvjnfuxruh7s8yjs6cyxgd/umbrn", 6},
    {{0xAB, 0xE4, 0x1D, 0xB0, 0x1D, 0x23, 0xE8, 0x66, 0x39, 0x65, 0xC8, 0x32, 0xB1, 0x82, 0x58, 0x59,
        0xF8, 0x0E, 0x7C, 0x4C, 0xD4, 0x27, 0x69, 0x49, 0x5D, 0x77, 0xFB, 0x80, 0x38, 0xB0, 0xD0, 0x00},
     "transfer/channel-4/fWITCH", "transfer/channel-4/factory/osmo1dywfmhyc8y0wga7qpzej0x0mgwqg25fj4eccp494w8yafzdpgamsx9ryyv/fWITCH", 9},
    {{0xAC, 0xD3, 0x6C, 0xBC, 0x48, 0xF5, 0x2C, 0x18, 0x5E, 0x04, 0x9D, 0xD1, 0xE2, 0xB2, 0x60, 0xFA,
        0x09, 0x86, 0xE9, 0x85, 0xD5, 0x18, 0x0E, 0xB1, 0x98, 0x70, 0x85, 0x57, 0x2D, 0x02, 0x3E, 0x03},
     "transfer/channel-4/loopedUSDCmars", "transfer/channel-4/factory/osmo1vf6e300hv2qe7r5rln8deft45ewgyytjnwfrdfcv5rgzrfy0s6cswjqf9r/mars-usdc-looped", 6},
    {{0xAE, 0xF3, 0xF2, 0xE5, 0xAC, 0xC6, 0xAE, 0xFC, 0x53, 0x2B, 0x47, 0x73, 0x9A, 0x22, 0xD6, 0x01,
        0xFC, 0xEC, 0x1A, 0xB9, 0x6A, 0x57, 0xD5, 0x8C, 0xEF, 0x07, 0xC7, 0x42, 0x13, 0xC9, 0x89, 0x01},
     "transfer/channel-4/TURD", "transfer/channel-4/factory/osmo1q77cw0mmlluxu0wr29fcdd0tdnh78gzhkvhe4n6ulal9qvrtu43qtd0nh8/turd", 6},
    {{0xB5, 0xAB, 0xC9, 0xFD, 0x30, 0x0A, 0xC2, 0xCD, 0x0C, 0xFC, 0xC8, 0x2E, 0x79, 0xFF, 0x30, 0xC7,
        0x1D, 0x2B, 0xA9, 0x94, 0x2D, 0x2A, 0x30, 0x05, 0x4C, 0x0F, 0x2F, 0x33, 0x8D, 0x7E, 0x4A, 0x07},
     "transfer/channel-4/toro", "transfer/channel-4/factory/osmo1nr8zfakf6jauye3uqa9lrmr5xumee5n42lv92z/toro", 6},
    {{0xB5, 0xC8, 0xC5, 0x78, 0x7B, 0x74, 0x87, 0xDD, 0xDD, 0xC7, 0x98, 0x84, 0xAA, 0xAD, 0xA2, 0x47,
        0xE5, 0x8A, 0xBB, 0x1B, 0x67, 0xEE, 0x7C, 0x54, 0xA4, 0x01, 0x0E, 0xAC, 0xF9, 0xBE, 0xDB, 0x06},
     "transfer/channel-4/ashLAB", "transfer/channel-4/factory/osmo1svj5kd8kzj7xxtrd6ftjk0856ffpyj4egz7f9pd9dge5wr4kwansmefq07/lab.ash", 6},
    {{0xB6, 0x2A, 0xA3, 0xA1, 0xE8, 0x8A, 0xC2, 0x09, 0x59, 0xC8, 0x56, 0xEF, 0x8A, 0x76, 0x21, 0x08,
        0xDA, 0x48, 0xF2, 0x5A, 0x54, 0x42, 0x6C, 0x4D, 0x96, 0xBB, 0x27, 0xED, 0x0D, 0xE6, 0xDA, 0x0C},
     "transfer/channel-4/sqosmo", "transfer/channel-4/factory/osmo1g8qypve6l95xmhgc0fddaecerffymsl7kn9muw/squosmo", 6},
    {{0xBD, 0x20, 0x95, 0x9A, 0xD9, 0xDD, 0xCF, 0x0D, 0x82, 0xD9, 0x16, 0x2E, 0x07, 0x6B, 0xDC, 0x29,
        0x67, 0xE4, 0x3D, 0x56, 0x51, 0x08, 0xFF, 0x46, 0x34, 0xDF, 0x0C, 0xC0, 0x1E, 0x90, 0x50, 0x05},
     "transfer/channel-4/COCA", "transfer/channel-4/factory/osmo1q77cw0mmlluxu0wr29fcdd0tdnh78gzhkvhe4n6ulal9qvrtu43qtd0nh8/coca", 6},
    {{0xBF, 0x8B, 0x03, 0x5D, 0xDA, 0x33, 0x9B, 0x6C, 0xDA, 0x8F, 0x22, 0x1E, 0x79, 0x77, 0x3B, 0x0F,
        0xD8, 0x71, 0xF2, 0x7A, 0x47, 0x29, 0x20, 0xF8, 0x4C, 0x4A, 0xA2, 0xB4, 0xF9, 0x8A, 0x70, 0x0D},
     "transfer/channel-4/allUSDT", "transfer/channel-4/factory/osmo1em6xs47hd82806f5cxgyufguxrrc7l0aqx7nzzptjuqgswczk8csavdxek/alloyed/allUSDT", 6},
    {{0xC0, 0xAD, 0xEA, 0xAA, 0xB5, 0xD6, 0xE2, 0xDB, 0xF9, 0x56, 0xF3, 0xB6, 0x7F, 0x28, 0x69, 0xBB,
        0x38, 0x4A, 0x47, 0x68, 0xB5, 0x13, 0xD5, 0xFB, 0x74, 0x51, 0x66, 0xD6, 0xA9, 0x57, 0x33, 0x06},
     "transfer/channel-4/XTRUMP", "transfer/channel-4/factory/osmo1hg0zf0c9can4tvtulh5gmmxe4jpflre3yewxjl/XTRUMP", 6},
    {{0xC2, 0xF7, 0x61, 0x65, 0x68, 0x0D, 0x6A, 0xB2, 0x87, 0x00, 0x98, 0xAD, 0xD6, 0x29, 0x2E, 0x05,
        0xE4, 0x7B, 0x9D, 0x00, 0x7B, 0xC7, 0xCD, 0xEB, 0x81, 0xFE, 0xAF, 0x61, 0x80, 0x47, 0x76, 0x0E},
     "transfer/channel-4/allETH", "transfer/channel-4/factory/osmo1k6c8jln7ejuqwtqmay3yvzrg3kueaczl96pk067ldg8u835w0yhsw27twm/alloyed/allETH", 18},
    {{0xC3, 0x9E, 0xBC, 0xD5, 0xC9, 0x2D, 0x07, 0x8F, 0xB8, 0x78, 0x67, 0x9F, 0xF0, 0x00, 0xED, 0x67,
        0xC7, 0xD0, 0x4A, 0x5B, 0x77, 0x22, 0xE2, 0xC8, 0xEC, 0x1B, 0x6E, 0x79, 0x4C, 0x15, 0x3A, 0x0D},
     "transfer/channel-4/usdt", "transfer/channel-4/factory/osmo1myv2g72h8dan7n4hx7stt3mmust6ws03zh6gxc7vz4hpmgp5z3lq9aunm9/USDT.rt", 6},
    {{0xC4, 0xD7, 0x60, 0xFC, 0xF7, 0x36, 0x0A, 0xFA, 0xED, 0x6B, 0x05, 0x17, 0xE3, 0x51, 0x01, 0x2D,
        0x39, 0x60, 0x8F, 0xCD, 0xD1, 0x4D, 0xE9, 0xD1, 0x25, 0xA0, 0x45, 0x5F, 0x1B, 0x12, 0x4F, 0x09},
     "transfer/channel-4/ion", "transfer/channel-4/uion", 6},
    {{0xCC, 0x0D, 0x3C, 0x9E, 0xEF, 0x0C, 0x7F, 0xF4, 0xE2, 0x25, 0xEC, 0xA8, 0x5A, 0x30, 0x94, 0x60,
        0x36, 0x91, 0xD2, 0x89, 0xAE, 0xAF, 0x42, 0x8A, 0xB0, 0xD8, 0x73, 0x19, 0xAD, 0x93, 0xA3, 0x02},
     "transfer/channel-2/usdy", "transfer/channel-2/ausdy", 18},
    {{0xCD, 0xAF, 0xCE, 0x9B, 0x1B, 0x46, 0x54, 0x0C, 0x2F, 0x5D, 0x68, 0xA2, 0xCC, 0x94, 0x87, 0x22,
        0x34, 0xFF, 0x3C, 0x1C, 0xD0, 0xE9, 0x78, 0x76, 0x95, 0x7B, 0x41, 0x6C, 0xCB, 0xFC, 0x53, 0x11},
     "transfer/channel-4/BVT1", "transfer/channel-4/factory/osmo16nxtnrnl7lctvnhhpcxqmmpv63n93zgg0ukaveyc0jl4dtad79cs53c3an/BVT", 18},
    {{0xD4, 0xA9, 0xE7, 0x5B, 0x68, 0x59, 0x6B, 0x95, 0x4E, 0x19, 0x19, 0x57, 0x48, 0x38, 0x1E, 0xC4,
        0x36, 0x3B, 0xD9, 0x03, 0xA3, 0xB9, 0x66, 0xB6, 0x89, 0xFE, 0x37, 0x8A, 0xCE, 0xE5, 0x77, 0x01},
     "transfer/channel-4/fSLOTH", "transfer/channel-4/factory/osmo1dywfmhyc8y0wga7qpzej0x0mgwqg25fj4eccp494w8yafzdpgamsx9ryyv/fSLOTH", 9},
    {{0xD5, 0xBD, 0x78, 0xF3, 0xD3, 0xE2, 0x47, 0x4E, 0xA4, 0x18, 0x06, 0x12, 0xE3, 0x2E, 0x3E, 0x27,
        0x46, 0xDA, 0xA9, 0x51, 0x1D, 0x61, 0x8A, 0x8B, 0x0E, 0x5C, 0x64, 0x1C, 0xBC, 0xB9, 0x0B, 0x00},
     "transfer/channel-4/ymos", "transfer/channel-4/factory/osmo1vdvnznwg597qngrq9mnfcfk0am9jdc9y446jewhcqdreqz4r75xq5j5zvy/ymos", 6},
    {{0xD7, 0x5B, 0xA5, 0x87, 0xEC, 0xCE, 0xDA, 0xFD, 0xB4, 0xE8, 0x40, 0xED, 0x6F, 0xA2, 0xF6, 0x49,
        0xD0, 0x74, 0xF6, 0xAE, 0x02, 0x86, 0xB1, 0xDC, 0x73, 0xEA, 0x75, 0xCC, 0x7B, 0x9B, 0xCE, 0x05},
     "transfer/channel-4/COOK", "transfer/channel-4/factory/osmo1q77cw0mmlluxu0wr29fcdd0tdnh78gzhkvhe4n6ulal9qvrtu43qtd0nh8/COOK", 6},
    {{0xD8, 0x41, 0xBA, 0xBC, 0xEA, 0xBF, 0xD9, 0x85, 0xC2, 0x62, 0x58, 0x73, 0x79, 0x24, 0x04, 0xC1,
        0x72, 0x08, 0x56, 0xFA, 0x2B, 0xA1, 0x5B, 0xCC, 0xB1, 0xED, 0x1E, 0xDC, 0x77, 0x38, 0x3C, 0x01},
     "transfer/channel-4/WOSMO", "transfer/channel-4/factory/osmo1pfyxruwvtwk00y8z06dh2lqjdj82ldvy74wzm3/WOSMO", 6},
    {{0xE2, 0x8C, 0xA3, 0xB2, 0xB2, 0x1F, 0xB5, 0xEC, 0xD7, 0x32, 0x6E, 0x71, 0x05, 0x53, 0x1E, 0x7F,
        0x63, 0xE7, 0xF9, 0xD8, 0x64, 0x13, 0x1D, 0x46, 0x17, 0x02, 0x0E, 0x9E, 0x28, 0x8F, 0x9D, 0x0C},
     "transfer/channel-4/wLIBRA", "transfer/channel-4/factory/osmo19hdqma2mj0vnmgcxag6ytswjnr8a3y07q7e70p/wLIBRA", 6},
    {{0xE2, 0xB0, 0x51, 0x0F, 0x51, 0xDE, 0xF5, 0x13, 0x57, 0x73, 0xEE, 0xBE, 0x28, 0xEC, 0x35, 0xBC,
        0x22, 0x86, 0xED, 0x9B, 0x0B, 0x5A, 0xA3, 0x95, 0x2F, 0x83, 0x51, 0x59, 0xD6, 0xCA, 0x0D, 0x07},
     "transfer/channel-4/trx", "transfer/channel-4/factory/osmo14mafhhp337yjj2aujplawz0tks6jd2lel4hkwz4agyzhvvztzaqsqzjq8x/alloyed/allTRX", 6},
    {{0xE4, 0x1D, 0x23, 0xC5, 0x63, 0xFD, 0x2D, 0x7F, 0x94, 0xE2, 0xE0, 0xFB, 0x69, 0x4F, 0x4E, 0x56,
        0xC2, 0x24, 0xD1, 0x4D, 0xFC, 0xE6, 0xE0, 0xF6, 0xAB, 0xE6, 0xB6, 0x78, 0x47, 0xAB, 0x1E, 0x0D},
     "transfer/channel-4/CAC", "transfer/channel-4/factory/osmo1q77cw0mmlluxu0wr29fcdd0tdnh78gzhkvhe4n6ulal9qvrtu43qtd0nh8/cac", 6},
    {{0xE5, 0x08, 0xC2, 0xDB, 0x91, 0x98, 0x2B, 0xD2, 0xCE, 0x53, 0x7F, 0x64, 0x8C, 0x71, 0x98, 0x4D,
        0x2F, 0x6D, 0x25, 0x20, 0x22, 0x3C, 0xDB, 0x83, 0x97, 0xF4, 0x43, 0x9C, 0xD2, 0xF1, 0x68, 0x11},
     "transfer/channel-4/PBB", "transfer/channel-4/factory/osmo1q77cw0mmlluxu0wr29fcdd0tdnh78gzhkvhe4n6ulal9qvrtu43qtd0nh8/pbb", 6},
    {{0xEB, 0x31, 0x26, 0xD5, 0xC4, 0xC7, 0x37, 0x53, 0xEC, 0xB9, 0xE8, 0xA6, 0xB8, 0xDC, 0x42, 0xFB,
        0xA4, 0xBF, 0xB7, 0xAB, 0x83, 0x61, 0x17, 0x61, 0x6E, 0x2B, 0x60, 0x91, 0x45, 0x7D, 0x23, 0x11},
     "transfer/channel-4/milkTIA", "transfer/channel-4/factory/osmo1f5vfcph2dvfeqcqkhetwv75fda69z7e5c2dldm3kvgj23crkv6wqcn47a0/umilkTIA", 6},
    {{0xF4, 0xAD, 0x27, 0x86, 0xC1, 0xC6, 0xE6, 0x99, 0xFF, 0x32, 0x99, 0xB1, 0x60, 0x39, 0x06, 0xC8,
        0x81, 0xDB, 0x55, 0x78, 0x76, 0x69, 0x99, 0xB7, 0x7A, 0x49, 0xC2, 0x63, 0x2F, 0x5A, 0xBB, 0x0B},
     "transfer/channel-4/BAG", "transfer/channel-4/factory/osmo1q77cw0mmlluxu0wr29fcdd0tdnh78gzhkvhe4n6ulal9qvrtu43qtd0nh8/bag", 6},
    {{0xF5, 0x9D, 0x87, 0xFC, 0x7E, 0x0C, 0x0A, 0x76, 0x07, 0xF1, 0xEB, 0xED, 0xD9, 0x0E, 0xA4, 0x30,
        0x24, 0xC5, 0x49, 0x87, 0xBC, 0xF0, 0x4B, 0x56, 0x9F, 0xBD, 0x77, 0xD6, 0xB9, 0xB0, 0x80, 0x00},
     "transfer/channel-4/LAB", "transfer/channel-4/factory/osmo17fel472lgzs87ekt9dvk0zqyh5gl80sqp4sk4n/LAB", 6},
    {{0xF6, 0xC2, 0x83, 0x49, 0x53, 0x3D, 0xA9, 0x4D, 0x06, 0xDC, 0x04, 0xA9, 0x66, 0xD7, 0x9D, 0xF3,
        0x02, 0x41, 0xF1, 0xF8, 0x5D, 0x69, 0xD0, 0x2D, 0xCF, 0x16, 0x71, 0x13, 0x9B, 0x49, 0x33, 0x12},
     "transfer/channel-4/arb", "transfer/channel-4/factory/osmo1p7x454ex08s4f9ztmm7wfv7lvtgdkfztj2u7v7fezfcauy85q35qmqrdpk/alloyed/allARB", 12}
};
