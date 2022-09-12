tonic::include_proto!("concordium.v2");

use super::Require;
use crate::types;
use crypto_common::{Deserial, Versioned, VERSION_0};
use id::{
    constants::{ArCurve, AttributeKind},
    types::{
        AccountCredentialWithoutProofs, CredentialDeploymentValues,
        InitialCredentialDeploymentValues,
    },
};
use std::collections::BTreeMap;

fn consume<A: Deserial>(bytes: &[u8]) -> Result<A, tonic::Status> {
    let mut cursor = std::io::Cursor::new(bytes);
    let res = A::deserial(&mut cursor);
    match res {
        Ok(v) if cursor.position() == bytes.len() as u64 => Ok(v),
        _ => Err(tonic::Status::internal(
            "Unexpected response from the server.",
        )),
    }
}

impl TryFrom<AccountAddress> for super::AccountAddress {
    type Error = tonic::Status;

    fn try_from(value: AccountAddress) -> Result<Self, Self::Error> {
        match value.value.try_into() {
            Ok(addr) => Ok(Self(addr)),
            Err(_) => Err(tonic::Status::internal(
                "Unexpected account address format.",
            )),
        }
    }
}

impl TryFrom<Address> for super::types::Address {
    type Error = tonic::Status;

    fn try_from(value: Address) -> Result<Self, Self::Error> {
        match value.r#type.require_owned()? {
            address::Type::Account(acc) => Ok(Self::Account(acc.try_into()?)),
            address::Type::Contract(contr) => Ok(Self::Contract(contr.into())),
        }
    }
}

impl TryFrom<ModuleRef> for super::ModuleRef {
    type Error = tonic::Status;

    fn try_from(value: ModuleRef) -> Result<Self, Self::Error> {
        match value.value.try_into() {
            Ok(mod_ref) => Ok(Self::new(mod_ref)),
            Err(_) => Err(tonic::Status::internal(
                "Unexpected module reference format.",
            )),
        }
    }
}

impl From<ContractAddress> for super::ContractAddress {
    fn from(value: ContractAddress) -> Self {
        super::ContractAddress::new(value.index, value.subindex)
    }
}

impl TryFrom<VersionedModuleSource> for types::smart_contracts::WasmModule {
    type Error = tonic::Status;

    fn try_from(versioned_module: VersionedModuleSource) -> Result<Self, Self::Error> {
        let module = match versioned_module.module.require_owned()? {
            versioned_module_source::Module::V0(versioned_module_source::ModuleSourceV0 {
                value,
            }) => types::smart_contracts::WasmModule {
                version: types::smart_contracts::WasmVersion::V0,
                source:  value.into(),
            },
            versioned_module_source::Module::V1(versioned_module_source::ModuleSourceV1 {
                value,
            }) => types::smart_contracts::WasmModule {
                version: types::smart_contracts::WasmVersion::V1,
                source:  value.into(),
            },
        };
        Ok(module)
    }
}

impl From<Parameter> for super::types::smart_contracts::Parameter {
    fn from(value: Parameter) -> Self { value.value.into() }
}

impl TryFrom<InstanceInfo> for super::InstanceInfo {
    type Error = tonic::Status;

    fn try_from(value: InstanceInfo) -> Result<Self, Self::Error> {
        match value.version.require_owned()? {
            instance_info::Version::V0(v0) => Ok(Self::V0 {
                model:         v0.model.require_owned()?.value,
                owner:         v0.owner.require_owned()?.try_into()?,
                amount:        v0.amount.require_owned()?.into(),
                methods:       v0
                    .methods
                    .into_iter()
                    .map(TryFrom::try_from)
                    .collect::<Result<_, tonic::Status>>()?,
                name:          v0.name.require_owned()?.try_into()?,
                source_module: v0.source_module.require_owned()?.try_into()?,
            }),
            instance_info::Version::V1(v1) => Ok(Self::V1 {
                owner:         v1.owner.require_owned()?.try_into()?,
                amount:        v1.amount.require_owned()?.into(),
                methods:       v1
                    .methods
                    .into_iter()
                    .map(TryFrom::try_from)
                    .collect::<Result<_, tonic::Status>>()?,
                name:          v1.name.require_owned()?.try_into()?,
                source_module: v1.source_module.require_owned()?.try_into()?,
            }),
        }
    }
}

impl TryFrom<ReceiveName> for concordium_contracts_common::OwnedReceiveName {
    type Error = tonic::Status;

    fn try_from(value: ReceiveName) -> Result<Self, Self::Error> {
        match Self::new(value.value) {
            Ok(rn) => Ok(rn),
            Err(_) => Err(tonic::Status::internal("Unexpected receive name format.")),
        }
    }
}

impl TryFrom<InitName> for concordium_contracts_common::OwnedContractName {
    type Error = tonic::Status;

    fn try_from(value: InitName) -> Result<Self, Self::Error> {
        match Self::new(value.value) {
            Ok(cn) => Ok(cn),
            Err(_) => Err(tonic::Status::internal("Unexpected contract name format.")),
        }
    }
}

