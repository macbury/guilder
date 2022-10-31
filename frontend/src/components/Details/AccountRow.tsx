import React from 'react'
import { useGetAccountQuery } from '../../store/api';
import { Row } from './Row';

export interface IAccountRowProps {
  name: string,
  accountId: number
}

export function AccountRow({ name, accountId, ...props } : IAccountRowProps) {
  const { data: account } = useGetAccountQuery(accountId);

  return (
    <Row name={name} {...props}>
      {account?.name || '-'}
    </Row>
  )
}
