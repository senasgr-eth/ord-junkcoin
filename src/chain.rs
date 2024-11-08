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
      Self::Mainnet => "010000000000000000000000000000000000000000000000000000000000000000000000c8dba1ac8b65829fb16cdbd83528c192cb76bf96e55021e9170743270b124e3d40bc9051f0ff0f1e21b0b006010100000001000000000000000000000000000000000000000000000000000000000000000000000000596e05ffff001d01044d57656420576179203120323031333a2053706f7420676f6c642066656c6c20312e332070657263656e7420746f2024312c3435372e393020616e206f756e636520627920333a313120702e6d2e2045445420283139313120474d5429ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000",
      Self::Regtest => "010000000000000000000000000000000000000000000000000000000000000000000000c8dba1ac8b65829fb16cdbd83528c192cb76bf96e55021e9170743270b124e3d90a51951f0ff0f1e7f8bb800010100000001000000000000000000000000000000000000000000000000000000000000000000000000596e05ffff001d01044d57656420576179203120323031333a2053706f7420676f6c642066656c6c20312e332070657263656e7420746f2024312c3435372e393020616e206f756e636520627920333a313120702e6d2e2045445420283139313120474d5429ffffffff0100802c5801000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000",
      Self::Signet => "010000000000000000000000000000000000000000000000000000000000000000000000696ad20e2dd4365c7459b4a4a5af743d5e92c6da3229e6532cd605f6533f2a5bb9a7f052f0ff0f1ef7390f000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff1004ffff001d0104084e696e746f6e646fffffffff010058850c020000004341040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9ac00000000",
      Self::Testnet => "010000000000000000000000000000000000000000000000000000000000000000000000696ad20e2dd4365c7459b4a4a5af743d5e92c6da3229e6532cd605f6533f2a5bb9a7f052f0ff0f1ef7390f000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff1004ffff001d0104084e696e746f6e646fffffffff010058850c020000004341040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9ac00000000",
    };
    let genesis_buf: Vec<u8> = hex::decode(genesis_hex).unwrap();
    bitcoin::consensus::deserialize(&genesis_buf). unwrap()
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
