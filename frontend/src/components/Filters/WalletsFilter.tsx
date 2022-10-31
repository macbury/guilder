import React from 'react';
import { useGetWalletsQuery } from '../../store/api';
import GenericCheckboxFilter from './GenericCheckboxFilter';

export interface IWalletsFilterProps {
  selectedWalletIds: number[],
  onTickWallet(selectedWalletIds: number[]) : void
}

export default function WalletsFilter({ selectedWalletIds, onTickWallet  } : IWalletsFilterProps) {
  const { data: wallets, isLoading } = useGetWalletsQuery();

  return (
    <GenericCheckboxFilter
      header="Wallet"
      items={wallets}
      loading={isLoading}
      onTickItem={onTickWallet}
      selectedItemsIds={selectedWalletIds}
    />
  )
}
