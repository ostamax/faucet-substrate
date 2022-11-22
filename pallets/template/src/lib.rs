#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, traits::Currency};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type LocalCurrency: Currency<Self::AccountId>;
	}

	// The pallet's runtime storage items.
	#[pallet::storage]
	#[pallet::getter(fn account_block_number_map)]
	pub(super) type AccountBlockNumMap<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::BlockNumber, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AccountMap { block: T::BlockNumber, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		EarlyRequest,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(0)]
		pub fn get_current_block_number(origin: OriginFor<T>, amount_of_tokens: u32) -> DispatchResult {
			// Get the block number from the FRAME System pallet.
			let block = <frame_system::Pallet<T>>::block_number();

			let who = ensure_signed(origin)?;

			if <AccountBlockNumMap<T>>::contains_key(&who) {
				let old = <AccountBlockNumMap<T>>::get(&who);
				if block - old >= 16u32.into() {
					<AccountBlockNumMap<T>>::insert(&who, block);
				}
				else {
					return Err(Error::<T>::EarlyRequest.into())
				}
			} else {
				<AccountBlockNumMap<T>>::insert(&who, block);
			}

			let imbalance =  T::LocalCurrency::issue(amount_of_tokens.into());

			let _ = T::LocalCurrency::resolve_into_existing(&who, imbalance);

			Self::deposit_event(Event::AccountMap { block: amount_of_tokens.into(), who});

			Ok(())

		}
	}
}