impl TryFrom<BlockHash> for super::BlockHash {
    type Error = tonic::Status;

    fn try_from(value: BlockHash) -> Result<Self, Self::Error> {
        match value.value.try_into() {
            Ok(hash) => Ok(Self::new(hash)),
            Err(_) => Err(tonic::Status::internal("Unexpected block hash format.")),
        }
    }
}

impl TryFrom<TransactionHash> for super::hashes::TransactionHash {
    type Error = tonic::Status;

    fn try_from(value: TransactionHash) -> Result<Self, Self::Error> {
        match value.value.try_into() {
            Ok(hash) => Ok(Self::new(hash)),
            Err(_) => Err(tonic::Status::internal("Unexpected block hash format.")),
        }
    }
}

impl From<AbsoluteBlockHeight> for super::AbsoluteBlockHeight {
    fn from(abh: AbsoluteBlockHeight) -> Self { Self { height: abh.value } }
}

impl From<BlockHeight> for super::types::BlockHeight {
    fn from(bh: BlockHeight) -> Self { Self { height: bh.value } }
}

impl From<SequenceNumber> for super::types::Nonce {
    fn from(n: SequenceNumber) -> Self { Self { nonce: n.value } }
}

impl From<Amount> for super::super::common::types::Amount {
    fn from(n: Amount) -> Self { Self { micro_ccd: n.value } }
}

impl From<AccountIndex> for super::types::AccountIndex {
    fn from(n: AccountIndex) -> Self { Self { index: n.value } }
}

impl From<super::types::AccountIndex> for AccountIndex {
    fn from(n: super::types::AccountIndex) -> Self { Self { value: n.index } }
}

impl From<BakerId> for super::types::BakerId {
    fn from(n: BakerId) -> Self { Self { id: n.value.into() } }
}

impl TryFrom<DelegationTarget> for super::types::DelegationTarget {
    type Error = tonic::Status;

    fn try_from(dt: DelegationTarget) -> Result<Self, Self::Error> {
        match dt.target.require_owned()? {
            delegation_target::Target::Passive(_) => Ok(Self::Passive),
            delegation_target::Target::Baker(bid) => Ok(Self::Baker {
                baker_id: bid.into(),
            }),
        }
    }
}

impl TryFrom<EncryptionKey> for id::elgamal::PublicKey<ArCurve> {
    type Error = tonic::Status;

    fn try_from(value: EncryptionKey) -> Result<Self, Self::Error> { consume(&value.value) }
}

impl TryFrom<AccountThreshold> for super::types::AccountThreshold {
    type Error = tonic::Status;

    fn try_from(value: AccountThreshold) -> Result<Self, Self::Error> {
        if let Ok(Ok(v)) = u8::try_from(value.value).map(Self::try_from) {
            Ok(v)
        } else {
            Err(tonic::Status::internal("Unexpected account threshold."))
        }
    }
}

impl TryFrom<EncryptedAmount> for encrypted_transfers::types::EncryptedAmount<ArCurve> {
    type Error = tonic::Status;

    fn try_from(value: EncryptedAmount) -> Result<Self, Self::Error> { consume(&value.value) }
}

impl TryFrom<BakerElectionVerifyKey> for super::types::BakerElectionVerifyKey {
    type Error = tonic::Status;

    fn try_from(value: BakerElectionVerifyKey) -> Result<Self, Self::Error> {
        consume(&value.value)
    }
}

impl TryFrom<BakerSignatureVerifyKey> for super::types::BakerSignatureVerifyKey {
    type Error = tonic::Status;

    fn try_from(value: BakerSignatureVerifyKey) -> Result<Self, Self::Error> {
        consume(&value.value)
    }
}

impl TryFrom<BakerAggregationVerifyKey> for super::types::BakerAggregationVerifyKey {
    type Error = tonic::Status;

    fn try_from(value: BakerAggregationVerifyKey) -> Result<Self, Self::Error> {
        consume(&value.value)
    }
}

impl TryFrom<EncryptedBalance> for super::types::AccountEncryptedAmount {
    type Error = tonic::Status;

    fn try_from(value: EncryptedBalance) -> Result<Self, Self::Error> {
        let self_amount = value.self_amount.require_owned()?.try_into()?;
        let start_index = value.start_index;
        let aggregated_amount = match (value.aggregated_amount, value.num_aggregated) {
            (Some(aa), Some(si)) => Some((aa.try_into()?, si)),
            (None, None) => None,
            _ => {
                return Err(tonic::Status::internal(
                    "Unexpected response for 'EncryptedBalance'.",
                ))
            }
        };
        let incoming_amounts = value
            .incoming_amounts
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<Result<_, tonic::Status>>()?;
        Ok(Self {
            self_amount,
            start_index,
            aggregated_amount,
            incoming_amounts,
        })
    }
}

impl From<Timestamp> for chrono::DateTime<chrono::Utc> {
    fn from(value: Timestamp) -> Self {
        chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH)
            + chrono::Duration::milliseconds(value.value as i64)
    }
}

