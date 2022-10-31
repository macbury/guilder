import React from 'react';
import { Link } from 'react-router-dom';
import styled from 'styled-components';
import Table from 'rsuite/Table';
import Avatar from 'rsuite/Avatar';
import AvatarGroup from 'rsuite/AvatarGroup';

const Container = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  height: 46px;
  padding-left: 10px;
`

const Name = styled.div`
  padding-left: 10px;
  display: inline-block;
`;

const Ticker = styled.small`
  padding-left: 5px;
  opacity: 0.5;
  color: #fff;
`

export default function AssetNameCell({ rowData, ...props } : any) {
  let src = rowData['secondaryLogoUrl'];
  let logo = rowData['logoUrl'];
  let name = rowData['name'];
  let ticker = rowData['id'];
  let to = `/assets/${ticker}`;
  return (
    <Table.Cell {...props} style={{ padding: 0 }}>
      <Container>
        <AvatarGroup stack>
          {src && <Avatar circle src={src} size="sm" />}
          {logo && <Avatar circle src={logo} size="sm" />}
        </AvatarGroup>
        <Link to={to}>
          <Name>{name}</Name>
          <Ticker>({ticker})</Ticker>
        </Link>
      </Container>
    </Table.Cell>
  )
}
