import React from 'react';
import { useGetAccountsQuery } from '../../store/api';
import { useAccountsState } from '../../store/hooks/accounts';
import ObjectSelect, { IGenericObjectSelectProps } from './ObjectSelect';

export default function AccountSelect(props : IGenericObjectSelectProps) {
  const { data: accounts } = useGetAccountsQuery();

  const { actions: { newAccount } } = useAccountsState();

  return (
    <ObjectSelect
      data={accounts}
      newObject={newAccount}
      {...props} />
  )
}
