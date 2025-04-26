use std::collections::HashMap;

mod preset;

use preset::PRESET_TOKENS;

use super::*;

#[derive(Serialize, Deserialize)]
pub struct Tokens {
    #[serde(skip, default = "init_custom_tokens")]
    stable_custom_tokens: StableBTreeMap<CanisterId, TokenInfo>,
    #[serde(skip)]
    custom_tokens: Option<HashMap<CanisterId, TokenInfo>>,
    frozen_tokens: HashSet<CanisterId>, // frozen token
}
impl Default for Tokens {
    fn default() -> Self {
        Self {
            stable_custom_tokens: init_custom_tokens(),
            custom_tokens: Default::default(),
            frozen_tokens: Default::default(),
        }
    }
}

impl Tokens {
    fn get_stable_custom_tokens(&self) -> HashMap<CanisterId, TokenInfo> {
        let mut custom_tokens = HashMap::with_capacity(self.stable_custom_tokens.len() as usize);
        for token in self.stable_custom_tokens.values() {
            custom_tokens.insert(token.canister_id, token);
        }
        custom_tokens
    }
    fn load_stable_custom_tokens(&mut self) {
        if self.custom_tokens.is_some() {
            return;
        }
        self.custom_tokens = Some(self.get_stable_custom_tokens());
    }
    pub fn get_custom_tokens(&self) -> Cow<'_, HashMap<CanisterId, TokenInfo>> {
        if let Some(custom_tokens) = &self.custom_tokens {
            return Cow::Borrowed(custom_tokens);
        }
        let custom_tokens = self.get_stable_custom_tokens();
        Cow::Owned(custom_tokens)
    }
    pub fn query_custom_tokens(&mut self) -> Cow<'_, HashMap<CanisterId, TokenInfo>> {
        self.load_stable_custom_tokens();
        self.get_custom_tokens()
    }
    pub fn get_all_tokens(&self) -> HashMap<CanisterId, Cow<'_, TokenInfo>> {
        let custom_tokens = self.get_custom_tokens();
        let mut all_tokens = HashMap::with_capacity(custom_tokens.len() + PRESET_TOKENS.len());
        match custom_tokens {
            Cow::Borrowed(custom_tokens) => {
                for token in custom_tokens.values() {
                    all_tokens.insert(token.canister_id, Cow::Borrowed(token));
                }
            }
            Cow::Owned(custom_tokens) => {
                for (_, token) in custom_tokens {
                    all_tokens.insert(token.canister_id, Cow::Owned(token));
                }
            }
        }
        for (_, token) in PRESET_TOKENS.iter() {
            all_tokens.insert(token.canister_id, Cow::Borrowed(token));
        }
        all_tokens
    }
}
