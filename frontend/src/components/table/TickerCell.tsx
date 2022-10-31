import React from 'react';
import { Link } from 'react-router-dom';
import Table from 'rsuite/Table';

export default function TickerCell({ rowData, dataKey, ...props }) {
  let ticker = rowData[dataKey];
  let href = `/assets/${ticker}`;
  return (
    <Table.Cell {...props}>
      <Link to={href}>{ticker}</Link>
    </Table.Cell>
  )
}
