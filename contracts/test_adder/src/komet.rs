#[allow(unused)]
use soroban_sdk::{Address, Bytes, BytesN, ConstructorArgs, Env, TryFromVal, TryIntoVal};

#[allow(dead_code)]
#[cfg(not(test))]
#[link(wasm_import_module = "env")]
extern "C" {
    fn kasmer_create_contract(addr_val: u64, hash_val: u64) -> u64;
    fn kasmer_set_ledger_timestamp(x: u64);
    fn kasmer_address_from_bytes(addr_val: u64, is_contract: u64) -> u64;
}

#[cfg(not(test))]
pub fn create_contract<T, T2, A>(env: &Env, salt: &T, hash: &T2, ctr_args: Option<A>) -> Address
where
    T: TryIntoVal<Env, BytesN<32>>,
    T2: TryIntoVal<Env, BytesN<32>>,
    A: ConstructorArgs,
{
    use soroban_sdk::{FromVal, Symbol, Val, Vec};

    let salt: BytesN<32> = salt.try_into_val(env).unwrap();
    let hash: BytesN<32> = hash.try_into_val(env).unwrap();

    let res =
        unsafe { kasmer_create_contract(salt.as_val().get_payload(), hash.as_val().get_payload()) };
    let addr = Address::from_val(env, &Val::from_payload(res));

    if let Some(ctr_args) = ctr_args {        
        let args_vec: Vec<Val> = ctr_args.into_val(env);
        env.invoke_contract::<()>(&addr, &Symbol::new(env, "__constructor"), args_vec);
    };

    addr
}

#[allow(dead_code)]
#[cfg(not(test))]
pub fn set_ledger_timestamp(env: &Env, x: u64) {
    use soroban_sdk::{FromVal, Val};

    unsafe {
        kasmer_set_ledger_timestamp(Val::from_val(env, &x).get_payload());
    }
}

#[allow(dead_code)]
#[cfg(not(test))]
pub fn address_from_bytes<T>(env: &Env, bs: &T, is_contract: bool) -> Address
where
    Bytes: TryFromVal<Env, T>,
{
    use soroban_sdk::{FromVal, Val};

    let bs: Bytes = Bytes::try_from_val(env, bs).unwrap();

    unsafe {
        let res = kasmer_address_from_bytes(
            Val::from_val(env, &bs).get_payload(),
            Val::from_val(env, &is_contract).get_payload(),
        );
        Address::from_val(env, &Val::from_payload(res))
    }
}

#[cfg(test)]
pub fn create_contract<T, T2, A>(env: &Env, addr: &T, hash: &T2, ctr_args: Option<A>) -> Address
where
    T: TryIntoVal<Env, BytesN<32>>,
    T2: TryIntoVal<Env, BytesN<32>>,
    A: ConstructorArgs,
{
    let addr: BytesN<32> = addr.try_into_val(env).unwrap();
    let hash: BytesN<32> = hash.try_into_val(env).unwrap();

    if let Some(ctr_args) = ctr_args {
        env.deployer()
            .with_current_contract(addr)
            .deploy_v2(hash, ctr_args)
    } else {
        env.deployer()
            .with_current_contract(addr)
            .deploy_v2(hash, ())
    }
}

#[allow(unused)]
#[cfg(test)]
pub fn set_ledger_timestamp(env: &Env, x: u64) {
    use soroban_sdk::testutils::Ledger;

    env.ledger().set_timestamp(x);
}

#[cfg(test)]
pub fn address_from_bytes<T>(env: &Env, _bs: &T, _is_contract: bool) -> Address
where
    Bytes: TryFromVal<Env, T>,
{
    use soroban_sdk::testutils::Address as _;

    Address::generate(env)
}

#[macro_export]
macro_rules! assume {
    ( $x:expr ) => {
        if !$x {
            return true;
        }
    };
}
