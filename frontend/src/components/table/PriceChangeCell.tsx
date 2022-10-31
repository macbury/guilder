import React, { useMemo } from 'react';
import get from 'lodash/get';
import Table from 'rsuite/Table';
import useFormattedPrice from '../hooks/useFormattedPrice';
import ColoredAmount from '../ColoredAmount';

export default function PriceChangeCell({ rowData, dataKey, ...props } : any) {
  const amount = get(rowData, dataKey, 0);
  const currency = rowData['currency'];
  const price = useFormattedPrice(amount, currency, true);

  return (
    <Table.Cell {...props}>
      <ColoredAmount amount={amount}>
        {price}
      </ColoredAmount>
    </Table.Cell>
  )
}
