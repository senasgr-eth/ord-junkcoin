use bitcoin::util::address;
use {super::*, clap::ValueEnum};

#[derive(Default, ValueEnum, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Chain {
  #[default]
  #[clap(alias("main"))]
  Mainnet,
  #[clap(alias("test"))]
  Testnet,
  Signet,
  Regtest,
}

impl Chain {
  pub(crate) fn network(self) -> Network {
    match self {
      Self::Mainnet => Network::Bitcoin,
      Self::Testnet => Network::Testnet,
      Self::Signet => Network::Signet,
      Self::Regtest => Network::Regtest,
    }
  }

  pub(crate) fn default_rpc_port(self) -> u16 {
    match self {
      Self::Mainnet => 9771,
      Self::Regtest => 19917,
      Self::Signet => 19917,
      Self::Testnet => 44555,
    }
  }

  pub(crate) fn inscription_content_size_limit(self) -> Option<usize> {
    match self {
      Self::Mainnet | Self::Regtest => None,
      Self::Testnet | Self::Signet => None,
    }
  }

  pub(crate) fn first_inscription_height(self) -> u32 {
    match self {
      Self::Mainnet => 140000,
      Self::Regtest => 0,
      Self::Signet => 0,
      Self::Testnet => 4250000,
    }
  }

  pub(crate) fn first_dune_height(self) -> u32 {
    match self {
      Self::Mainnet => 5084000,
      Self::Regtest => 0,
      Self::Signet => 0,
      Self::Testnet => 4250000,
    }
  }

  pub(crate) fn genesis_block(self) -> Block {
    let genesis_hex: &str = match self {
        Self::Mainnet => 
            // From chainparams.cpp mainnet params:
            // time: 1367394064
            // nonce: 112158625
            // bits: 0x1e0ffff0
            // merkle root: 3de124b0274307911fe12550e96bf76cb92c12835db6cb19f82658b8aca1dbc8
            "01000000" + // version
            "0000000000000000000000000000000000000000000000000000000000000000" + // prev block
            "c8dba1ac8b65829fb16cdbd83528c192cb76bf96e55021e9170743270b124e3d" + // merkle root
            "40029651" + // timestamp (1367394064)
            "f0ff0f1e" + // bits
            "21b0b006" + // nonce (112158625)
            "01" + // tx count
            "01000000" + // tx version
            "01" + // input count
            "0000000000000000000000000000000000000000000000000000000000000000" + // prev tx
            "ffffffff" + // prev index
            "10" + // script length (16 bytes)
            "04ffff001d0104084e696e746f6e646f" + // script sig
            "ffffffff" + // sequence
            "01" + // output count
            "0058850c02000000" + // value 
            "43" + // script length 
            "41040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9ac" + // pubkey script
            "00000000", // locktime

        Self::Testnet | Self::Regtest => 
            // From chainparams.cpp testnet/regtest params:
            // time: 1369199888
            // nonce: 12097647 
            // bits: 0x1e0ffff0
            "01000000" +
            "0000000000000000000000000000000000000000000000000000000000000000" +
            "c8dba1ac8b65829fb16cdbd83528c192cb76bf96e55021e9170743270b124e3d" +
            "909a9151" + // timestamp (1369199888)
            "f0ff0f1e" + // bits
            "7f8bb800" + // nonce (12097647)
            "01" +
            "01000000" +
            "01" +
            "0000000000000000000000000000000000000000000000000000000000000000" +
            "ffffffff" +
            "10" +
            "04ffff001d0104084e696e746f6e646f" +
            "ffffffff" +
            "01" +
            "0058850c02000000" +
            "43" +
            "41040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9ac" +
            "00000000",

        Self::Signet => Self::Mainnet.genesis_block(), // Fallback to mainnet genesis
    };
    let genesis_buf: Vec<u8> = hex::decode(genesis_hex).unwrap();
    bitcoin::consensus::deserialize(&genesis_buf).unwrap()
}

  pub(crate) fn address_from_script(
    self,
    script: &Script,
  ) -> Result<Address, address::Error> {
    Address::from_script(script, self.network())
  }

  pub(crate) fn join_with_data_dir(self, data_dir: &Path) -> PathBuf {
    match self {
      Self::Mainnet => data_dir.to_owned(),
      Self::Testnet => data_dir.join("testnet3"),
      Self::Signet => data_dir.join("signet"),
      Self::Regtest => data_dir.join("regtest"),
    }
  }
}

impl Display for Chain {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Mainnet => "mainnet",
        Self::Regtest => "regtest",
        Self::Signet => "signet",
        Self::Testnet => "testnet",
      }
    )
  }
}