impl From<Duration> for chrono::Duration {
    fn from(value: Duration) -> Self { chrono::Duration::milliseconds(value.value as i64) }
}

impl From<Duration> for super::types::SlotDuration {
    fn from(value: Duration) -> Self {
        super::types::SlotDuration {
            millis: value.value,
        }
    }
}

impl From<GenesisIndex> for super::types::GenesisIndex {
    fn from(value: GenesisIndex) -> Self { value.value.into() }
}

impl From<ProtocolVersion> for super::types::ProtocolVersion {
    fn from(value: ProtocolVersion) -> Self {
        match value {
            ProtocolVersion::ProtocolVersion1 => super::types::ProtocolVersion::P1,
            ProtocolVersion::ProtocolVersion2 => super::types::ProtocolVersion::P2,
            ProtocolVersion::ProtocolVersion3 => super::types::ProtocolVersion::P3,
            ProtocolVersion::ProtocolVersion4 => super::types::ProtocolVersion::P4,
        }
    }
}

impl TryFrom<StakePendingChange> for super::types::StakePendingChange {
    type Error = tonic::Status;

    fn try_from(value: StakePendingChange) -> Result<Self, Self::Error> {
        match value.change.require_owned()? {
            stake_pending_change::Change::Reduce(rs) => {
                let new_stake = rs.new_stake.require_owned()?.into();

                Ok(Self::ReduceStake {
                    new_stake,
                    effective_time: rs.effective_time.require_owned()?.into(),
                })
            }
            stake_pending_change::Change::Remove(rs) => Ok(Self::RemoveStake {
                effective_time: rs.into(),
            }),
        }
    }
}

impl TryFrom<BakerInfo> for super::types::BakerInfo {
    type Error = tonic::Status;

    fn try_from(value: BakerInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            baker_id:                     value.baker_id.require_owned()?.into(),
            baker_election_verify_key:    value.election_key.require_owned()?.try_into()?,
            baker_signature_verify_key:   value.signature_key.require_owned()?.try_into()?,
            baker_aggregation_verify_key: value.aggregation_key.require_owned()?.try_into()?,
        })
    }
}

impl From<OpenStatus> for super::types::OpenStatus {
    fn from(os: OpenStatus) -> Self {
        match os {
            OpenStatus::OpenForAll => Self::OpenForAll,
            OpenStatus::ClosedForNew => Self::ClosedForNew,
            OpenStatus::ClosedForAll => Self::ClosedForAll,
        }
    }
}

impl From<AmountFraction> for super::types::AmountFraction {
    fn from(af: AmountFraction) -> Self {
        Self {
            parts_per_hundred_thousands: crate::types::PartsPerHundredThousands {
                parts: af.parts_per_hundred_thousand,
            },
        }
    }
}

impl TryFrom<CommissionRates> for super::types::CommissionRates {
    type Error = tonic::Status;

    fn try_from(value: CommissionRates) -> Result<Self, Self::Error> {
        Ok(Self {
            finalization: value.finalization.require_owned()?.into(),
            baking:       value.baking.require_owned()?.into(),
            transaction:  value.transaction.require_owned()?.into(),
        })
    }
}

impl TryFrom<BakerPoolInfo> for super::types::BakerPoolInfo {
    type Error = tonic::Status;

    fn try_from(value: BakerPoolInfo) -> Result<Self, Self::Error> {
        let open_status = value.open_status().into();
        let metadata_url = value
            .url
            .try_into()
            .map_err(|_| tonic::Status::internal("Unexpected metadata length."))?;
        let commission_rates = value.commission_rates.require_owned()?.try_into()?;
        Ok(Self {
            open_status,
            metadata_url,
            commission_rates,
        })
    }
}

impl TryFrom<AccountStakingInfo> for super::types::AccountStakingInfo {
    type Error = tonic::Status;

    fn try_from(value: AccountStakingInfo) -> Result<Self, Self::Error> {
        match value.staking_info.require_owned()? {
            account_staking_info::StakingInfo::Baker(bsi) => {
                let staked_amount = bsi.staked_amount.require_owned()?.into();
                let restake_earnings = bsi.restake_earnings;
                let baker_info = bsi.baker_info.require_owned()?.try_into()?;
                let pending_change = match bsi.pending_change {
                    None => None,
                    Some(pc) => Some(pc.try_into()?),
                };
                let pool_info = match bsi.pool_info {
                    None => None,
                    Some(bi) => Some(bi.try_into()?),
                };
                Ok(Self::Baker {
                    staked_amount,
                    restake_earnings,
                    baker_info: Box::new(baker_info),
                    pending_change,
                    pool_info,
                })
            }
            account_staking_info::StakingInfo::Delegator(dsi) => {
                let staked_amount = dsi.staked_amount.require_owned()?.into();
                let restake_earnings = dsi.restake_earnings;
                let delegation_target = dsi.target.require_owned()?.try_into()?;
                let pending_change = match dsi.pending_change {
                    None => None,
                    Some(pc) => Some(pc.try_into()?),
                };
                Ok(Self::Delegated {
                    staked_amount,
                    restake_earnings,
                    delegation_target,
                    pending_change,
                })
            }
        }
    }
}

