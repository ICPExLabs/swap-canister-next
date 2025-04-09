// cSpell:words eurc kinic dolr goldao trax neutrinite sneed elna icfc yuku motoko icpcc origyn dogmi icvc nfid nfidw draggin
use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::*;

// =================================== token info ===================================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenInfo {
    pub canister_id: CanisterId,
    #[allow(unused)]
    pub name: String,
    #[allow(unused)]
    pub symbol: String,
    #[allow(unused)]
    pub decimals: u8,
    pub fee: Nat,
}

impl TokenInfo {
    fn new(
        canister_id: &'static str,
        name: &'static str,
        symbol: &'static str,
        decimals: u8,
        fee: u128,
    ) -> Self {
        #[allow(clippy::unwrap_used)] // ? SAFETY
        Self {
            canister_id: CanisterId::from_text(canister_id).unwrap(),
            name: name.into(),
            symbol: symbol.into(),
            decimals,
            fee: Nat::from(fee),
        }
    }
}

// =================================== tokens ===================================

#[rustfmt::skip]
fn init_tokens() -> HashMap<CanisterId, TokenInfo> {
    let tokens: Vec<TokenInfo> = vec![
        // ICP
        TokenInfo::new("ryjl3-tyaaa-aaaaa-aaaba-cai", "Internet Computer", "ICP", 8, 10_000), // fee 0.0001 ICP
        // CK
        TokenInfo::new("mxzaz-hqaaa-aaaar-qaada-cai", "ckBTC",    "ckBTC",     8,                            10), // fee 0.0000001 ckBTC
        TokenInfo::new("cngnf-vqaaa-aaaar-qag4q-cai", "ckUSDT",   "ckUSDT",    6,                        10_000), // fee 0.01 ckUSDT
        TokenInfo::new("xevnm-gaaaa-aaaar-qafnq-cai", "ckUSDC",   "ckUSDC",    6,                        10_000), // fee 0.01 ckUSDC
        TokenInfo::new("ss2fx-dyaaa-aaaar-qacoq-cai", "ckETH",    "ckETH",    18,             2_000_000_000_000), // fee 0.000002 ckETH
        TokenInfo::new("g4tto-rqaaa-aaaar-qageq-cai", "ckLINK",   "ckLINK",   18,           100_000_000_000_000), // fee 0.0001 ckLINK
        TokenInfo::new("ebo5g-cyaaa-aaaar-qagla-cai", "ckOCT",    "ckOCT",    18,        34_000_000_000_000_000), // fee 0.034 ckOCT
        TokenInfo::new("pe5t5-diaaa-aaaar-qahwa-cai", "ckEURC",   "ckEURC",    6,                        10_000), // fee 0.01 ckEURC
        TokenInfo::new("nza5v-qaaaa-aaaar-qahzq-cai", "ckXAUT",   "ckXAUT",    6,                             1), // fee 0.000001 ckXAUT
        TokenInfo::new("j2tuh-yqaaa-aaaar-qahcq-cai", "ckWSTETH", "ckWSTETH", 18,             1_000_000_000_000), // fee 0.000001 ckWSTETH
        TokenInfo::new("ilzky-ayaaa-aaaar-qahha-cai", "ckUNI",    "ckUNI",    18,         1_000_000_000_000_000), // fee 0.001 ckUNI
        TokenInfo::new("fxffn-xiaaa-aaaar-qagoa-cai", "ckSHIB",   "ckSHIB",   18,   100_000_000_000_000_000_000), // fee 100 ckSHIB
        TokenInfo::new("etik7-oiaaa-aaaar-qagia-cai", "ckPEPE",   "ckPEPE",   18, 1_000_000_000_000_000_000_000), // fee 1000 ckPEPE
        TokenInfo::new("bptq2-faaaa-aaaar-qagxq-cai", "ckWBTC",   "ckWBTC",    8,                            10), // fee 0.0000001 ckWBTC
        // SNS
        TokenInfo::new("2ouva-viaaa-aaaaq-aaamq-cai", "CHAT",                 "CHAT",   8,                   100_000), // fee 0.001 CHAT
        TokenInfo::new("73mez-iiaaa-aaaaq-aaasq-cai", "KINIC",                "KINIC",  8,                   100_000), // fee 0.001 KINIC
        TokenInfo::new("6rdgd-kyaaa-aaaaq-aaavq-cai", "DOLR AI",              "DOLR",   8,                   100_000), // fee 0.001 DOLR
        TokenInfo::new("4c4fd-caaaa-aaaaq-aaa3a-cai", "GHOST",                "GHOST",  8,               100_000_000), // fee 1 GHOST
        TokenInfo::new("xsi2v-cyaaa-aaaaq-aabfq-cai", "DecideAI",             "DCD",    8,                    10_000), // fee 0.0001 DCD
        TokenInfo::new("uf2wh-taaaa-aaaaq-aabna-cai", "CatalyzeDAO",          "CTZ",    8,                   100_000), // fee 0.001 CTZ
        TokenInfo::new("vtrom-gqaaa-aaaaq-aabia-cai", "BoomDAO",              "BOOM",   8,                   100_000), // fee 0.001 BOOM
        TokenInfo::new("rffwt-piaaa-aaaaq-aabqq-cai", "Seers",                "SEER",   8,                   100_000), // fee 0.001 SEER
        TokenInfo::new("rxdbk-dyaaa-aaaaq-aabtq-cai", "Nuance",               "NUA",    8,                   100_000), // fee 0.001 NUA
        TokenInfo::new("qbizb-wiaaa-aaaaq-aabwq-cai", "Sonic",                "SONIC",  8,                   100_000), // fee 0.001 SONIC
        TokenInfo::new("tyyy3-4aaaa-aaaaq-aab7a-cai", "GOLDAO",               "GOLDAO", 8,                   100_000), // fee 0.001 GOLDAO
        TokenInfo::new("emww2-4yaaa-aaaaq-aacbq-cai", "TRAX",                 "TRAX",   8,                   100_000), // fee 0.001 TRAX
        TokenInfo::new("f54if-eqaaa-aaaaq-aacea-cai", "Neutrinite",           "NTN",    8,                    10_000), // fee 0.0001 NTN
        TokenInfo::new("hvgxa-wqaaa-aaaaq-aacia-cai", "Sneed DAO",            "SNEED",  8,                     1_000), // fee 0.00001 SNEED
        TokenInfo::new("hhaaz-2aaaa-aaaaq-aacla-cai", "ICLighthouse DAO",     "ICL",    8,                 1_000_000), // fee 0.01 ICL
        TokenInfo::new("gemj7-oyaaa-aaaaq-aacnq-cai", "ELNA",                 "ELNA",   8,                   100_000), // fee 0.001 ELNA
        TokenInfo::new("ddsp7-7iaaa-aaaaq-aacqq-cai", "ICFC",                 "ICFC",   8,                   100_000), // fee 0.001 ICFC
        TokenInfo::new("druyg-tyaaa-aaaaq-aactq-cai", "ICPanda",              "PANDA",  8,                    10_000), // fee 0.0001 PANDA
        TokenInfo::new("ca6gz-lqaaa-aaaaq-aacwa-cai", "ICPSwap Token",        "ICS",    8,                 1_000_000), // fee 0.01 ICS
        TokenInfo::new("atbfz-diaaa-aaaaq-aacyq-cai", "Yuku AI",              "YUKU",   8,                 1_000_000), // fee 0.01 YUKU
        TokenInfo::new("bliq2-niaaa-aaaaq-aac4q-cai", "ESTATE",               "EST",    8,                   100_000), // fee 0.001 EST
        TokenInfo::new("k45jy-aiaaa-aaaaq-aadcq-cai", "Motoko",               "MOTOKO", 8,               100_000_000), // fee 1 MOTOKO
        TokenInfo::new("lrtnw-paaaa-aaaaq-aadfa-cai", "ICPCC DAO LLC",        "CONF",   8,                    10_000), // fee 0.0001 CONF
        TokenInfo::new("lkwrt-vyaaa-aaaaq-aadhq-cai", "ORIGYN",               "OGY",    8,                   200_000), // fee 0.002 OGY
        TokenInfo::new("jcmow-hyaaa-aaaaq-aadlq-cai", "WaterNeuron",          "WTN",    8,                 1_000_000), // fee 0.01 WTN
        TokenInfo::new("itgqj-7qaaa-aaaaq-aadoa-cai", "----",                 "----",   8, 1_000_000_000_000_000_000), // fee 10000000000 ----
        TokenInfo::new("np5km-uyaaa-aaaaq-aadrq-cai", "DOGMI",                "DOGMI",  8,           100_000_000_000), // fee 1000 DOGMI
        TokenInfo::new("m6xut-mqaaa-aaaaq-aadua-cai", "ICVC",                 "ICVC",   8,                    10_000), // fee 0.0001 ICVC
        TokenInfo::new("o7oak-iyaaa-aaaaq-aadzq-cai", "KongSwap",             "KONG",   8,                    10_000), // fee 0.0001 KONG
        TokenInfo::new("o4zzi-qaaaa-aaaaq-aaeeq-cai", "FomoWell",             "WELL",   8,                   100_000), // fee 0.001 WELL
        TokenInfo::new("oj6if-riaaa-aaaaq-aaeha-cai", "ALICE",                "ALICE",  8,               500_000_000), // fee 5 ALICE
        TokenInfo::new("mih44-vaaaa-aaaaq-aaekq-cai", "NFID Wallet",          "NFIDW",  8,                    10_000), // fee 0.0001 NFIDW
        TokenInfo::new("nfjys-2iaaa-aaaaq-aaena-cai", "FUEL",                 "FUEL",   8,                   100_000), // fee 0.001 FUEL
        TokenInfo::new("ifwyg-gaaaa-aaaaq-aaeqq-cai", "ICExplorer",           "ICE",    8,                   100_000), // fee 0.001 ICE
        TokenInfo::new("ixqp7-kqaaa-aaaaq-aaetq-cai", "Personal DAO",         "DAO",    8,                    10_000), // fee 0.0001 DAO
        TokenInfo::new("jg2ra-syaaa-aaaaq-aaewa-cai", "Cecil The Lion DAO",   "CECIL",  8,                 1_000_000), // fee 0.01 CECIL
        TokenInfo::new("lvfsa-2aaaa-aaaaq-aaeyq-cai", "ICPEx",                "ICX",    8,                   100_000), // fee 0.001 ICX
        TokenInfo::new("zfcdd-tqaaa-aaaaq-aaaga-cai", "Draggin Karma Points", "DKP",    8,                   100_000), // fee 0.001 DKP
    ];
    tokens.into_iter().map(|token| (token.canister_id, token)).collect()
}

#[allow(unused)]
pub static TOKENS: Lazy<HashMap<CanisterId, TokenInfo>> = Lazy::new(init_tokens);
