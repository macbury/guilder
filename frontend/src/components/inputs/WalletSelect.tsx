import React from 'react';
import { useGetWalletsQuery } from '../../store/api';
import ObjectSelect, { IGenericObjectSelectProps } from './ObjectSelect';

export default function WalletSelect(props : IGenericObjectSelectProps) {
  const { data: wallets } = useGetWalletsQuery()

  return (
    <ObjectSelect
      data={wallets || []}
      {...props} />
  )
}
