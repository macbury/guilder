import React from 'react'
import { useGetWalletQuery } from '../../store/api';
import { Row } from './Row';

export interface IWalletRowProps {
  name: string,
  walletId: number
}

export function WalletRow({ name, walletId, ...props } : IWalletRowProps) {
  const { data: wallet } = useGetWalletQuery(walletId);

  return (
    <Row name={name} {...props}>
      {wallet?.name || '-'}
    </Row>
  )
}
