//use support::{decl_storage, decl_module, decl_event, StorageMap, StorageValue, dispatch::Result};
// use support::{decl_storage, decl_module, StorageValue, StorageMap,
// 	dispatch::Result, ensure, decl_event};
use support::{decl_module, decl_storage, decl_event, ensure, StorageValue, StorageMap, dispatch::Result};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
use parity_codec::{Encode, Decode};



// This struct mirrors an object that is populated by our WebRTC infrastructure with the necessary transaction engine payment audit capabilities (royalty, advertsing, subscription, crowdfunding etc)
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct WebRTCFundableItem<Hash, Balance> {
	id: Hash, 
	item_id: u64, // external world identifier
	metadata_ipfs_hash: Hash, // UI metadata related to the video information
    statistic_audit_ipfs_hash: Hash, // stats related to video audit
	advert_audit_ipfs_hash: Hash, // programmatic RTB / PAP advertising audit
	royality_pool_balance: Balance, // royalties attributed to the item
	advert_profit_pool_balance: Balance, // programmatic/ad payments
	fiat_backing_pool_balance: Balance, // backing crowdfund or purchases
	currency_code: u32, // external world enum
	// 
	//collective <<< TODO: Collective controls royality_pool_balance / advert_profit_pool_balance / fiat_backing_pool_balance  
	// 

}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Seed<Hash, Balance> {
	id: Hash,
	metadata_ipfs_hash: Hash, // front end UI animation instructions
	backer_payment_amount_balance: Balance, //projects can be backed this reprents those contributions
	fees_amount_balance: Balance, // visa stripe etc charge fees these fees are accounted here
	transaction_processor: u32, // enum stripe visa , paypal, apple store etc
	currency_code: u32, // external world enum
	item_id: u64, // external world identifier
    gen: u64, //version 
}

pub trait Trait: balances::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash
    {
		Created(AccountId, Hash),
		
		// TODO
		//ItemMetaDataAdded(AccountId, Hash, Balance),
		//ItemStatisticsAdded(AccountId, Hash, Balance),
		//ItemAdvertAuditAdded(AccountId, Hash, Balance),
		//ItemFunded(AccountId, Hash, Balance),
		//SeedFunded(AccountId, Hash, Balance),

		// Find events from collective https://docs.rs/pallet-collective/2.0.0-alpha.8/pallet_collective/ ? /ItemProposalAdded(AccountId, Hash, Balance),

		// WithdrawItemFunds(AccountId, Hash, Balance),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as LivetreeStorage {

		SeedTokens get(seed_tokens): map T::Hash => Seed<T::Hash, T::Balance>;
        SeedOwner get(owner_of): map T::Hash => Option<T::AccountId>;
		// <SeedTokens<T>>::insert(seed_identifier_hash, new_seed);
		// <SeedOwner<T>>::insert(seed_identifier_hash, &sender);

		AllSeedArray get(seed_by_index): map u64 => T::Hash;
        AllSeedCount get(all_seed_count): u64;
		AllSeedIndex: map T::Hash => u64;
		// <AllSeedArray<T>>::insert(all_seed_count, seed_identifier_hash);
		// <AllSeedCount<T>>::put(new_all_seed_count);
		// <AllSeedIndex<T>>::insert(seed_identifier_hash, all_seed_count);

        OwnedSeedArray get(seed_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
        OwnedSeedCount get(owned_seed_count): map T::AccountId => u64;
		OwnedSeedIndex: map T::Hash => u64;
		// <OwnedSeedArray<T>>::insert((sender.clone(), owned_seed_count), seed_identifier_hash);
		// <OwnedSeedCount<T>>::insert(&sender, new_owned_seed_count);
		// <OwnedSeedIndex<T>>::insert(seed_identifier_hash, owned_seed_count);



		Nonce: u64;


	
		ExtrinsicsTestBool get(my_bool_getter): bool;


	}
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
	   
		fn deposit_event<T>() = default;// add a function which deposits those events

		fn create_seed(origin, currency_code:u32, transaction_processor:u32 , item_id:u64 ) -> Result {
			
			let sender = ensure_signed(origin)?; // signed: public signed / root: governance / inherent: only block authors and validators
			



			// generate unique identifier
			let nonce = <Nonce<T>>::get(); // get nonce

            let seed_identifier_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
                				.using_encoded(<T as system::Trait>::Hashing::hash); // generate random hash not that the random_seed may return bugger all for hte first 80 blocks

            ensure!(!<SeedOwner<T>>::exists(seed_identifier_hash), "Seed token hash already exists, no new seed have been created bugging out."); // ensure we haven't already got this 

            let new_seed = Seed {
				id: seed_identifier_hash,
				metadata_ipfs_hash: seed_identifier_hash,
				backer_payment_amount_balance: <T::Balance as As<u64>>::sa(0), //how much the person paid to back the project
				fees_amount_balance: <T::Balance as As<u64>>::sa(0),
				transaction_processor: transaction_processor,
				currency_code: currency_code,
				item_id: item_id,
				gen: 0,
			};
			
			Self::mint(sender, seed_identifier_hash, new_seed)?;

			<Nonce<T>>::mutate(|n| *n += 1);
		

            Ok(())
		}
		
		fn contribute_seed_to_item(origin, currency_code:u32, transaction_processor:u32 ) -> Result {

            Ok(())
        }

		
		fn testextrinsics_function(origin, input_bool: bool) -> Result {
			// Public calls that are signed by an external account.
			// Root calls that are allowed to be made only by the governance system.
			// Inherent calls that are allowed to be made only by the block authors and validators.
			
			
			let _sender = ensure_signed(origin)?;

            <ExtrinsicsTestBool<T>>::put(input_bool);

            Ok(())
		}
		


	}
		
}

impl<T: Trait> Module<T> {
	fn mint(to: T::AccountId, seed_identifier_hash: T::Hash, new_seed: Seed<T::Hash, T::Balance>) -> Result {

		ensure!(!<SeedOwner<T>>::exists(seed_identifier_hash), "seed already exists");

		let all_seed_count = Self::all_seed_count();
		let owned_seed_count = Self::owned_seed_count(&to);

		let new_owned_seed_count = owned_seed_count.checked_add(1)
			.ok_or("Overflow error no seed has been added to the account because the account addition would result in an overflow error")?;

		let new_all_seed_count = all_seed_count.checked_add(1)
			.ok_or("Overflow error while adding 1 to the total seed count. Bugging out no new seed tokens were created.")?;


			<SeedTokens<T>>::insert(seed_identifier_hash, new_seed);
			<SeedOwner<T>>::insert(seed_identifier_hash, &to);
			
            <AllSeedArray<T>>::insert(all_seed_count, seed_identifier_hash);
            <AllSeedCount<T>>::put(new_all_seed_count);
            <AllSeedIndex<T>>::insert(seed_identifier_hash, all_seed_count);

			<OwnedSeedArray<T>>::insert((to.clone(), owned_seed_count), seed_identifier_hash);
            <OwnedSeedCount<T>>::insert(&to, new_owned_seed_count);
            <OwnedSeedIndex<T>>::insert(seed_identifier_hash, owned_seed_count);

		Self::deposit_event(RawEvent::Created(to, seed_identifier_hash));

		Ok(())
	}
}