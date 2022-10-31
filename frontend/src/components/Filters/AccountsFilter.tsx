import React from 'react';
import { useGetAccountsQuery } from '../../store/api';
import { useAccountsState } from '../../store/hooks/accounts';
import GenericCheckboxFilter from './GenericCheckboxFilter';

export interface IAccountsFilterProps {
  selectedAccountsIds: number[],
  onTickAccount(selectedAccountsIds: number[]) : void
}

export default function AccountsFilter({ selectedAccountsIds, onTickAccount  } : IAccountsFilterProps) {
  const { data: accounts, isLoading } = useGetAccountsQuery();

  const {
    actions: { newAccount }
  } = useAccountsState();

  return (
    <GenericCheckboxFilter
      header="Accounts"
      items={accounts}
      loading={isLoading}
      newItem={newAccount}
      onTickItem={onTickAccount}
      selectedItemsIds={selectedAccountsIds}
    />
  )
}
