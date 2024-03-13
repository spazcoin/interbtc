(function() {var implementors = {
"bitcoin":[["impl MaxEncodedLen for <a class=\"struct\" href=\"bitcoin/types/struct.BlockChain.html\" title=\"struct bitcoin::types::BlockChain\">BlockChain</a>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"bitcoin/struct.PublicKey.html\" title=\"struct bitcoin::PublicKey\">PublicKey</a>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"bitcoin/types/struct.H256Le.html\" title=\"struct bitcoin::types::H256Le\">H256Le</a>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"bitcoin/types/struct.BlockHeader.html\" title=\"struct bitcoin::types::BlockHeader\">BlockHeader</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bitcoin/enum.Address.html\" title=\"enum bitcoin::Address\">Address</a>"]],
"btc_relay":[["impl&lt;BlockNumber&gt; MaxEncodedLen for <a class=\"struct\" href=\"btc_relay/types/struct.RichBlockHeader.html\" title=\"struct btc_relay::types::RichBlockHeader\">RichBlockHeader</a>&lt;BlockNumber&gt;<span class=\"where fmt-newline\">where\n    BlockNumber: MaxEncodedLen,</span>"]],
"clients_info":[["impl&lt;Uri, Hash&gt; MaxEncodedLen for <a class=\"struct\" href=\"clients_info/struct.ClientRelease.html\" title=\"struct clients_info::ClientRelease\">ClientRelease</a>&lt;Uri, Hash&gt;<span class=\"where fmt-newline\">where\n    Uri: MaxEncodedLen,\n    Hash: MaxEncodedLen,</span>"]],
"collator_selection":[["impl&lt;AccountId, Balance&gt; MaxEncodedLen for <a class=\"struct\" href=\"collator_selection/pallet/struct.CandidateInfo.html\" title=\"struct collator_selection::pallet::CandidateInfo\">CandidateInfo</a>&lt;AccountId, Balance&gt;<span class=\"where fmt-newline\">where\n    AccountId: MaxEncodedLen,\n    Balance: MaxEncodedLen,</span>"]],
"democracy":[["impl&lt;BlockNumber, Proposal, Balance&gt; MaxEncodedLen for <a class=\"enum\" href=\"democracy/enum.ReferendumInfo.html\" title=\"enum democracy::ReferendumInfo\">ReferendumInfo</a>&lt;BlockNumber, Proposal, Balance&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"democracy/struct.ReferendumStatus.html\" title=\"struct democracy::ReferendumStatus\">ReferendumStatus</a>&lt;BlockNumber, Proposal, Balance&gt;: MaxEncodedLen,\n    BlockNumber: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"democracy/enum.VoteThreshold.html\" title=\"enum democracy::VoteThreshold\">VoteThreshold</a>"],["impl&lt;BlockNumber, Proposal, Balance&gt; MaxEncodedLen for <a class=\"struct\" href=\"democracy/struct.ReferendumStatus.html\" title=\"struct democracy::ReferendumStatus\">ReferendumStatus</a>&lt;BlockNumber, Proposal, Balance&gt;<span class=\"where fmt-newline\">where\n    BlockNumber: MaxEncodedLen,\n    Proposal: MaxEncodedLen,\n    <a class=\"struct\" href=\"democracy/struct.Tally.html\" title=\"struct democracy::Tally\">Tally</a>&lt;Balance&gt;: MaxEncodedLen,</span>"],["impl&lt;Balance&gt; MaxEncodedLen for <a class=\"struct\" href=\"democracy/struct.Tally.html\" title=\"struct democracy::Tally\">Tally</a>&lt;Balance&gt;<span class=\"where fmt-newline\">where\n    Balance: MaxEncodedLen,</span>"],["impl&lt;Balance, MaxVotes: Get&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt;&gt; MaxEncodedLen for <a class=\"struct\" href=\"democracy/struct.Voting.html\" title=\"struct democracy::Voting\">Voting</a>&lt;Balance, MaxVotes&gt;<span class=\"where fmt-newline\">where\n    BoundedVec&lt;(<a class=\"type\" href=\"democracy/type.ReferendumIndex.html\" title=\"type democracy::ReferendumIndex\">ReferendumIndex</a>, <a class=\"struct\" href=\"democracy/struct.Vote.html\" title=\"struct democracy::Vote\">Vote</a>&lt;Balance&gt;), MaxVotes&gt;: MaxEncodedLen,</span>"],["impl&lt;Balance&gt; MaxEncodedLen for <a class=\"struct\" href=\"democracy/struct.Vote.html\" title=\"struct democracy::Vote\">Vote</a>&lt;Balance&gt;<span class=\"where fmt-newline\">where\n    Balance: MaxEncodedLen,</span>"]],
"dex_general":[["impl&lt;Balance, Account&gt; MaxEncodedLen for <a class=\"struct\" href=\"dex_general/struct.PairMetadata.html\" title=\"struct dex_general::PairMetadata\">PairMetadata</a>&lt;Balance, Account&gt;<span class=\"where fmt-newline\">where\n    Account: MaxEncodedLen,\n    Balance: MaxEncodedLen,</span>"],["impl&lt;Balance, BlockNumber, Account&gt; MaxEncodedLen for <a class=\"enum\" href=\"dex_general/enum.PairStatus.html\" title=\"enum dex_general::PairStatus\">PairStatus</a>&lt;Balance, BlockNumber, Account&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"dex_general/struct.PairMetadata.html\" title=\"struct dex_general::PairMetadata\">PairMetadata</a>&lt;Balance, Account&gt;: MaxEncodedLen,\n    <a class=\"struct\" href=\"dex_general/struct.BootstrapParameter.html\" title=\"struct dex_general::BootstrapParameter\">BootstrapParameter</a>&lt;Balance, BlockNumber, Account&gt;: MaxEncodedLen,</span>"],["impl&lt;Balance, BlockNumber, Account&gt; MaxEncodedLen for <a class=\"struct\" href=\"dex_general/struct.BootstrapParameter.html\" title=\"struct dex_general::BootstrapParameter\">BootstrapParameter</a>&lt;Balance, BlockNumber, Account&gt;<span class=\"where fmt-newline\">where\n    Balance: MaxEncodedLen,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(Balance, Balance)</a>: MaxEncodedLen,\n    BlockNumber: MaxEncodedLen,\n    Account: MaxEncodedLen,</span>"]],
"escrow":[["impl&lt;Balance, BlockNumber&gt; MaxEncodedLen for <a class=\"struct\" href=\"escrow/struct.LockedBalance.html\" title=\"struct escrow::LockedBalance\">LockedBalance</a>&lt;Balance, BlockNumber&gt;<span class=\"where fmt-newline\">where\n    Balance: MaxEncodedLen,\n    BlockNumber: MaxEncodedLen,</span>"],["impl&lt;Balance, BlockNumber&gt; MaxEncodedLen for <a class=\"struct\" href=\"escrow/struct.Point.html\" title=\"struct escrow::Point\">Point</a>&lt;Balance, BlockNumber&gt;<span class=\"where fmt-newline\">where\n    Balance: MaxEncodedLen,\n    BlockNumber: MaxEncodedLen,</span>"]],
"farming":[["impl&lt;Balance&gt; MaxEncodedLen for <a class=\"struct\" href=\"farming/struct.RewardSchedule.html\" title=\"struct farming::RewardSchedule\">RewardSchedule</a>&lt;Balance&gt;<span class=\"where fmt-newline\">where\n    Balance: HasCompact + MaxEncodedLen,</span>"]],
"fee":[["impl MaxEncodedLen for <a class=\"enum\" href=\"fee/types/enum.Version.html\" title=\"enum fee::types::Version\">Version</a>"]],
"interbtc_primitives":[["impl MaxEncodedLen for <a class=\"enum\" href=\"interbtc_primitives/oracle/enum.Key.html\" title=\"enum interbtc_primitives::oracle::Key\">Key</a>"],["impl&lt;AccountId, BlockNumber, Balance, CurrencyId&gt; MaxEncodedLen for <a class=\"struct\" href=\"interbtc_primitives/issue/struct.IssueRequest.html\" title=\"struct interbtc_primitives::issue::IssueRequest\">IssueRequest</a>&lt;AccountId, BlockNumber, Balance, CurrencyId&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"interbtc_primitives/struct.VaultId.html\" title=\"struct interbtc_primitives::VaultId\">VaultId</a>&lt;AccountId, CurrencyId&gt;: MaxEncodedLen,\n    BlockNumber: MaxEncodedLen,\n    Balance: MaxEncodedLen,\n    CurrencyId: MaxEncodedLen + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,\n    AccountId: MaxEncodedLen,</span>"],["impl&lt;AccountId, BlockNumber, Balance, CurrencyId: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>&gt; MaxEncodedLen for <a class=\"struct\" href=\"interbtc_primitives/replace/struct.ReplaceRequest.html\" title=\"struct interbtc_primitives::replace::ReplaceRequest\">ReplaceRequest</a>&lt;AccountId, BlockNumber, Balance, CurrencyId&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"interbtc_primitives/struct.VaultId.html\" title=\"struct interbtc_primitives::VaultId\">VaultId</a>&lt;AccountId, CurrencyId&gt;: MaxEncodedLen,\n    Balance: MaxEncodedLen,\n    BlockNumber: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"interbtc_primitives/enum.LpToken.html\" title=\"enum interbtc_primitives::LpToken\">LpToken</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"interbtc_primitives/redeem/enum.RedeemRequestStatus.html\" title=\"enum interbtc_primitives::redeem::RedeemRequestStatus\">RedeemRequestStatus</a>"],["impl&lt;AccountId, CurrencyId: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>&gt; MaxEncodedLen for <a class=\"struct\" href=\"interbtc_primitives/struct.VaultId.html\" title=\"struct interbtc_primitives::VaultId\">VaultId</a>&lt;AccountId, CurrencyId&gt;<span class=\"where fmt-newline\">where\n    AccountId: MaxEncodedLen,\n    <a class=\"struct\" href=\"interbtc_primitives/struct.VaultCurrencyPair.html\" title=\"struct interbtc_primitives::VaultCurrencyPair\">VaultCurrencyPair</a>&lt;CurrencyId&gt;: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"interbtc_primitives/replace/enum.ReplaceRequestStatus.html\" title=\"enum interbtc_primitives::replace::ReplaceRequestStatus\">ReplaceRequestStatus</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"interbtc_primitives/enum.TokenSymbol.html\" title=\"enum interbtc_primitives::TokenSymbol\">TokenSymbol</a>"],["impl&lt;AccountId, BlockNumber, Balance, CurrencyId: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>&gt; MaxEncodedLen for <a class=\"struct\" href=\"interbtc_primitives/redeem/struct.RedeemRequest.html\" title=\"struct interbtc_primitives::redeem::RedeemRequest\">RedeemRequest</a>&lt;AccountId, BlockNumber, Balance, CurrencyId&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"interbtc_primitives/struct.VaultId.html\" title=\"struct interbtc_primitives::VaultId\">VaultId</a>&lt;AccountId, CurrencyId&gt;: MaxEncodedLen,\n    BlockNumber: MaxEncodedLen,\n    Balance: MaxEncodedLen,\n    AccountId: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"interbtc_primitives/enum.CurrencyId.html\" title=\"enum interbtc_primitives::CurrencyId\">CurrencyId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"interbtc_primitives/issue/enum.IssueRequestStatus.html\" title=\"enum interbtc_primitives::issue::IssueRequestStatus\">IssueRequestStatus</a>"],["impl&lt;CurrencyId&gt; MaxEncodedLen for <a class=\"struct\" href=\"interbtc_primitives/struct.VaultCurrencyPair.html\" title=\"struct interbtc_primitives::VaultCurrencyPair\">VaultCurrencyPair</a>&lt;CurrencyId&gt;<span class=\"where fmt-newline\">where\n    CurrencyId: MaxEncodedLen + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,</span>"]],
"interlay_runtime_parachain":[["impl MaxEncodedLen for <a class=\"enum\" href=\"interlay_runtime_parachain/enum.RuntimeFreezeReason.html\" title=\"enum interlay_runtime_parachain::RuntimeFreezeReason\">RuntimeFreezeReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"interlay_runtime_parachain/enum.RuntimeHoldReason.html\" title=\"enum interlay_runtime_parachain::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"interlay_runtime_parachain/enum.RuntimeSlashReason.html\" title=\"enum interlay_runtime_parachain::RuntimeSlashReason\">RuntimeSlashReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"interlay_runtime_parachain/enum.OriginCaller.html\" title=\"enum interlay_runtime_parachain::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"interlay_runtime_parachain/enum.ProxyType.html\" title=\"enum interlay_runtime_parachain::ProxyType\">ProxyType</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"interlay_runtime_parachain/enum.RuntimeLockId.html\" title=\"enum interlay_runtime_parachain::RuntimeLockId\">RuntimeLockId</a>"]],
"issue":[["impl MaxEncodedLen for <a class=\"enum\" href=\"issue/types/enum.Version.html\" title=\"enum issue::types::Version\">Version</a>"]],
"kintsugi_runtime_parachain":[["impl MaxEncodedLen for <a class=\"enum\" href=\"kintsugi_runtime_parachain/enum.RuntimeFreezeReason.html\" title=\"enum kintsugi_runtime_parachain::RuntimeFreezeReason\">RuntimeFreezeReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"kintsugi_runtime_parachain/enum.ProxyType.html\" title=\"enum kintsugi_runtime_parachain::ProxyType\">ProxyType</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"kintsugi_runtime_parachain/enum.RuntimeHoldReason.html\" title=\"enum kintsugi_runtime_parachain::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"kintsugi_runtime_parachain/enum.OriginCaller.html\" title=\"enum kintsugi_runtime_parachain::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"kintsugi_runtime_parachain/enum.RuntimeSlashReason.html\" title=\"enum kintsugi_runtime_parachain::RuntimeSlashReason\">RuntimeSlashReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"kintsugi_runtime_parachain/enum.RuntimeLockId.html\" title=\"enum kintsugi_runtime_parachain::RuntimeLockId\">RuntimeLockId</a>"]],
"loans":[["impl MaxEncodedLen for <a class=\"struct\" href=\"loans/struct.CurveModel.html\" title=\"struct loans::CurveModel\">CurveModel</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"loans/enum.InterestRateModel.html\" title=\"enum loans::InterestRateModel\">InterestRateModel</a>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"loans/struct.JumpModel.html\" title=\"struct loans::JumpModel\">JumpModel</a>"],["impl&lt;BlockNumber, Balance&gt; MaxEncodedLen for <a class=\"struct\" href=\"loans/struct.RewardMarketState.html\" title=\"struct loans::RewardMarketState\">RewardMarketState</a>&lt;BlockNumber, Balance&gt;<span class=\"where fmt-newline\">where\n    Balance: MaxEncodedLen,\n    BlockNumber: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"loans/enum.Versions.html\" title=\"enum loans::Versions\">Versions</a>"],["impl&lt;Balance&gt; MaxEncodedLen for <a class=\"struct\" href=\"loans/struct.BorrowSnapshot.html\" title=\"struct loans::BorrowSnapshot\">BorrowSnapshot</a>&lt;Balance&gt;<span class=\"where fmt-newline\">where\n    Balance: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"loans/enum.MarketState.html\" title=\"enum loans::MarketState\">MarketState</a>"],["impl&lt;Balance&gt; MaxEncodedLen for <a class=\"struct\" href=\"loans/struct.Market.html\" title=\"struct loans::Market\">Market</a>&lt;Balance&gt;<span class=\"where fmt-newline\">where\n    Balance: MaxEncodedLen,</span>"]],
"oracle":[["impl&lt;Value, Moment&gt; MaxEncodedLen for <a class=\"struct\" href=\"oracle/struct.TimestampedValue.html\" title=\"struct oracle::TimestampedValue\">TimestampedValue</a>&lt;Value, Moment&gt;<span class=\"where fmt-newline\">where\n    Value: MaxEncodedLen,\n    Moment: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"oracle/types/enum.Version.html\" title=\"enum oracle::types::Version\">Version</a>"]],
"redeem":[["impl MaxEncodedLen for <a class=\"enum\" href=\"redeem/types/enum.Version.html\" title=\"enum redeem::types::Version\">Version</a>"]],
"replace":[["impl MaxEncodedLen for <a class=\"enum\" href=\"replace/types/enum.Version.html\" title=\"enum replace::types::Version\">Version</a>"]],
"vault_registry":[["impl MaxEncodedLen for <a class=\"enum\" href=\"vault_registry/enum.VaultStatus.html\" title=\"enum vault_registry::VaultStatus\">VaultStatus</a>"],["impl&lt;Balance, CurrencyId: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>&gt; MaxEncodedLen for <a class=\"struct\" href=\"vault_registry/struct.SystemVault.html\" title=\"struct vault_registry::SystemVault\">SystemVault</a>&lt;Balance, CurrencyId&gt;<span class=\"where fmt-newline\">where\n    Balance: MaxEncodedLen,\n    <a class=\"struct\" href=\"vault_registry/types/struct.VaultCurrencyPair.html\" title=\"struct vault_registry::types::VaultCurrencyPair\">VaultCurrencyPair</a>&lt;CurrencyId&gt;: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"vault_registry/types/enum.Version.html\" title=\"enum vault_registry::types::Version\">Version</a>"],["impl&lt;AccountId, BlockNumber, Balance, CurrencyId: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>, UnsignedFixedPoint&gt; MaxEncodedLen for <a class=\"struct\" href=\"vault_registry/struct.Vault.html\" title=\"struct vault_registry::Vault\">Vault</a>&lt;AccountId, BlockNumber, Balance, CurrencyId, UnsignedFixedPoint&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"vault_registry/struct.VaultId.html\" title=\"struct vault_registry::VaultId\">VaultId</a>&lt;AccountId, CurrencyId&gt;: MaxEncodedLen,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;BlockNumber&gt;: MaxEncodedLen,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;UnsignedFixedPoint&gt;: MaxEncodedLen,\n    Balance: MaxEncodedLen,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()