import React from 'react';
import { Link } from 'react-router-dom';
import styled from 'styled-components';
import Table from 'rsuite/Table';
import Tooltip from 'rsuite/Tooltip';
import Whisper from 'rsuite/Whisper';

const Container = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  height: 46px;
  padding-left: 10px;
`

export default function BondsNameCell({ rowData, dataKey, ...props } : any) {
  let name = rowData['name'];
  let emission = rowData[dataKey];
  let ticker = rowData['id'];
  let to = `/bonds/${ticker}`;

  const tooltip = (
    <Tooltip>
      {name}
    </Tooltip>
  );

  return (
    <Table.Cell {...props} style={{ padding: 0 }}>
      <Container>
        <Whisper placement="rightEnd" trigger="hover" speaker={tooltip}>
          <Link to={to}>
            {emission}
          </Link>
        </Whisper>
      </Container>
    </Table.Cell>
  )
}
