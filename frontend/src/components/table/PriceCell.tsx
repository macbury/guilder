import React, { useMemo } from 'react';
import get from 'lodash/get';
import Table from 'rsuite/Table';
import useFormattedPrice from '../hooks/useFormattedPrice';

export default function PriceCell({ rowData, dataKey, ...props } : any) {
  const amount = get(rowData, dataKey, 0);
  const currency = rowData['currency'];
  const price = useFormattedPrice(amount, currency, false);

  return (
    <Table.Cell {...props}>
      {price}
    </Table.Cell>
  )
}
