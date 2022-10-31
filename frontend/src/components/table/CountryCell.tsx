import React from 'react';
import styled from 'styled-components';
import Table from 'rsuite/Table';
import Avatar from 'rsuite/Avatar';
import AvatarGroup from 'rsuite/AvatarGroup';

const Container = styled.div`
  margin: 0 auto;
  padding: 10px;
  height: 80px;
`

export default function CountryCell({ rowData, dataKey, ...props }) {
  let src = rowData[dataKey];
  let logo = rowData['logo'];
  return (
    <Table.Cell {...props} style={{ padding: 0 }}>
      <Container>
        <AvatarGroup stack>
          {src && <Avatar circle src={src} size="sm" />}
          {logo && <Avatar circle src={logo} size="sm" />}
        </AvatarGroup>
      </Container>
    </Table.Cell>
  )
}
