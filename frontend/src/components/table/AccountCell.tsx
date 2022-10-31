import React, { useEffect } from 'react';
import get from 'lodash/get';
import LoadingCell from './LoadingCell';
import { useLazyGetAccountQuery } from '../../store/api';

export default function AccountCell({ rowData, dataKey, ...props } : any) {
  const accountId = get(rowData, dataKey, null);
  const [fetch, { data: account, isLoading }] = useLazyGetAccountQuery();

  useEffect(() => {
    if (accountId) {
      fetch(accountId, true)
    }
  }, [accountId])

  return (
    <LoadingCell loading={isLoading} {...props}>
      {accountId ? account?.name : '-'}
    </LoadingCell>
  )
}
