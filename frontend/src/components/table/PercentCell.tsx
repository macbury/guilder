import React from 'react';
import get from 'lodash/get';
import Table from 'rsuite/Table';
import useFormattedPercent from '../hooks/useFormattedPercent';

export interface IPercentChangeCellProps {
  rowData?: any,
  dataKey: string,
  change?: boolean,
}

export default function PercentCell({ rowData, change, dataKey, ...props } : IPercentChangeCellProps) {
  const amount = get(rowData, dataKey, 0);
  const percent = useFormattedPercent(amount, true);

  return (
    <Table.Cell {...props}>
      {amount == null ? "-" : percent}
    </Table.Cell>
  )
}
