import React from 'react';
import Loader from 'rsuite/Loader';
import Table from 'rsuite/Table';

export interface ILoadingCellProps  {
  children: any
  loading: boolean
}

export default function LoadingCell({ loading, children, ...props } : ILoadingCellProps) {
  if (loading) {
    return (
      <Table.Cell {...props} align="center" verticalAlign="middle">
        <Loader />
      </Table.Cell>
    )
  }

  return (
    <Table.Cell {...props}>
      {children}
    </Table.Cell>
  )
}
