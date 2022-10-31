import React from 'react';
import get from 'lodash/get';
import Table from 'rsuite/Table';
import styled from 'styled-components';
import ColoredAmount from '../ColoredAmount';
import useFormattedPercent from '../hooks/useFormattedPercent';

export interface IPercentChangeCellProps {
  rowData?: any,
  dataKey: string,
  change?: boolean
}

const Change = styled(ColoredAmount)`
  padding: 13px 10px;
`;

export default function PercentChangeCell({ rowData, change, dataKey, ...props } : IPercentChangeCellProps) {
  const amount = get(rowData, dataKey, 0);
  const percent = useFormattedPercent(amount, true);

  return (
    <Table.Cell style={{ padding: 0 }} {...props}>
      <Change amount={amount}>
        {!amount ? "-" : percent}
      </Change>
    </Table.Cell>
  )
}
