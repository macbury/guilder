import React, { useMemo } from 'react';
import get from 'lodash/get';
import Table from 'rsuite/Table';
import Tooltip from 'rsuite/Tooltip';
import Whisper from 'rsuite/Whisper';

export interface IDaysLeftCellProps {
  rowData?: any,
  dataKey: string,
  tooltipKey: string
}

export default function DaysLeftCell({ rowData, tooltipKey, dataKey, ...props } : IDaysLeftCellProps) {
  const days = get(rowData, dataKey);

  const text = useMemo(() => {
    let years = Math.floor(days / 365);
    let months = Math.floor(days / 30);

    if (years > 0) {
      return `${years} years`
    } else if (months > 0) {
      return `${months} months`
    } else {
      return `${days} days`
    }
  }, [days])

  const tooltip = (
    <Tooltip>
      {get(rowData, tooltipKey)}
    </Tooltip>
  );

  return (
    <Table.Cell {...props}>
      <Whisper placement="bottom" trigger="hover" speaker={tooltip}>
        <span>{text}</span>
      </Whisper>
    </Table.Cell>
  )
}