impl TryFrom<Release> for super::types::Release {
    type Error = tonic::Status;

    fn try_from(value: Release) -> Result<Self, Self::Error> {
        Ok(Self {
            timestamp:    value.timestamp.require_owned()?.into(),
            amount:       value.amount.require_owned()?.into(),
            transactions: value
                .transactions
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, tonic::Status>>()?,
        })
    }
}

impl TryFrom<ReleaseSchedule> for super::types::AccountReleaseSchedule {
    type Error = tonic::Status;

    fn try_from(value: ReleaseSchedule) -> Result<Self, Self::Error> {
        Ok(Self {
            total:    value.total.require_owned()?.into(),
            schedule: value
                .schedules
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, tonic::Status>>()?,
        })
    }
}

impl TryFrom<AccountVerifyKey> for id::types::VerifyKey {
    type Error = tonic::Status;

    fn try_from(value: AccountVerifyKey) -> Result<Self, Self::Error> {
        match value.key.require_owned()? {
            account_verify_key::Key::Ed25519Key(v) => Ok(Self::Ed25519VerifyKey(consume(&v)?)),
        }
    }
}

impl TryFrom<SignatureThreshold> for id::types::SignatureThreshold {
    type Error = tonic::Status;

    fn try_from(value: SignatureThreshold) -> Result<Self, Self::Error> {
        if let Ok(v) = u8::try_from(value.value) {
            if v == 0 {
                Err(tonic::Status::internal(
                    "Unexpected zero signature threshold.",
                ))
            } else {
                Ok(Self(v))
            }
        } else {
            Err(tonic::Status::internal("Unexpected signature threshold."))
        }
    }
}

impl TryFrom<ArThreshold> for id::secret_sharing::Threshold {
    type Error = tonic::Status;

    fn try_from(value: ArThreshold) -> Result<Self, Self::Error> {
        if let Ok(v) = u8::try_from(value.value) {
            if v == 0 {
                Err(tonic::Status::internal("Unexpected zero AR threshold."))
            } else {
                Ok(Self(v))
            }
        } else {
            Err(tonic::Status::internal("Unexpected AR threshold."))
        }
    }
}

impl TryFrom<CredentialPublicKeys> for id::types::CredentialPublicKeys {
    type Error = tonic::Status;

    fn try_from(value: CredentialPublicKeys) -> Result<Self, Self::Error> {
        Ok(Self {
            keys:      value
                .keys
                .into_iter()
                .map(|(k, v)| {
                    if let Ok(k) = u8::try_from(k) {
                        let k = k.into();
                        let v = v.try_into()?;
                        Ok((k, v))
                    } else {
                        Err(tonic::Status::internal("Unexpected key index."))
                    }
                })
                .collect::<Result<_, tonic::Status>>()?,
            threshold: value.threshold.require_owned()?.try_into()?,
        })
    }
}

impl TryFrom<CredentialRegistrationId> for super::types::CredentialRegistrationID {
    type Error = tonic::Status;

    fn try_from(value: CredentialRegistrationId) -> Result<Self, Self::Error> {
        consume(&value.value)
    }
}

impl TryFrom<CredentialRegistrationId> for ArCurve {
    type Error = tonic::Status;

    fn try_from(value: CredentialRegistrationId) -> Result<Self, Self::Error> {
        consume(&value.value)
    }
}

impl From<IdentityProviderIdentity> for id::types::IpIdentity {
    fn from(v: IdentityProviderIdentity) -> Self { Self(v.value) }
}

impl TryFrom<YearMonth> for id::types::YearMonth {
    type Error = tonic::Status;

    fn try_from(value: YearMonth) -> Result<Self, Self::Error> {
        Ok(Self {
            year:  value
                .year
                .try_into()
                .map_err(|_| tonic::Status::internal("Unexpected year."))?,
            month: value
                .month
                .try_into()
                .map_err(|_| tonic::Status::internal("Unexpected year."))?,
        })
    }
}

impl TryFrom<Policy> for id::types::Policy<ArCurve, AttributeKind> {
    type Error = tonic::Status;

    fn try_from(value: Policy) -> Result<Self, Self::Error> {
        Ok(Self {
            valid_to:   value.valid_to.require_owned()?.try_into()?,
            created_at: value.created_at.require_owned()?.try_into()?,
            policy_vec: value
                .attributes
                .into_iter()
                .map(|(k, v)| {
                    let k = id::types::AttributeTag(
                        k.try_into()
                            .map_err(|_| tonic::Status::internal("Unexpected attribute tag."))?,
                    );
                    let v = consume(&v)?;
                    Ok((k, v))
                })
                .collect::<Result<_, tonic::Status>>()?,
            _phantom:   std::marker::PhantomData,
        })
    }
}

