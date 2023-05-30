

// Create multiple accounts at once
let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

// Create individual accounts
let alice = AccountId::from([0x01; 32]);
let bob = AccountId::from([0x02; 32]);