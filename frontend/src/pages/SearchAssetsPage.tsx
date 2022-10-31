import React from 'react';
import { useParams } from 'react-router-dom';
import styled from 'styled-components';
import useAxios from 'axios-hooks';
import Breadcrumb from 'rsuite/Breadcrumb';
import Table from 'rsuite/Table';
import Panel from 'rsuite/Panel';

import Grid from 'rsuite/Grid';
import Row from 'rsuite/Row';
import Col from 'rsuite/Col';

import CountryCell from '../components/table/CountryCell';
import TickerCell from '../components/table/TickerCell';
import SearchAssetInput from '../components/SearchAssetInput';
import useWindowTitle from '../components/hooks/useWindowTitle';

const MarginRow = styled(Row)`
  margin-bottom: 25px;
`;

const BodyRow = styled(Row)`

`;

export default function SearchAssetsPage() {
  useWindowTitle('Watch Asset');

  const params = useParams()
  const [{ data, loading, error }] = useAxios(`/api/assets/search?query=${params?.query}`);

  return (
    <Grid fluid>
      <MarginRow>
        <Col xs={18}>
          <Breadcrumb>
            <Breadcrumb.Item href="/assets">Assets</Breadcrumb.Item>
            <Breadcrumb.Item active>Search</Breadcrumb.Item>
          </Breadcrumb>
        </Col>
        <Col xs={6}>
          <SearchAssetInput />
        </Col>
      </MarginRow>
      <BodyRow>
        <Col xs={24}>
          <Panel header={`Results: ${data?.results?.length || 0}`} bordered bodyFill>
            <Table
              autoHeight
              loading={loading}
              data={data?.results}>
              <Table.Column width={70} fixed>
                <Table.HeaderCell>Country</Table.HeaderCell>
                <CountryCell dataKey="country" />
              </Table.Column>
              <Table.Column width={220} fixed>
                <Table.HeaderCell>Ticker</Table.HeaderCell>
                <TickerCell dataKey="ticker" />
              </Table.Column>
              <Table.Column fixed flexGrow={1}>
                <Table.HeaderCell>Name</Table.HeaderCell>
                <Table.Cell dataKey="description" />
              </Table.Column>
            </Table>
          </Panel>
        </Col>
      </BodyRow>
    </Grid>
  )
}