impl TryFrom<ChainArData> for id::types::ChainArData<ArCurve> {
    type Error = tonic::Status;

    fn try_from(value: ChainArData) -> Result<Self, Self::Error> {
        consume(&value.enc_id_cred_pub_share)
    }
}

impl TryFrom<Commitment> for id::pedersen_commitment::Commitment<ArCurve> {
    type Error = tonic::Status;

    fn try_from(value: Commitment) -> Result<Self, Self::Error> { consume(&value.value) }
}

impl TryFrom<CredentialCommitments> for id::types::CredentialDeploymentCommitments<ArCurve> {
    type Error = tonic::Status;

    fn try_from(value: CredentialCommitments) -> Result<Self, Self::Error> {
        Ok(Self {
            cmm_prf: value.prf.require_owned()?.try_into()?,
            cmm_cred_counter: value.cred_counter.require_owned()?.try_into()?,
            cmm_max_accounts: value.max_accounts.require_owned()?.try_into()?,
            cmm_attributes: value
                .attributes
                .into_iter()
                .map(|(k, v)| {
                    let k = id::types::AttributeTag(
                        k.try_into()
                            .map_err(|_| tonic::Status::internal("Unexpected attribute tag."))?,
                    );
                    let v = v.try_into()?;
                    Ok((k, v))
                })
                .collect::<Result<_, tonic::Status>>()?,
            cmm_id_cred_sec_sharing_coeff: value
                .id_cred_sec_sharing_coeff
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, tonic::Status>>()?,
        })
    }
}

impl TryFrom<AccountCredential> for AccountCredentialWithoutProofs<ArCurve, AttributeKind> {
    type Error = tonic::Status;

    fn try_from(value: AccountCredential) -> Result<Self, Self::Error> {
        match value.credential_values.require_owned()? {
            account_credential::CredentialValues::Initial(ic) => {
                let icdv = InitialCredentialDeploymentValues {
                    cred_account: ic.keys.require_owned()?.try_into()?,
                    reg_id:       ic.cred_id.require_owned()?.try_into()?,
                    ip_identity:  ic.ip_id.require_owned()?.into(),
                    policy:       ic.policy.require_owned()?.try_into()?,
                };
                Ok(Self::Initial { icdv })
            }
            account_credential::CredentialValues::Normal(nc) => {
                let cdv = CredentialDeploymentValues {
                    cred_key_info: nc.keys.require_owned()?.try_into()?,
                    cred_id:       nc.cred_id.require_owned()?.try_into()?,
                    ip_identity:   nc.ip_id.require_owned()?.into(),
                    threshold:     nc.ar_threshold.require_owned()?.try_into()?,
                    ar_data:       nc
                        .ar_data
                        .into_iter()
                        .map(|(k, v)| {
                            let k = k
                                .try_into()
                                .map_err(|_| tonic::Status::internal("Unexpected AR identity."))?;
                            let v = v.try_into()?;
                            Ok((k, v))
                        })
                        .collect::<Result<_, tonic::Status>>()?,
                    policy:        nc.policy.require_owned()?.try_into()?,
                };
                let commitments = nc.commitments.require_owned()?.try_into()?;
                Ok(Self::Normal { cdv, commitments })
            }
        }
    }
}

impl TryFrom<AccountInfo> for super::types::AccountInfo {
    type Error = tonic::Status;

    fn try_from(value: AccountInfo) -> Result<Self, Self::Error> {
        let AccountInfo {
            sequence_number,
            amount,
            schedule,
            creds,
            threshold,
            encrypted_balance,
            encryption_key,
            index,
            stake,
            address,
        } = value;
        let account_nonce = sequence_number.require_owned()?.into();
        let account_amount = amount.require_owned()?.into();
        let account_release_schedule = schedule.require_owned()?.try_into()?;
        let account_threshold = threshold.require_owned()?.try_into()?;
        let account_encrypted_amount = encrypted_balance.require_owned()?.try_into()?;
        let account_encryption_key = encryption_key.require_owned()?.try_into()?;
        let account_index = index.require_owned()?.into();
        let account_stake = match stake {
            Some(stake) => Some(stake.try_into()?),
            None => None,
        };
        let account_address = address.require_owned()?.try_into()?;
        Ok(Self {
            account_nonce,
            account_amount,
            account_release_schedule,
            account_credentials: creds
                .into_iter()
                .map(|(k, v)| {
                    let k = u8::try_from(k)
                        .map_err(|_| tonic::Status::internal("Unexpected credential index."))?
                        .into();
                    let v = v.try_into()?;
                    Ok((k, Versioned::new(VERSION_0, v)))
                })
                .collect::<Result<_, tonic::Status>>()?,
            account_threshold,
            account_encrypted_amount,
            account_encryption_key,
            account_index,
            account_stake,
            account_address,
        })
    }
}

impl TryFrom<TransactionStatus> for super::types::TransactionStatus {
    type Error = tonic::Status;

