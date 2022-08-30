use crate::c_oracle_header::{
    AccountHeader,
    CPubkey,
    MappingAccount,
    PriceAccount,
    PriceComponent,
    PriceEma,
    PriceInfo,
    ProductAccount,
    PythAccount,
    PC_COMP_SIZE,
    PC_MAP_TABLE_SIZE,
    PC_VERSION,
    PRICE_ACCOUNT_SIZE,
};
use crate::deserialize::{
    initialize_pyth_account_checked,
    load_checked,
};
use crate::instruction::{
    AddPriceArgs,
    AddPublisherArgs,
    CommandHeader,
    DelPublisherArgs,
    InitPriceArgs,
    SetMinPubArgs,
    UpdPriceArgs,
};
use crate::tests::test_utils::AccountSetup;
use crate::time_machine_types::PriceAccountWrapper;
use crate::utils::try_convert;
use solana_program::pubkey::Pubkey;
use std::mem::{
    size_of,
    size_of_val,
};

#[test]
fn test_sizes() {
    assert_eq!(size_of::<CPubkey>(), 32);
    assert_eq!(
        size_of::<MappingAccount>(),
        24 + (PC_MAP_TABLE_SIZE as usize + 1) * size_of::<CPubkey>()
    );
    assert_eq!(size_of::<PriceInfo>(), 32);
    assert_eq!(
        size_of::<PriceComponent>(),
        size_of::<CPubkey>() + 2 * size_of::<PriceInfo>()
    );
    assert_eq!(
        size_of::<PriceAccount>(),
        48 + 8 * size_of::<u64>()
            + 3 * size_of::<CPubkey>()
            + size_of::<PriceInfo>()
            + (PC_COMP_SIZE as usize) * size_of::<PriceComponent>()
    );
    assert_eq!(size_of::<CommandHeader>(), 8);
    assert_eq!(size_of::<AddPriceArgs>(), 16);
    assert_eq!(size_of::<InitPriceArgs>(), 16);
    assert_eq!(size_of::<SetMinPubArgs>(), 12);
    assert_eq!(size_of::<AddPublisherArgs>(), 40);
    assert_eq!(size_of::<DelPublisherArgs>(), 40);
    assert_eq!(size_of::<UpdPriceArgs>(), 40);
    assert_eq!(size_of::<CPubkey>(), 32);
    assert_eq!(size_of::<AccountHeader>(), 16);
    assert_eq!(size_of::<MappingAccount>(), 20536);
    assert_eq!(size_of::<ProductAccount>(), 48);
    assert_eq!(size_of::<PriceComponent>(), 96);
    assert_eq!(size_of::<PriceEma>(), 24);
    assert_eq!(size_of::<PriceAccount>(), 3312);
    assert_eq!(
        size_of::<PriceAccountWrapper>(),
        try_convert::<_, usize>(PRICE_ACCOUNT_SIZE).unwrap()
    );
}

#[test]
fn test_offsets() {
    let program_id = Pubkey::new_unique();

    let mut price_setup = AccountSetup::new::<PriceAccount>(&program_id);
    let price_account = price_setup.to_account_info();

    initialize_pyth_account_checked::<PriceAccount>(&price_account, PC_VERSION).unwrap();
    let price_data = load_checked::<PriceAccount>(&price_account, PC_VERSION).unwrap();

    assert_eq!(
        size_of::<PriceAccount>() - size_of_val(&price_data.comp_),
        try_convert::<_, usize>(PriceAccount::INITIAL_SIZE).unwrap()
    );

    let mut mapping_setup = AccountSetup::new::<MappingAccount>(&program_id);
    let mapping_account = mapping_setup.to_account_info();

    initialize_pyth_account_checked::<MappingAccount>(&mapping_account, PC_VERSION).unwrap();
    let mapping_data = load_checked::<MappingAccount>(&mapping_account, PC_VERSION).unwrap();

    assert_eq!(
        size_of::<MappingAccount>() - size_of_val(&mapping_data.products_list),
        try_convert::<_, usize>(MappingAccount::INITIAL_SIZE).unwrap()
    );
}