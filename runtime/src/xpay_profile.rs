use support::{StorageValue, StorageMap,StorageLinkedMap, dispatch::Result};
use super::xpay::Trait;
use super::xpay;
use rstd::vec::Vec;
pub fn save_profile<T:Trait>(acc_to_edit:T::AccountId,profile_name:Vec<u8>,image:Vec<u8>,ipfs:Vec<u8>)->Result{
  if <xpay::Profiles<T>>::exists(acc_to_edit.clone()){
    <xpay::Profiles<T>>::mutate(acc_to_edit,|q|{
    let fp=xpay::Profile{
      profile_name,image,ipfs
    };
    *q = Some(fp);
  });
  }else{
    let fp = xpay::Profile{
      profile_name,image,ipfs
      };
    <xpay::Profiles<T>>::insert(acc_to_edit, fp);
  }
  Ok(())
}
pub fn remove_profile<T:Trait>(acc_to_edit:T::AccountId)->Result{
  <xpay::Profiles<T>>::remove(acc_to_edit);
  Ok(())
}