    fn try_from(value: TransactionStatus) -> Result<Self, Self::Error> {
        match value.status.require_owned()? {
            transaction_status::Status::Received(_) => {
                Ok(super::types::TransactionStatus::Received)
            }
            transaction_status::Status::Finalized(f) => {
                let mut summaries: BTreeMap<super::BlockHash, super::types::BlockItemSummary> =
                    BTreeMap::new();
                let o = f.outcome.require_owned()?;
                let k = o.block_hash.require_owned()?.try_into()?;
                let v = o.outcome.require_owned()?.try_into()?;
                summaries.insert(k, v);
                Ok(super::types::TransactionStatus::Finalized(summaries))
            }
            transaction_status::Status::Committed(_) => todo!(),
        }
    }
}

impl TryFrom<BlockItemSummary> for super::types::BlockItemSummary {
    type Error = tonic::Status;

    fn try_from(value: BlockItemSummary) -> Result<Self, Self::Error> {
        Ok(Self {
            index:       value.index.require_owned()?.into(),
            energy_cost: value.energy_cost.require_owned()?.into(),
            hash:        value.hash.require_owned()?.try_into()?,
            details:     match value.details.require_owned()? {
                block_item_summary::Details::AccountTransaction(acc_trx) => {
                    super::types::BlockItemSummaryDetails::AccountTransaction(
                        super::types::AccountTransactionDetails {
                            cost:    acc_trx.cost.require_owned()?.into(),
                            sender:  acc_trx.sender.require_owned()?.try_into()?,
                            effects: acc_trx.effects.require_owned()?.try_into()?,
                        },
                    )
                }
                _ => todo!(),
            },
        })
    }
}

impl TryFrom<AccountTransactionEffects> for super::types::AccountTransactionEffects {
    type Error = tonic::Status;

    fn try_from(value: AccountTransactionEffects) -> Result<Self, Self::Error> {
        match value.effect.require_owned()? {
            account_transaction_effects::Effect::None(n) => Ok(Self::None {
                transaction_type: {
                    match n.transaction_type {
                        None => None,
                        Some(tt) => Some(tt.try_into()?),
                    }
                },
                reject_reason:    n.reject_reason.require_owned()?.try_into()?,
            }),
            account_transaction_effects::Effect::AccountTransfer(at) => Ok(Self::AccountTransfer {
                amount: at.amount.require_owned()?.into(),
                to:     at.to.require_owned()?.try_into()?,
            }),
            _ => todo!(),
        }
    }
}

impl TryFrom<RejectReason> for super::types::RejectReason {
    type Error = tonic::Status;

