import React, { useEffect } from 'react';
import get from 'lodash/get';
import { useLazyGetWalletQuery } from '../../store/api';
import LoadingCell from './LoadingCell';

export default function WalletCell({ rowData, dataKey, ...props } : any) {
  const walletId = get(rowData, dataKey, null);
  const [fetch, { data: wallet, isLoading }] = useLazyGetWalletQuery();

  useEffect(() => {
    if (walletId) {
      fetch(walletId, true)
    }
  }, [walletId])

  return (
    <LoadingCell loading={isLoading} {...props}>
      {walletId ? wallet?.name : '-'}
    </LoadingCell>
  )
}
