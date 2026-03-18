#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        dispatch::DispatchResult,
    };
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Simple storage example (placeholder for future state handling)
    #[pallet::storage]
    #[pallet::getter(fn stored_value)]
    pub type StoredValue<T> = StorageValue<_, u32, ValueQuery>;

    /// Events for this pallet
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Value was set
        ValueStored(u32, T::AccountId),
    }

    /// Errors
    #[pallet::error]
    pub enum Error<T> {
        /// Example error
        NoneValue,
    }

    /// Dispatchable functions
    #[pallet::call]
    impl<T: Config> Pallet<T> {

        /// Store a value on-chain
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn set_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            StoredValue::<T>::put(value);

            Self::deposit_event(Event::ValueStored(value, sender));
            Ok(())
        }
    }
}
