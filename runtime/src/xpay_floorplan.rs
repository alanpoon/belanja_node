use support::{StorageValue, StorageMap,StorageLinkedMap, dispatch::Result};
use super::xpay::Trait;
use super::xpay;
use sr_primitives::traits::{CheckedAdd};
use rstd::vec::Vec;
pub fn add_floorplan<T:Trait>(origin: T::AccountId,item_id:T::ItemId,address:Vec<u8>,cubes:Vec<(u8,i16,i16,i16)>,desc:Vec<u8>,image:Vec<u8>,ipfs:Vec<u8>)->Result{
  let next_item_id = item_id.checked_add(&1.into()).ok_or_else(||"No new item id is available.")?;
	<xpay::FloorplanNextItemId<T>>::put(next_item_id);
  let fp = xpay::Floorplan{
    address,cubes,desc,image,ipfs};
	<xpay::Floorplans<T>>::insert(item_id.clone(), fp.clone());
  if <xpay::OwnerFloormapIds<T>>::exists(origin.clone()){
    <xpay::OwnerFloormapIds<T>>::mutate(origin.clone(), |q| {
      q.push(item_id);
    });
  }else{
    let mut v:Vec<T::ItemId> = Vec::new();
    v.push(item_id);
    <xpay::OwnerFloormapIds<T>>::insert(origin,v);
  }

  Ok(())
}
pub fn remove_floorplan<T:Trait>(origin: T::AccountId,item_id:T::ItemId)->Result{
  <xpay::Floorplans<T>>::remove(item_id);
  if <xpay::OwnerFloormapIds<T>>::exists(origin.clone()){
    <xpay::OwnerFloormapIds<T>>::mutate(origin.clone(), |q| {
      *q = Vec::new();
    });
  }
  Ok(())
}

pub fn change_floorplan<T:Trait>(item_id:T::ItemId,address:Vec<u8>,cubes:Vec<(u8,i16,i16,i16)>,desc:Vec<u8>,image:Vec<u8>,ipfs:Vec<u8>)->Result{
  <xpay::Floorplans<T>>::mutate(item_id,|q|{
    let fp = xpay::Floorplan{address,cubes,desc,image,ipfs};
    *q = Some(fp);
  });
  Ok(())
}