    fn try_from(value: RejectReason) -> Result<Self, Self::Error> {
        Ok(match value.reason.require_owned()? {
            reject_reason::Reason::ModuleNotWf(_) => Self::ModuleNotWF,
            reject_reason::Reason::ModuleHashAlreadyExists(v) => Self::ModuleHashAlreadyExists {
                contents: v.contents.require_owned()?.try_into()?,
            },
            reject_reason::Reason::InvalidAccountReference(v) => Self::InvalidAccountReference {
                contents: v.contents.require_owned()?.try_into()?,
            },
            reject_reason::Reason::InvalidInitMethod(v) => Self::InvalidInitMethod {
                contents: (
                    v.module_ref.require_owned()?.try_into()?,
                    v.init_name.require_owned()?.try_into()?,
                ),
            },
            reject_reason::Reason::InvalidReceiveMethod(v) => Self::InvalidReceiveMethod {
                contents: (
                    v.module_ref.require_owned()?.try_into()?,
                    v.receive_name.require_owned()?.try_into()?,
                ),
            },
            reject_reason::Reason::InvalidModuleReference(v) => Self::InvalidModuleReference {
                contents: v.contents.require_owned()?.try_into()?,
            },
            reject_reason::Reason::InvalidContractAddress(v) => Self::InvalidContractAddress {
                contents: v.contents.require_owned()?.into(),
            },
            reject_reason::Reason::RuntimeFailure(_) => Self::RuntimeFailure,
            reject_reason::Reason::AmountTooLarge(v) => Self::AmountTooLarge {
                contents: (
                    v.address.require_owned()?.try_into()?,
                    v.amount.require_owned()?.into(),
                ),
            },
            reject_reason::Reason::SerializationFailure(_) => Self::SerializationFailure,
            reject_reason::Reason::OutOfEnergy(_) => Self::OutOfEnergy,
            reject_reason::Reason::RejectedInit(v) => Self::RejectedInit {
                reject_reason: v.reject_reason,
            },
            reject_reason::Reason::RejectedReceive(v) => Self::RejectedReceive {
                reject_reason:    v.reject_reason,
                contract_address: v.contract_address.require_owned()?.into(),
                receive_name:     v.receive_name.require_owned()?.try_into()?,
                parameter:        v.parameter.require_owned()?.into(),
            },
            reject_reason::Reason::InvalidProof(_) => Self::InvalidProof,
            reject_reason::Reason::AlreadyABaker(v) => Self::AlreadyABaker {
                contents: v.contents.require_owned()?.into(),
            },
            reject_reason::Reason::NotABaker(v) => Self::NotABaker {
                contents: v.contents.require_owned()?.try_into()?,
            },
            reject_reason::Reason::InsufficientBalanceForBakerStake(_) => {
                Self::InsufficientBalanceForBakerStake
            }
            reject_reason::Reason::StakeUnderMinimumThresholdForBaking(_) => {
                Self::StakeUnderMinimumThresholdForBaking
            }
            reject_reason::Reason::BakerInCooldown(_) => Self::BakerInCooldown,
            reject_reason::Reason::DuplicateAggregationKey(v) => Self::DuplicateAggregationKey {
                contents: Box::new(v.contents.require_owned()?.try_into()?),
            },
            reject_reason::Reason::NonExistentCredentialId(_) => Self::NonExistentCredentialID,
            reject_reason::Reason::KeyIndexAlreadyInUse(_) => Self::KeyIndexAlreadyInUse,
            reject_reason::Reason::InvalidAccountThreshold(_) => Self::InvalidAccountThreshold,
            reject_reason::Reason::InvalidCredentialKeySignThreshold(_) => {
                Self::InvalidCredentialKeySignThreshold
            }
            reject_reason::Reason::InvalidEncryptedAmountTransferProof(_) => {
                Self::InvalidEncryptedAmountTransferProof
            }
            reject_reason::Reason::InvalidTransferToPublicProof(_) => {
                Self::InvalidTransferToPublicProof
            }
            reject_reason::Reason::EncryptedAmountSelfTransfer(v) => {
                Self::EncryptedAmountSelfTransfer {
                    contents: v.contents.require_owned()?.try_into()?,
                }
            }
            reject_reason::Reason::InvalidIndexOnEncryptedTransfer(_) => {
                Self::InvalidIndexOnEncryptedTransfer
            }
            reject_reason::Reason::ZeroScheduledAmount(_) => Self::ZeroScheduledAmount,
            reject_reason::Reason::NonIncreasingSchedule(_) => Self::NonIncreasingSchedule,
            reject_reason::Reason::FirstScheduledReleaseExpired(_) => {
                Self::FirstScheduledReleaseExpired
            }
            reject_reason::Reason::ScheduledSelfTransfer(v) => Self::ScheduledSelfTransfer {
                contents: v.contents.require_owned()?.try_into()?,
            },
            reject_reason::Reason::InvalidCredentials(_) => Self::InvalidCredentials,
            reject_reason::Reason::DuplicateCredIds(v) => Self::DuplicateCredIDs {
                contents: v
                    .contents
                    .into_iter()
                    .map(TryFrom::try_from)
                    .collect::<Result<_, tonic::Status>>()?,
            },
            reject_reason::Reason::NonExistentCredIds(v) => Self::NonExistentCredIDs {
                contents: v
                    .contents
                    .into_iter()
                    .map(TryFrom::try_from)
                    .collect::<Result<_, tonic::Status>>()?,
            },
            reject_reason::Reason::RemoveFirstCredential(_) => Self::RemoveFirstCredential,
            reject_reason::Reason::CredentialHolderDidNotSign(_) => {
                Self::CredentialHolderDidNotSign
            }
            reject_reason::Reason::NotAllowedMultipleCredentials(_) => {
                Self::NotAllowedMultipleCredentials
            }
            reject_reason::Reason::NotAllowedToReceiveEncrypted(_) => {
                Self::NotAllowedToReceiveEncrypted
            }
            reject_reason::Reason::NotAllowedToHandleEncrypted(_) => {
                Self::NotAllowedToHandleEncrypted
            }
            reject_reason::Reason::MissingBakerAddParameters(_) => Self::MissingBakerAddParameters,
            reject_reason::Reason::FinalizationRewardCommissionNotInRange(_) => {
                Self::FinalizationRewardCommissionNotInRange
            }
            reject_reason::Reason::BakingRewardCommissionNotInRange(_) => {
                Self::BakingRewardCommissionNotInRange
            }
            reject_reason::Reason::TransactionFeeCommissionNotInRange(_) => {
                Self::TransactionFeeCommissionNotInRange
            }
            reject_reason::Reason::AlreadyADelegator(_) => Self::AlreadyADelegator,
            reject_reason::Reason::InsufficientBalanceForDelegationStake(_) => {
                Self::InsufficientBalanceForDelegationStake
            }
            reject_reason::Reason::MissingDelegationAddParameters(_) => {
                Self::MissingDelegationAddParameters
            }
            reject_reason::Reason::InsufficientDelegationStake(_) => {
                Self::InsufficientDelegationStake
            }
            reject_reason::Reason::DelegatorInCooldown(_) => Self::DelegatorInCooldown,
            reject_reason::Reason::NotADelegator(v) => Self::NotADelegator {
                address: v.contents.require_owned()?.try_into()?,
            },
            reject_reason::Reason::DelegationTargetNotABaker(v) => {
                Self::DelegationTargetNotABaker {
                    target: v.contents.require_owned()?.into(),
                }
            }
            reject_reason::Reason::StakeOverMaximumThresholdForPool(_) => {
                Self::StakeOverMaximumThresholdForPool
            }
            reject_reason::Reason::PoolWouldBecomeOverDelegated(_) => {
                Self::PoolWouldBecomeOverDelegated
            }
            reject_reason::Reason::PoolClosed(_) => Self::PoolClosed,
        })
    }
}

