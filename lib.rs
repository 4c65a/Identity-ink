#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod identity {


    #[ink(storage)]
    pub struct Identity {
   
        value: bool,
    }

    impl Identity {
 
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

   
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }


        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }


    #[cfg(test)]
    mod tests {
  
        use super::*;

        #[ink::test]
        fn default_works() {
            let identity = Identity::default();
            assert_eq!(identity.get(), false);
        }


        #[ink::test]
        fn it_works() {
            let mut identity = Identity::new(false);
            assert_eq!(identity.get(), false);
            identity.flip();
            assert_eq!(identity.get(), true);
        }
    }


    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
   
        use super::*;


        use ink_e2e::build_message;

 
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

      
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
           
            let constructor = IdentityRef::default();

   
            let contract_account_id = client
                .instantiate("identity", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<IdentityRef>(contract_account_id.clone())
                .call(|identity| identity.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

     
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = IdentityRef::new(false);
            let contract_account_id = client
                .instantiate("identity", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<IdentityRef>(contract_account_id.clone())
                .call(|identity| identity.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));


            let flip = build_message::<IdentityRef>(contract_account_id.clone())
                .call(|identity| identity.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

    
            let get = build_message::<IdentityRef>(contract_account_id.clone())
                .call(|identity| identity.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
