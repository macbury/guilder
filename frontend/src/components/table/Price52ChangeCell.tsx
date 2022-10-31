import React, { useMemo } from 'react';
import get from 'lodash/get';
import Table from 'rsuite/Table';
import Tooltip from 'rsuite/Tooltip';
import Whisper from 'rsuite/Whisper';
import styled from 'styled-components';
import useFormattedPrice from '../hooks/useFormattedPrice';

export interface IPrice52ChangeCellProps {
  rowData?: any,
  dataKey: string,
}

const CandleContainer = styled.div`
  margin-top: 5px;
  height: 10px;
  overflow: hidden;
  background: var(--rs-progress-bg);
  border-radius: 5px;
`;

const Candle = styled.div`
  height: 10px;
  border-right: 3px solid var(--rs-progress-bar-fail);
`;

export default function Price52ChangeCell({ rowData, dataKey, ...props } : IPrice52ChangeCellProps) {
  const percent = get(rowData, dataKey, 0);
  const currency = rowData['currency'];
  const lowValue = useFormattedPrice(get(rowData, "performance.lowValue", 0), currency, true);
  const highValue = useFormattedPrice(get(rowData, "performance.highValue", 0), currency, true);

  const tooltip = (
    <Tooltip>
      Low: {lowValue}<br/>
      High: {highValue}
    </Tooltip>
  );

  return (
    <Table.Cell {...props}>
      <Whisper placement="top" controlId="control-id-hover" trigger="hover" speaker={tooltip}>
        <CandleContainer>
          <Candle style={{ width: `${percent}%` }} />
        </CandleContainer>
      </Whisper>
    </Table.Cell>
  )
}