impl TryFrom<NextAccountSequenceNumber> for types::queries::AccountNonceResponse {
    type Error = tonic::Status;

    fn try_from(value: NextAccountSequenceNumber) -> Result<Self, Self::Error> {
        Ok(Self {
            nonce:     value.sequence_number.require_owned()?.into(),
            all_final: value.all_final,
        })
    }
}

impl TryFrom<i32> for super::types::TransactionType {
    type Error = tonic::Status;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::DeployModule,
            1 => Self::InitContract,
            2 => Self::Update,
            3 => Self::Transfer,
            4 => Self::AddBaker,
            5 => Self::RemoveBaker,
            6 => Self::UpdateBakerStake,
            7 => Self::UpdateBakerRestakeEarnings,
            8 => Self::UpdateBakerKeys,
            9 => Self::UpdateCredentialKeys,
            10 => Self::EncryptedAmountTransfer,
            11 => Self::TransferToEncrypted,
            12 => Self::TransferToPublic,
            13 => Self::TransferWithSchedule,
            14 => Self::UpdateCredentials,
            15 => Self::RegisterData,
            16 => Self::TransferWithMemo,
            17 => Self::EncryptedAmountTransferWithMemo,
            18 => Self::TransferWithScheduleAndMemo,
            19 => Self::ConfigureBaker,
            20 => Self::ConfigureDelegation,
            n => {
                return Err(tonic::Status::invalid_argument(format!(
                    "{} is not a valid index for a TransactionType",
                    n
                )))
            }
        })
    }
}

impl From<block_item_summary::TransactionIndex> for super::types::TransactionIndex {
    fn from(value: block_item_summary::TransactionIndex) -> Self { Self { index: value.value } }
}

impl From<Energy> for super::types::Energy {
    fn from(value: Energy) -> Self {
        Self {
            energy: value.value,
        }
    }
}
impl TryFrom<ConsensusInfo> for types::queries::ConsensusInfo {
    type Error = tonic::Status;

    fn try_from(value: ConsensusInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            last_finalized_block_height:    value
                .last_finalized_block_height
                .require_owned()?
                .into(),
            block_arrive_latency_e_m_s_d:   value.block_arrive_latency_emsd,
            block_receive_latency_e_m_s_d:  value.block_receive_latency_emsd,
            last_finalized_block:           value
                .last_finalized_block
                .require_owned()?
                .try_into()?,
            block_receive_period_e_m_s_d:   value.block_receive_period_emsd,
            block_arrive_period_e_m_s_d:    value.block_arrive_period_emsd,
            blocks_received_count:          value.blocks_received_count.into(),
            transactions_per_block_e_m_s_d: value.transactions_per_block_emsd,
            finalization_period_e_m_a:      value.finalization_period_ema,
            best_block_height:              value.best_block_height.require_owned()?.into(),
            last_finalized_time:            value.last_finalized_time.map(|v| v.into()),
            finalization_count:             value.finalization_count.into(),
            epoch_duration:                 value.epoch_duration.require_owned()?.into(),
            blocks_verified_count:          value.blocks_verified_count.into(),
            slot_duration:                  value.slot_duration.require_owned()?.into(),
            genesis_time:                   value.genesis_time.require_owned()?.into(),
            finalization_period_e_m_s_d:    value.finalization_period_emsd,
            transactions_per_block_e_m_a:   value.transactions_per_block_ema,
            block_arrive_latency_e_m_a:     value.block_arrive_latency_ema,
            block_receive_latency_e_m_a:    value.block_receive_latency_ema,
            block_arrive_period_e_m_a:      value.block_arrive_period_ema,
            block_receive_period_e_m_a:     value.block_receive_period_ema,
            block_last_arrived_time:        value.block_last_arrived_time.map(|v| v.into()),
            best_block:                     value.best_block.require_owned()?.try_into()?,
            genesis_block:                  value.genesis_block.require_owned()?.try_into()?,
            block_last_received_time:       value.block_last_received_time.map(|v| v.into()),
            protocol_version:               ProtocolVersion::from_i32(value.protocol_version)
                .ok_or_else(|| tonic::Status::internal("Unknown protocol version"))?
                .into(),
            genesis_index:                  value.genesis_index.require_owned()?.into(),
            current_era_genesis_block:      value
                .current_era_genesis_block
                .require_owned()?
                .try_into()?,
            current_era_genesis_time:       value.current_era_genesis_time.require_owned()?.into(),
        })
    }
}
