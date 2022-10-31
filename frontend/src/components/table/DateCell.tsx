import React, { useMemo } from 'react';
import dayjs from 'dayjs';
import Table from 'rsuite/Table';

export default function DateCell({ rowData, dataKey, ...props }) {
  const value = rowData[dataKey];
  const date = useMemo(() => {
    if (value) {
      return dayjs(value).format('DD MMMM YYYY HH:mm')
    } else {
      return "-"
    }
  }, [value])

  return (
    <Table.Cell {...props}>
      {date}
    </Table.Cell>
  )
}
